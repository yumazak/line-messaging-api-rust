extern crate line_messaging_api_rust as line;

use line::events::{ LineEvent, ReplyableEvent };
use line::utils;

use common;

#[test]
fn text_message_test() {  
    let data = common::get_test_text();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn image_test() {
    let data = common::get_test_image();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn file_test() {
    let data = common::get_test_file();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn location_test() {
    let data = common::get_test_location();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn follow_test() {
    let data = common::get_test_follow();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn unfollow_test() {
    let data = common::get_test_unfollow();
    let content: LineEvent = utils::to_events(&data).unwrap();
}

#[test]
fn join_test() {
    let data = common::get_test_join();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn leave_test() {
    let data = common::get_test_leave();
    let content: LineEvent = utils::to_events(&data).unwrap();
}

#[test]
fn postback_test() {
    // let data = common::get_test_postback();
    let data = r#" {"events" :[{"postback":{"data":"action=done&id=1"},"replyToken":"96137b2f5ca94ad89b0c00cddffcbf26","source":{"type":"user","userId":"Ua2829b4c5a9b21984c091fc0b641fa8f"},"timestamp":1529131400505,"type":"postback"}]}"#;
    let content: ReplyableEvent = utils::to_events(&data).unwrap();

}

#[test]
fn beacon_test() {
    let data = common::get_test_postback();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]
fn account_link_test() {
    let data = common::get_test_account_link();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
}

#[test]

fn replyable_get_text_test() {
    let data = common::get_test_text();
    let content: ReplyableEvent = utils::to_events(&data).unwrap();
    println!("{}", content.get_text().unwrap());
}