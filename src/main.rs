extern crate aws_lambda;

use std::collections::BTreeMap;
use aws_lambda::Json;

#[allow(unused_variables)]
fn handle(event: BTreeMap<String, Json>, context: BTreeMap<String, Json>) -> Json {
    Json::String("Hello World".to_string())
}

fn main() {
    aws_lambda::listen(handle);
}
