extern crate tensorflux;

use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(unused_variables)]
fn main() {
    let options = ok!(Options::new());
    let mut session = ok!(Session::new(options));

    let definition = ok!(Definition::load(GRAPH_PATH)); // c = a * b
    ok!(session.extend(&definition));

    let (mut inputs, mut outputs) = (vec![], vec![]);

    let tensor = ok!(Tensor::new(vec![1f32, 2f32, 3f32], &[3]));
    inputs.push(Input::new("a:0", tensor));

    let tensor = ok!(Tensor::new(vec![4f32, 5f32, 6f32], &[3]));
    inputs.push(Input::new("b:0", tensor));

    outputs.push(Output::new("c:0"));

    ok!(session.run(inputs, outputs));
}
