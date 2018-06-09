extern crate rustc_serialize as serialize;
extern crate reqwest;
extern crate crypto;
#[macro_use]
extern crate serde_json;

mod line_actions;
mod line_bot; 
mod line_messages;
mod line_sources;
mod structs;



#[cfg(test)]
mod tests {
    use super::line_bot::LineBot;
    use super::structs::LineBotConfig;
    use super::line_messages::LineMessage;
    use super::line_messages::LineMessageType;

    #[test]
    fn hamc_test() {
        let bot = LineBot::new("secret key", "this is a pen.");
        assert_eq!(false, bot.check_signature("a", "This is a pen."));
    }

    #[test]
    fn url_test() {
        let bot = LineBot::new("secret key", "this is a pen.");
        let message = LineMessage::new("testid", LineMessageType::Text, "hi");
        bot.push("Ua2829b4c5a9b21984c091fc0b641fa8f", vec![message]);
    }
}
