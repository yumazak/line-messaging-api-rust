pub struct LineBotConfig {
    channel_secret: String,
    channel_token:  String,
}

impl LineBotConfig {
    pub fn new(channel_secret: &str, channel_token: &str) -> LineBotConfig{
        LineBotConfig {
            channel_secret: String::from(channel_secret),
            channel_token:  String::from(channel_token)
        }
    }

    pub fn get_channel_secret(&self) -> String{
        self.channel_secret.clone()
    }
}

pub struct Rectangle {
    x:      u32,
    y:      u32,
    width:  u32,
    height: u32
}

// pub struct PostData {
//     to: String,
    
// }