use serde_json::Value;

use bot::LineBot;
use sources::LineSource;
use messages::LineMessage;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LineEventType {
    Unfollow,
    Leave,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LineEvent {
    #[serde(flatten, rename = "type")]  
    kind     : LineEventType,
    timestamp: u64,
    source   : LineSource,
}

impl LineEvent {
    pub fn new (kind: LineEventType, timestamp: u64, source: LineSource) -> LineEvent {
        LineEvent { kind, timestamp, source }
    }

}



#[derive(Serialize, Deserialize, Clone)]
pub enum PostBackParams {
    Date { date: String },
    Time { time: String },
    Datetime { datetime: String },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostBack {
    data  : String,
    params: PostBackParams,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BeaconEventType {
    Enter,
    Leave,
    Banner,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Beacon {
    hwid: String,
    kind: BeaconEventType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    result: String,
    nonce : String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ReplyableEventType {
    Message       { message: LineMessage },
    Postback      { postback: Value },
    Beacon        { beacon: Beacon },
    AccountLink   { link: Link },
    Follow,
    Join,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ReplyableEvent {
    #[serde(flatten, rename = "type")]  
    pub kind       : ReplyableEventType,
    pub timestamp  : u64,
    pub source     : LineSource,
    #[serde(rename = "replyToken")]
    pub reply_token: String,
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

    pub fn get_message(&self) -> Option<LineMessage> {
        match self.kind.clone() {
            ReplyableEventType::Message { message } => Some(message),
            _ => None
        }
    }
}