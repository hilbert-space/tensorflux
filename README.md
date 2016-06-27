# TensorFlux [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [TensorFlow][tensorflow].

## [Documentation][documentation]

## [Example][example]

```rust
use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

let mut session = Session::new(Options::new().unwrap()).unwrap();
session.extend(&Definition::load(GRAPH_PATH).unwrap()).unwrap(); // c = a * b

let a = Tensor::new(vec![1f32, 2.0, 3.0], vec![3]).unwrap();
let b = Tensor::new(vec![4f32, 5.0, 6.0], vec![3]).unwrap();

let inputs = vec![Input::new("a", a), Input::new("b", b)];
let outputs = vec![Output::new("c")];

let mut results = session.run(inputs, outputs, vec![]).unwrap();

let c: Tensor<f32> = results.pop().unwrap().into().unwrap();

assert_eq!(&c[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
```

## [Requirements][requirements]

## [Compilation][compilation]

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[compilation]: https://github.com/stainless-steel/tensorflow-sys#compilation
[documentation]: https://stainless-steel.github.io/tensorflux
[example]: examples/workflow.rs
[requirements]: https://github.com/stainless-steel/tensorflow-sys#requirements
[tensorflow]: https://www.tensorflow.org

[status-img]: https://travis-ci.org/stainless-steel/tensorflux.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tensorflux
[version-img]: https://img.shields.io/crates/v/tensorflux.svg
[version-url]: https://crates.io/crates/tensorflux
