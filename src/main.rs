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
                .as_object().expect("`event` key was not an object").clone();

    let context = root.get("context")
                .expect("No key `context` on message object")
                .as_object().expect("`context` key was not an object").clone();

    thread::spawn(move || {
        let invokeid = context.get("invokeid").expect("no key `invokeid`")
                .as_string().expect("`invokeid` wasn't a string").to_string();

        let res = handle(event, context);

        let output = json::encode(&EventResponse {
            invokeid: invokeid.to_string(),
            response: res
        }).expect("Failed to encode response");

        println!("{}", output);
    });
}

#[allow(unused_variables)]
fn handle(event: BTreeMap<String, Json>, context: BTreeMap<String, Json>) -> Json {
    event.get("foo").expect("no prop foo").clone()
}

#[derive(RustcEncodable)]
struct EventResponse {
    invokeid: String,
    response: Json
}
