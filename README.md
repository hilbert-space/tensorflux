# TensorFlux [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [TensorFlow][tensorflow].

## [Documentation][documentation]

## [Example][example]

```rust
use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

macro_rules! ok(($result:expr) => ($result.unwrap()));

let mut session = ok!(Session::new(ok!(Options::new())));
ok!(session.extend(&ok!(Definition::load(GRAPH_PATH)))); // c = a * b

let a = ok!(Tensor::new(vec![1f32, 2.0, 3.0], &[3]));
let b = ok!(Tensor::new(vec![4f32, 5.0, 6.0], &[3]));

let inputs = vec![Input::new("a", a), Input::new("b", b)];
let mut outputs = vec![Output::new("c")];

ok!(session.run(inputs, &mut outputs, vec![]));

let c: Tensor<f32> = outputs.pop().unwrap().into().unwrap();

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
