use serde::de::Deserialize;
use serde_json;
use serde_json::Error;
use serde_json::value::Value;

use events::{ ReplyableEvent };

pub fn is_replyable(data: &str) -> bool {
    let events: Value = match serde_json::from_str(data) {
        Ok(events) => events,
        Err(_) => return false,
    };

    serde_json::to_string(&events["events"][0]["replyToken"]).unwrap() != "null"
}

pub fn to_events<'a, T>(data: &str) -> Result<T, String>
where for<'de> T: Deserialize<'de>, 
{
    let events: Value = match serde_json::from_str(data) {
        Ok(events) => events,
        Err(err) => return Err(err.to_string()),
    };
    
    let contents: Value = match serde_json::to_value(&events["events"][0]) {
        Ok(contents) => contents,
        Err(err) => return Err(err.to_string()),
    };
    serde_json::from_value(contents).map_err(|err| { err.to_string() })
}