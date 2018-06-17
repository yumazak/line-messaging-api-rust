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



#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostBackParams {
    date: String,
    #[serde(default)]
    time: String,
    #[serde(default)]
    datetime: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostBack {
    pub data  : String,
    #[serde(default)]
    params: PostBackParams,
}

// impl PostBack {
//     pub fn get_data() -> Option<String> {
//         self.data().clone()
//     }
// }

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
    Postback      { postback: PostBack },
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

    pub fn get_text(&self) -> Option<String> {
        match self.kind.clone() {
            ReplyableEventType::Message { message } => {
                match message.get_text() {
                    Some(text) => Some(text),
                    None => None,
                }
            },
            _ => None
        }
    }
}