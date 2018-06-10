extern crate rustc_serialize as serialize;
extern crate reqwest;
extern crate crypto;
extern crate serde;
extern crate tokio;
extern crate bytes;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate futures;

pub mod actions;
pub mod bot;
pub mod events;
pub mod messages;
pub mod sources;
pub mod templates;
pub mod models;



#[cfg(test)]
mod tests {
    use super::bot::LineBot;
    use super::models::LineBotConfig;
    use super::messages::LineMessage;
    use super::messages::LineMessageType;

    use super::serde_json;

    #[test]
    fn hamc_test() {
        let bot = LineBot::new("secret key", "this is a pen.");
        assert_eq!(false, bot.check_signature("a", "This is a pen."));
    }

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
        assert_eq!(r#"{"id":"","type":"text","text":"Hello"}"#, j);
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
        assert_eq!(r#"[{"id":"","type":"text","text":"Hello"},{"id":"","type":"text","text":"i am message2"}]"#, j);
    }

    #[test]
    fn get_test() {
        let bot = LineBot::new(
            "b83d12adec48f46c4434f6cc035f745d",
            "oSY5LoHilHgd4E9sF/HO1Srm/BnAWCW7SqLaErLRybcQURpgq8W9UD/5jCuh4IZIxCbhWLqXshCxVuBKeK0O1x4cnVLan7Y8WZcF84mamJMBvWTV0c9SQzLXTKpua7FZyKTztMY/YLFja+b0VusGwgdB04t89/1O/w1cDnyilFU=",
        );

        let message = LineMessage::new("", LineMessageType::Text{ text: String::from("Hello") });
        let messages = vec![message];
        println!("{}", messages.len());
        bot.push("Ua2829b4c5a9b21984c091fc0b641fa8f", messages);        
    }
}
