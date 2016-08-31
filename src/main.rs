extern crate rustc_serialize;
use std::io;
use std::collections::BTreeMap;
use std::thread;
use rustc_serialize::json::Json;

fn main() {
    loop { receive(); }
}

#[allow(non_snake_case,dead_code)]
#[derive(RustcDecodable)]
struct EventContext {
    functionName: String,
    invokeid: String,
    awsRequestId: String,
    invokedFunctionArn: String,
    memoryLimitInMB: String,
    functionVersion: String,
    logGroupName: String,
    logStreamName: String
}

fn receive() {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read event");

    let root = Json::from_str(&data).expect("Failed to parse JSON");
    let obj = root.as_object().expect("root was not of type Json::Object");

    let event = obj.get("event")
                .expect("No key `event` on message object")
                .as_object().unwrap().clone();
    let context = obj.get("context")
                .expect("No key `context` on message object")
                .clone();

    thread::spawn(move || {
        let res = handle(event, context);
        println!("{}", res); // TODO: This should also output the invokeid for out-of-order responses
    });
}

#[allow(unused_variables)]
fn handle(event: BTreeMap<String, Json>, context: Json) -> Json {
    let value = event.get("foo").unwrap().as_string().unwrap();
    Json::String(value.to_string())
}
