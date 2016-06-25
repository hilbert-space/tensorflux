extern crate tensorflux;

use tensorflux::{Options, Session};

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(unused_variables)]
fn main() {
    let options = ok!(Options::new());
    let session = ok!(Session::new(options));
}
