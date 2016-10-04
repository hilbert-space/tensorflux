# TensorFlux [![Version][version-icon]][version-page] [![Status][status-icon]][status-page]

The package provides an interface to [TensorFlow][tensorflow].

## [Documentation][documentation]
[![Chat on IRC](https://img.shields.io/badge/mozilla-%tensorflow-brightgreen.svg)](irc://irc.mozilla.org/rust-tensorflow)

## [Example](examples/multiplication.rs)

Create a graph in Python:

```python
import tensorflow as tf

a = tf.placeholder(tf.float32, name='a')
b = tf.placeholder(tf.float32, name='b')
c = tf.mul(a, b, name='c')

tf.train.write_graph(tf.Session().graph_def, '', 'graph.pb', as_text=False)
```

Evaluate the graph in Rust:

```rust
use tensorflux::{Buffer, Input, Options, Output, Session, Tensor};

macro_rules! ok(($result:expr) => ($result.unwrap()));

let graph = "graph.pb"; // c = a * b
let mut session = ok!(Session::new(&ok!(Options::new())));
ok!(session.extend(&ok!(Buffer::load(graph))));

let a = ok!(Tensor::new(vec![1f32, 2.0, 3.0], &[3]));
let b = ok!(Tensor::new(vec![4f32, 5.0, 6.0], &[3]));

let inputs = vec![Input::new("a", a), Input::new("b", b)];
let mut outputs = vec![Output::new("c")];
ok!(session.run(&inputs, &mut outputs, &[], None, None));

let c = ok!(outputs[0].get::<f32>());
assert_eq!(&c[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
```

This and other examples can be found in the [examples](examples) directory.

## [Requirements][requirements]

## [Configuration][configuration]

## Collaboration

Rust has an IRC culture, and most real-time collaborations happen in a variety
of channels on Mozilla’s IRC network, irc.mozilla.org. The channels that are
relevant to TensorFlow are #rust-machine-learning and #rust-tensorflow.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[configuration]: https://github.com/stainless-steel/tensorflux-sys#configuration
[documentation]: https://stainless-steel.github.io/tensorflux
[requirements]: https://github.com/stainless-steel/tensorflux-sys#requirements
[tensorflow]: https://www.tensorflow.org

[status-icon]: https://travis-ci.org/stainless-steel/tensorflux.svg?branch=master
[status-page]: https://travis-ci.org/stainless-steel/tensorflux
[version-icon]: https://img.shields.io/crates/v/tensorflux.svg
[version-page]: https://crates.io/crates/tensorflux
