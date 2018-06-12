extern crate line_messaging_api_rust as line;
extern crate serde_json;

use common::serde_json::value::Value;

use line::bot::LineBot;
use line::messages::LineMessage;

use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn get_bot() -> LineBot {
    println!("{}", module_path!());
    let mut f = File::open("tests/common/config.json").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let config: Value = serde_json::from_slice(&buffer).unwrap();
    LineBot::new(
        &config["channel_secret"].as_str().unwrap(),
        &config["channel_token"].as_str().unwrap(),
    )
}

pub fn get_user_id() -> String {
    let mut f = File::open("tests/common/config.json").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let config: Value = serde_json::from_slice(&buffer).unwrap();
    String::from(config["user_id"].as_str().unwrap())
}

pub fn push_message(message: LineMessage) {
    let bot     = get_bot();
    let user_id = get_user_id();
    bot.push_message(&user_id, message);
}

pub fn get_test_text() -> String {
    String::from(
        r#"
            {"events":[{"type":"message","replyToken":"63ca831b72f94011b38bde2676d7a6eb","source":{"userId":"Ua2829b4c5a9b21984c091fc0b641fa8f","type":"user"},"timestamp":1528728227563,"message":{"type":"text","id":"8100074921758","text":"はい"}}]}
        "#
    )
}

pub fn get_test_image() -> String {
    String::from(
        r#"
            {"events":[{"type":"message","replyToken":"378bcb8c251b43c79393aedf7418ff02","source":{"userId":"Ua2829b4c5a9b21984c091fc0b641fa8f","type":"user"},"timestamp":1528728934280,"message":{"type":"image","id":"8100131723119"}}]}
        "#
    )
}

pub fn get_test_video() -> String {
    String::from(
        r#"
            {"events":[{"type":"message","replyToken":"c03cce4ba8514cf49b2139c26c6c7486","source":{"userId":"Ua2829b4c5a9b21984c091fc0b641fa8f","type":"user"},"timestamp":1528732060803,"message":{"type":"video","id":"8100347360051"}}]}
        "#
    )
}

pub fn get_test_audio() -> String {
    String::from(
        r#"
            {"events":[{"type":"message","replyToken":"e8355dc0634b4e6097d0365454610643","source":{"userId":"Ua2829b4c5a9b21984c091fc0b641fa8f","type":"user"},"timestamp":1528732180464,"message":{"type":"audio","id":"8100354340055"}}]}
        "#
    )
}

pub fn get_test_file() -> String {
    String::from(
        r#"
            {"events" :[{"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA","type": "message","timestamp": 1462629479859,"source": {"type": "user","userId": "U4af4980629..."},"message": {"id": "325708","type": "file","fileName": "file.txt","fileSize": 2138}}]}
        "#
    )
}

pub fn get_test_location() -> String {
    String::from(
        r#"
            {"events" :[{"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA","type": "message","timestamp": 1462629479859,"source": {"type": "user","userId": "U4af4980629..."},"message": {"id": "325708","type": "file","fileName": "file.txt","fileSize": 2138}}]}    
        "#
    )
}

pub fn get_test_follow() -> String {
    String::from(
        r#"
            {"events" :[{"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA","type": "message","timestamp": 1462629479859,"source": {"type": "user","userId": "U4af4980629..."},"message": {"id": "325708","type": "file","fileName": "file.txt","fileSize": 2138}}]}    
        "#
    )
}

pub fn get_test_unfollow() -> String {
    String::from(
        r#"
            {"events": [{"type": "unfollow","timestamp": 1462629479859,"source": {"type": "user","userId": "U4af4980629..."}}]}
        "#
    )
}

pub fn get_test_join() -> String {
    String::from(
        r#"
            {"events": [{"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA","type": "join","timestamp": 1462629479859,"source": {"type": "group","groupId": "C4af4980629..."}}]}
        "#
    )
}

pub fn get_test_leave() -> String {
    String::from(
        r#"
            {"events":[{"type":"leave","source":{"groupId":"C4991a36653054b7525ac684337557e23","type":"group"},"timestamp":1528734310446}]}
        "#
    )
}

pub fn get_test_postback() -> String {
    String::from(
        r#"
            {"events": [{"type":"postback","replyToken":"b60d432864f44d079f6d8efe86cf404b","source":{"userId":"U91eeaf62d...","type":"user"},"timestamp":1513669370317,"postback":{"data":"storeId=12345","params":{"datetime":"2017-12-25T01:00"}}}]}
        "#
    )
}

pub fn get_test_beacon() -> String {
    String::from(
        r#"
            {"events": [{"replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA","type": "beacon","timestamp": 1462629479859,"source": {"type": "user","userId": "U4af4980629..."},"beacon": {"hwid": "d41d8cd98f","type": "enter"}}]}
        "#
    )
}
pub fn get_test_account_link() -> String {
    String::from(
        r#"
            {"events": [{"type": "accountLink","replyToken": "b60d432864f44d079f6d8efe86cf404b","source": {"userId": "U91eeaf62d...","type": "user"},"timestamp": 1513669370317,"link": {"result": "ok","nonce": "xxxxxxxxxxxxxxx"}}]}
        "#
    )
}