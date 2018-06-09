use reqwest::Client;
use serialize::hex::ToHex;
use serialize::base64::{STANDARD, ToBase64};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use serde_json::Value;

use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;

use structs::LineBotConfig;
use line_messages::{LineMessageType, LineMessage};

static BASE_URL: &'static str = "https://api.line.me/v2/bot";

pub struct LineBot {
    pub config: LineBotConfig,
    pub client: Client,
}

impl LineBot {
    pub fn new(channel_secret: &str, channel_token: &str) -> LineBot {
        LineBot { 
            config: LineBotConfig::new(channel_secret, channel_token),
            client: Client::new()
        }
    }

    // fn initHyper(&self) {
    //     self.client = Client::new().expect("Couldn't create client");
    // }

    pub fn check_signature(&self, signature: &str, body: &str) -> bool {
        let mut hmac = Hmac::new(
            Sha256::new(), 
            self.config.get_channel_secret().as_bytes()
        );
        hmac.input(body.as_bytes());

        let mut hmac_hashed = Sha1::new();
        hmac_hashed.input_str(&hmac.result().code().to_base64(STANDARD));

        let mut signature_hashed = Sha1::new();
        signature_hashed.input_str(signature);

        println!("hmac_hashed: {}", hmac_hashed.result_str());
        println!("signature_hashed: {}", signature_hashed.result_str());

        hmac_hashed.result_str() == signature_hashed.result_str()
    }

    pub fn push(&self, to: &str, msg: Vec<LineMessage>) {
        let mut messages: Vec<String> = vec![];

        for message in msg.into_iter() {
            messages.push(message.get_text());
        }


        let mut data = HashMap::new();
        let json_data = json!(["an", "value"]).to_string();
        data.insert(String::from("to"), String::from(to));
        data.insert(String::from("messages"), json_data);

        self.post("/message/push", data, "test");
    }

    pub fn post(&self, endpoint: &str, data: HashMap<String, String>, options: &str) {
        let url = format!("{}{}{}", BASE_URL, endpoint, options);
        println!("{}", url);
        // self.client.post(&url).send().expect("Failed to send request");
    }
}

