extern crate line_messaging_api_rust as line;
extern crate serde_json;

use serde_json::value::Value;

use line::events::ReplyableEvent;
use line::utils;
use line::bot::LineBot;

use common;

#[test]
fn to_replyable_test() {   
    let location = "{\"events\":[{\"message\":{\"address\":\"〒090-0001 北海道北見市小泉\",\"id\":\"8095062583627\",\"latitude\":43.82667841361864,\"longitude\":143.9001400552902,\"type\":\"location\"},\"replyToken\":\"2b00fe9a2395479d8867f8929335d6a0\",\"source\":{\"type\":\"user\",\"userId\":\"Ua2829b4c5a9b21984c091fc0b641fa8f\"},\"timestamp\":1528640971584,\"type\":\"message\"}]}";
    let d: ReplyableEvent = utils::to_events(location).unwrap();
    let message = d.get_message().unwrap().get_address().unwrap();
    println!("{:?}", message);
}

#[test]
fn is_replyable_test() {
    let data = common::get_test_follow();
    assert!(utils::is_replyable(&data))
}
#[test]
fn replyable_test_should_return_false() {
    let data = common::get_test_unfollow();
    assert_eq!(false, utils::is_replyable(&data))
}

#[test]
fn hmac_test() {
    let bot = common::get_bot();
    let body = r#"{"events":[{"type":"message","replyToken":"408c40f387f64fd597483939830e6420","source":{"userId":"Ua2829b4c5a9b21984c091fc0b641fa8f","type":"user"},"timestamp":1528749086843,"message":{"type":"text","id":"8100880274489","text":"な\n"}}]}"#;
    let key = "uXpWAeXL2wiKcIVxcOLYw3DC/5QzwliQnBK1WGscwK4=";
    assert!(bot.check_signature(body, key));
}
#[test]
fn hmac_test2() {
    let bot = common::get_bot();
    let body = r#"{"events":[{"type":"message","replyToken":"6ee5d89f6fcd45918d6e0811e37c04d6","source":{"userId":"Ua2829b4c5a9b21984c091fc0b641fa8f","type":"user"},"timestamp":1528790258300,"message":{"type":"text","id":"8102980405603","text":"な"}}]}"#;
    let key = "EtnSZnGYchr4zm7VDk67hbSOH0L+jxMqfunJJxD4Vek=";
    assert!(bot.check_signature(body, key));
}