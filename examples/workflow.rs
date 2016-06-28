extern crate tensorflux;

use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let mut session = ok!(Session::new(ok!(Options::new())));
    ok!(session.extend(&ok!(Definition::load(GRAPH_PATH)))); // c = a * b

    let mut inputs = vec![Input::new("a"), Input::new("b")];
    inputs[0].set(ok!(Tensor::new(vec![1f32, 2.0, 3.0], &[3])));
    inputs[1].set(ok!(Tensor::new(vec![4f32, 5.0, 6.0], &[3])));

    let mut outputs = vec![Output::new("c")];

    ok!(session.run(&mut inputs, &mut outputs, &vec![]));

    let result = ok!(outputs[0].get::<f32>());
    assert_eq!(&result[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
}
