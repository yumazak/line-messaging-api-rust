use reqwest::Client;
use serialize::hex::ToHex;
use serialize::base64::{STANDARD, ToBase64};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use serde_json::Value;
use reqwest::Response; 

use std::thread;
use std::sync::mpsc;
use std::io::Read;
use std::collections::HashMap;

use structs::LineBotConfig;
use line_messages::{LineMessageType, LineMessage};
use line_sources::LineSources;

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

        self.post("/message/push", data, HashMap::new());
    }

    pub fn get_content_from_message(&self, message: LineMessage) {
        self.get_content(message.get_id())
    }

    // need error process;
    pub fn get_content(&self, message_id: String) {
        let endpoint = format!("/message/{}/content", message_id);
        let mut data = HashMap::new();

        data.insert(String::from("responseType"), String::from("stream"));
        self.get(endpoint.as_str(), data);
    }

    pub fn get_profile_from_user_source(&self, user: LineSources) {
        match user {
            LineSources::User{ id } => self.get_profile(&id),
            _ => {}
        }
    }

    pub fn get_profile(&self, user_id: &str) {
        let endpoint = format!("/profile/{}", user_id);

        self.get(&endpoint, HashMap::new());
    }


    pub fn leave_from_source(&self, source: LineSources) {
        match source {
            LineSources::Group { group_id, user_id } => self.leave(LineSources::Group { group_id, user_id }),
            LineSources::Room { room_id, user_id }   => self.leave(LineSources::Room { room_id, user_id }),
            _                                        => {}
        }
    }

    pub fn leave(&self, kind: LineSources) {
        let url = match kind {
            LineSources::Group { group_id, user_id } => format!("/group/{}/leave", group_id),
            LineSources::Room { room_id, user_id }   => format!("/room/{}/leave", room_id),
            _                                        => String::new()
        };

        self.post(&url, HashMap::new(), HashMap::new());
    }

    //i dont know what is options
    pub fn get(&self, endpoint: &str, options: HashMap<String, String>) -> Response{
        let url = format!("{}{}", BASE_URL, endpoint);

        self.client.get(&url)
            .form(&options)
            .send()
            .expect(&format!("Failed to get to {}", endpoint))
    }

    pub fn post(&self, endpoint: &str, data: HashMap<String, String>, options: HashMap<String, String>) -> Response{
        let url = format!("{}{}", BASE_URL, endpoint);
        println!("{}", url);

        self.client.post(&url)
            .form(&data)
            .form(&options)            
            .send()
            .expect(&format!("Failed to post to {}", endpoint))
    }

}

