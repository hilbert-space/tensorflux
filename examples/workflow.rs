extern crate tensorflux;

use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(unused_variables)]
fn main() {
    let mut session = ok!(Session::new(ok!(Options::new())));
    ok!(session.extend(&ok!(Definition::load(GRAPH_PATH)))); // c = a * b

    let mut inputs = vec![];
    inputs.push(Input::new("a:0", ok!(Tensor::new(vec![1f32, 2f32, 3f32], &[3]))));
    inputs.push(Input::new("b:0", ok!(Tensor::new(vec![4f32, 5f32, 6f32], &[3]))));

    let mut outputs = vec![];
    outputs.push(Output::new("c:0"));

    let targets = vec![];

    ok!(session.run(inputs, outputs, targets));
}
