use serde_json::Value;

use bot::LineBot;
use sources::LineSource;
use messages::LineMessage;

pub enum LineEventType {

}

pub struct LineEvent {
    kind:      LineEventType,
    timestamp: u32,
    source:    LineSource,
}

impl LineEvent {
    pub fn new (kind: LineEventType, timestamp: u32, source: LineSource) -> LineEvent {
        LineEvent { kind, timestamp, source }
    }

}

pub enum BeaconEventType {
    Enter,
}

pub struct Beacon {
    hwid: String,
    kind: BeaconEventType,
}

pub enum ReplyableEventType {
    MessageEvent   { message: LineMessage },
    PostbackEvent  { postback: Value },
    BeaconEvent    { beacon: Beacon }
}

pub struct ReplyableEvent {
    kind: ReplyableEventType,
    timestamp: u32,
    source: LineSource,
    reply_token: String,
    bot: LineBot

}

impl ReplyableEvent {
    pub fn new (kind: ReplyableEventType, timestamp: u32,
        source: LineSource, reply_token: String, bot: LineBot) -> ReplyableEvent
    {
        ReplyableEvent { kind, timestamp, source, reply_token, bot}
    }


    pub fn reply(&self, msg: Vec<LineMessage>) {
        let data = json!({
            "replyToken": self.reply_token,
            "messages": msg
        });
        self.bot.post("/message/reply", data, json!({}));
    }
}