extern crate tensorflux;

use tensorflux::{Definition, Input, Options, Output, Session, Tensor};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(unused_variables)]
fn main() {
    let mut session = ok!(Session::new(ok!(Options::new())));
    ok!(session.extend(&ok!(Definition::load(GRAPH_PATH)))); // c = a * b

    let a = ok!(Tensor::new(vec![1f32, 2f32, 3f32], &[3]));
    let b = ok!(Tensor::new(vec![4f32, 5f32, 6f32], &[3]));
    let mut c = ok!(Tensor::new(vec![0f32, 0f32, 0f32], &[3]));

    ok!(session.run(
        vec![Input::new("a:0", a), Input::new("b:0", b)],
        vec![Output::new("c:0", &mut c)],
        vec![],
    ));

    assert_eq!(&c[..], &[0f32, 0f32, 0f32]);
}
