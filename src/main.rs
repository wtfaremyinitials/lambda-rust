extern crate rustc_serialize;
use std::io;
use std::collections::BTreeMap;
use std::thread;
use rustc_serialize::json;
use rustc_serialize::json::Json;

fn main() {
    loop { receive(); }
}

fn receive() {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read event");

    let root = Json::from_str(&data).expect("Failed to parse JSON");
    let root = root.as_object().expect("Root of event JSON was not of type Json::Object");

    let event = root.get("event")
                .expect("No key `event` on message object")
                .as_object().unwrap().clone();

    let context = root.get("context").expect("No key `context` on message object");
    let context: EventContext = json::decode(&context.to_string()).unwrap();

    thread::spawn(move || {
        let res = handle(event, &context);

        let output = json::encode(&EventResponse {
            invokeid: context.invokeid,
            response: res
        }).expect("Failed to encode response");

        println!("{}", output);
    });
}

fn handle(event: BTreeMap<String, Json>, context: &EventContext) -> Json {
    let foo = event.get("foo").unwrap().as_string().unwrap().to_string();
    let value = foo + "-with-" + &context.memoryLimitInMB + "mb";
    Json::String(value.to_string())
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

#[derive(RustcEncodable)]
struct EventResponse {
    invokeid: String,
    response: Json
}
