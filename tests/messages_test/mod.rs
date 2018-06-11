extern crate line_messaging_api_rust as line;

use line::events::ReplyableEvent;
use line::utils;

use common;

#[test]
fn text_message_test() {
    println!("{}", module_path!());
    
    let data = common::get_test_text();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn image_test() {
    let data = common::get_test_image();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}