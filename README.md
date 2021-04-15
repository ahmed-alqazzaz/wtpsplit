# NNSplit

[![PyPI](https://img.shields.io/pypi/v/nnsplit)](https://pypi.org/project/nnsplit/)
[![Crates.io](https://img.shields.io/crates/v/nnsplit)](https://crates.io/crates/nnsplit)
[![npm](https://img.shields.io/npm/v/nnsplit)](https://www.npmjs.com/package/nnsplit)
![CI](https://github.com/bminixhofer/nnsplit/workflows/CI/badge.svg)
![License](https://img.shields.io/github/license/bminixhofer/nnsplit)

A tool to split text using a neural network. The main application is sentence boundary detection, but e. g. compound splitting for German is also supported.

## Features

- __Robust__: Not reliant on proper punctuation, spelling and case. See the [metrics](https://bminixhofer.github.io/nnsplit/#metrics).
- __Small__: NNSplit uses a byte-level LSTM, so weights are small (< 4MB) and models can be trained for every unicode encodable language.
- __Portable__: NNSplit is written in Rust with bindings for Rust, Python, and Javascript (Browser and Node.js). See how to get started in the [usage](https://bminixhofer.github.io/nnsplit/#usage) section.
- __Fast__: Up to 2x faster than Spacy sentencization, see the [benchmark](https://bminixhofer.github.io/nnsplit/#benchmark).
- __Multilingual__: NNSplit currently has models for 9 different languages (German, English, French, Norwegian, Swedish, Simplified Chinese, Turkish, Russian and Ukrainian). Try them in the [demo](https://bminixhofer.github.io/nnsplit/#demo).

Documentation has moved to the NNSplit website: https://bminixhofer.github.io/nnsplit.

## License

NNSplit is licensed under the MIT license.
