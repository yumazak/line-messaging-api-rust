extern crate line_messaging_api_rust as line;
extern crate serde_json;

use line::bot::LineBot;
use line::models::LineBotConfig;
use line::messages::{ LineMessageType, LineMessage};
use line::events::{ ReplyableEventType, ReplyableEvent };
use line::sources::{ LineSourceType, LineSource };

use serde_json::{Value, Error};

pub mod common;
pub mod utils_test;
pub mod events_test;
pub mod messages_test;
pub mod flex_message_test;

#[test]
fn url_test() {
    let bot = LineBot::new("secret key", "this is a pen.");
    // let message = LineMessage::new("testid", LineMessageType::Text, "hi");
    // bot.push("Ua2829b4c5a9b21984c091fc0b641fa8f", vec![message]);
}

#[test]
fn message_test() {
    let message = LineMessage::new("", LineMessageType::Text{ text: String::from("Hello") });
    let j = serde_json::to_string(&message).unwrap();
    println!("message_json: {}", j);
}

#[test]
fn message_vec_test() {
    let mut messages = vec![];
    let message = LineMessage::new("", LineMessageType::Text{ text: String::from("Hello") });
    let message2 = LineMessage::new("", LineMessageType::Text{ text: String::from("i am message2") });
    messages.push(message);
    messages.push(message2);
    let j = serde_json::to_string(&messages).unwrap();
    println!("messages_json: {}", j);
}

#[test]
#[ignore]
fn get_test() {
    let bot = common::get_bot();

    let message = LineMessage::new("", LineMessageType::Text{ text: String::from("Hello") });
    bot.push_message("Ua2829b4c5a9b21984c091fc0b641fa8f", message);        
}

#[test]
fn get_signature_test() {
    let bot = common::get_bot();
    let body = r#"{"message":"from google app script"}"#;
    println!("{}", bot.get_signature(body))
}