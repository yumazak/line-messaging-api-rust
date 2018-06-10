use actions::ImagemapAction;
use templates::TemplateComponent;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LineMessageType {
    Image,
    Video,
    Audio,
    Text     { text: String },
    Template { alt_text: String, template: TemplateComponent },
    Location { address: String, latitude: f64, longitude: f64 },
    Imagemap { base_url: String, alt_text: String, base_size: u32, actions: Vec<ImagemapAction> },
    File {
        #[serde(rename = "fileName")]
        file_name: String,
        #[serde(rename = "fileSize")]
        file_size: String,
    },
    Sticker {
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "stickerId")]
        sticker_id: String
    },
}

#[derive(Serialize, Deserialize)]
pub struct LineMessage {
    #[serde(flatten, rename = "type")]
    kind: LineMessageType,
    id:   String,
}

impl LineMessage {
    pub fn new(id: &str, kind: LineMessageType) -> LineMessage {
        LineMessage { id: String::from(id), kind }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }  
}
