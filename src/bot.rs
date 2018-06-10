use reqwest::Client;
use reqwest::header::{ Authorization, Bearer };
use reqwest::Response; 

use serialize::base64::{STANDARD, ToBase64};

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;

use serde_json::Value;

use std::io::Read;
use std::collections::HashMap;

use models::LineBotConfig;
use messages::LineMessage;
use sources::{ LineSource, LineSourceType };

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

        hmac_hashed.result_str() == signature_hashed.result_str()
    }

    pub fn push(&self, to: &str, msg: Vec<LineMessage>) {
        let data = json!({
            "to": to,
            "messages": msg
        });

        self.post("/message/push", data, json!({}));
    }

    pub fn get_content_from_message(&self, message: LineMessage) {
        self.get_content(message.get_id())
    }

    pub fn get_content(&self, message_id: String) {
        let endpoint = format!("/message/{}/content", message_id);
        let mut data = HashMap::new();

        data.insert(String::from("responseType"), String::from("stream"));
        self.get(endpoint.as_str(), data);
    }

    pub fn get_profile_from_user_source(&self, user: LineSource) {
        match user.kind {
            LineSourceType::User{ user_id } => self.get_profile(&user_id),
            _ => {}
        }
    }

    pub fn get_profile(&self, user_id: &str) {
        let endpoint = format!("/profile/{}", user_id);

        self.get(&endpoint, HashMap::new());
    }

    // pub fn leave_from_source(&self, source: LineSource) {
    //     match source.kind {
    //         LineSourceType::Group { group_id, user_id } => self.leave(source),
    //         LineSourceType::Room { room_id, user_id }   => self.leave(source),
    //         _                                        => {}
    //     }
    // }

    pub fn leave(&self, source: LineSource) {
        let url = match source.kind {
            LineSourceType::Group { group_id, user_id } => format!("/group/{}/leave", group_id),
            LineSourceType::Room { room_id, user_id }   => format!("/room/{}/leave", room_id),
            _                                           => String::new()
        };

        self.post(&url, json!({}), json!({}));
    }

    pub fn get(&self, endpoint: &str, options: HashMap<String, String>) -> Response{
        let url = format!("{}{}", BASE_URL, endpoint);

        self.client.get(&url)
            .form(&options)
            .send()
            .expect(&format!("Failed to get to {}", endpoint))
    }

    pub fn post(&self, endpoint: &str, data: Value, options: Value) -> Response{
        let url = format!("{}{}", BASE_URL, endpoint);

        let mut response = self.client.post(&url)
            .header(
                Authorization(
                    Bearer {
                        token: self.config.get_channel_token()
                    }
                )
            )
            .json(&data)
            .send()
            .expect(&format!("Failed to post to {}", endpoint));

        let mut buf = String::new();
        response.read_to_string(&mut buf).expect("Failed to read response");

        // println!("url: {}", url);
        // println!("body: {}", data);
        // println!("Response: {}", buf);
        // println!("res: {:?}", response);
        response
    }
}

