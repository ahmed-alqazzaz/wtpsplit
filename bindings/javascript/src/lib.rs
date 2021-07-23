mod tractjs_backend;
mod utils;

use js_sys::{Array, Promise};
use std::rc::Rc;
use tractjs_backend::TractJSBackend;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use nnsplit as core;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(inspectable)]
pub struct Split {
    text: String,
    parts: Vec<JsValue>,
}

#[wasm_bindgen]
impl Split {
    #[wasm_bindgen(getter)]
    pub fn parts(&self) -> Vec<JsValue> {
        self.parts.to_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

impl<'a> From<core::Split<'a>> for Split {
    fn from(split: core::Split) -> Self {
        match split {
            core::Split::Text(_) => panic!("text can not be converted to a Split"),
            core::Split::Split((text, split_parts)) => {
                let parts = split_parts
                    .into_iter()
                    .map(|x| match &x {
                        core::Split::Split(_) => {
                            let split: Split = x.into();
                            split.into()
                        }
                        core::Split::Text(text) => text.to_owned().into(),
                    })
                    .collect();

                Split {
                    text: text.to_owned(),
                    parts,
                }
            }
        }
    }
}

#[wasm_bindgen]
pub struct NNSplit {
    backend: Rc<TractJSBackend>,
    logic: Rc<core::NNSplitLogic>,
}

#[wasm_bindgen]
impl NNSplit {
    #[wasm_bindgen(constructor)]
    pub fn invalid_new() -> Result<(), JsValue> {
        Err("NNSplit can't be constructed directly because it is asynchronous! Please use NNSplit.new.".into())
    }

    /// Create a new splitter from the given model path. The path is passed to `fetch` in the browser or `fs.readFile` in Node.js
    /// Optionally an object containing options can be provided as second argument:
    ///     * threshold (float): Threshold from 0 to 1 above which predictions will be considered positive.
    ///     * stride (int): How much to move the window after each prediction (comparable to stride of 1d convolution).
    ///     * max_length (int): The maximum length of each cut (comparable to kernel size of 1d convolution).
    ///     * padding (int): How much to zero pad the text on both sides.
    ///     * batch_size (int): Batch size to use.
    ///     * length_divisor (int): Total length will be padded until it is divisible by this number. Allows some additional optimizations.
    pub async fn new(path: String, options: JsValue) -> Result<NNSplit, JsValue> {
        utils::set_panic_hook();
        let options = if options.is_undefined() || options.is_null() {
            core::NNSplitOptions::default()
        } else {
            options.into_serde().unwrap()
        };

        // larger batch sizes seem to slow things down so hardcode to 1 for now
        // would need more investigation to find the cause
        let backend = TractJSBackend::new(&path, options.length_divisor, 1).await?;
        let metadata = backend.get_metadata().await?;

        Ok(NNSplit {
            backend: Rc::new(backend),
            logic: Rc::new(core::NNSplitLogic::new(
                options,
                serde_json::from_str(
                    metadata
                        .get("split_sequence")
                        .ok_or("Model must contain `split_sequence` metadata key")?,
                )
                .map_err(|_| "split_sequence must be valid JSON.")?,
            )),
        })
    }

    /// Split texts. Takes an array of strings as input and returns an array of the same length of `Split` objects.
    /// Each split has the properties
    ///     * .text, the text in this `Split`.
    ///     * .parts, the `Split`s contained in this `Split` (e. g. tokens in a sentence).
    /// unless at the lowest level, at which it is just a string.
    pub fn split(&self, texts: Vec<JsValue>) -> Promise {
        let logic = self.logic.clone();
        let backend = self.backend.clone();

        // future_to_promise needed to avoid move - see discussion in https://github.com/rustwasm/wasm-bindgen/issues/1858
        future_to_promise(async move {
            let texts: Vec<String> = texts
                .into_iter()
                .map(|x| x.as_string().unwrap_throw())
                .collect();
            let texts: Vec<&str> = texts.iter().map(|x| x.as_ref()).collect();

            let (inputs, indices) = logic.get_inputs_and_indices(&texts);
            let slice_preds = backend.predict(inputs).await?;

            let splits = logic.split(&texts, slice_preds, indices);
            let splits = splits
                .into_iter()
                .map(|x| {
                    let split: Split = x.into();
                    split.into()
                })
                .collect::<Vec<JsValue>>();

            let array = Array::new();
            for split in &splits {
                array.push(split);
            }

            Ok(array.into())
        })
    }

    /// Gets names of the levels of this splitter.
    #[wasm_bindgen(js_name = getLevels)]
    pub fn get_levels(&self) -> Array {
        self.logic
            .split_sequence()
            .get_levels()
            .iter()
            .map(|x| JsValue::from(x.0.clone()))
            .collect()
    }
}
