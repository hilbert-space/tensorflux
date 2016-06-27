extern crate tensorflux;

use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let mut session = ok!(Session::new(ok!(Options::new())));
    ok!(session.extend(&ok!(Definition::load(GRAPH_PATH)))); // c = a * b

    let a = ok!(Tensor::new(vec![1f32, 2.0, 3.0], vec![3]));
    let b = ok!(Tensor::new(vec![4f32, 5.0, 6.0], vec![3]));

    let inputs = vec![Input::new("a", a), Input::new("b", b)];
    let outputs = vec![Output::new("c")];

    let mut results = ok!(session.run(inputs, outputs, vec![]));

    let c: Tensor<f32> = results.pop().unwrap().into().unwrap();

    assert_eq!(&c[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
}
