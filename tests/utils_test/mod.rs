extern crate line_messaging_api_rust as line;
extern crate serde_json;

use serde_json::value::Value;

use line::events::ReplyableEvent;
use line::utils;

#[test]
fn to_replyable_test() {   
    let location = "{\"events\":[{\"message\":{\"address\":\"〒090-0001 北海道北見市小泉\",\"id\":\"8095062583627\",\"latitude\":43.82667841361864,\"longitude\":143.9001400552902,\"type\":\"location\"},\"replyToken\":\"2b00fe9a2395479d8867f8929335d6a0\",\"source\":{\"type\":\"user\",\"userId\":\"Ua2829b4c5a9b21984c091fc0b641fa8f\"},\"timestamp\":1528640971584,\"type\":\"message\"}]}";
    let d: ReplyableEvent = utils::to_events(location).unwrap();
    let message = d.get_message().unwrap().get_address().unwrap();
    println!("{:?}", message);
}