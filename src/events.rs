use serde_json::Value;

use bot::LineBot;
use sources::LineSource;
use messages::LineMessage;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LineEventType {
    Unfollow,
    Leave,
}

#[derive(Serialize, Deserialize)]
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



#[derive(Serialize, Deserialize)]
pub enum PostBackParams {
    Date { date: String },
    Time { time: String },
    Datetime { datetime: String },
}

#[derive(Serialize, Deserialize)]
pub struct PostBack {
    data:   String,
    params: PostBackParams,
}

#[derive(Serialize, Deserialize)]
pub enum BeaconEventType {
    Enter,
    Leave,
    Banner,
}

#[derive(Serialize, Deserialize)]
pub struct Beacon {
    hwid: String,
    kind: BeaconEventType,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    result: String,
    nonce:  String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ReplyableEventType {
    Message       { message: LineMessage },
    PostbackEvent { postback: Value },
    BeaconEvent   { beacon: Beacon },
    AccountLink   { link: Link },
    Follow,
    Join,
}


#[derive(Serialize, Deserialize)]
pub struct ReplyableEvent {
    #[serde(flatten, rename = "type")]  
    kind: ReplyableEventType,
    timestamp: u64,
    source: LineSource,
    #[serde(rename = "replyToken")]
    reply_token: String,
}

impl ReplyableEvent {
    pub fn new (kind: ReplyableEventType, timestamp: u64,
        source: LineSource, reply_token: &str) -> ReplyableEvent
    {
        ReplyableEvent {
            kind,
            timestamp,
            source,
            reply_token: String::from(reply_token),
        }
    }


    pub fn reply(&self, msg: Vec<LineMessage>, bot: LineBot) {
        let data = json!({
            "replyToken": self.reply_token,
            "messages": msg
        });
        bot.post("/message/reply", data, json!({}));
    }
}