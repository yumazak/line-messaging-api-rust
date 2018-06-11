use actions::ImagemapAction;
use templates::TemplateComponent;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LineMessageType {
    Image,
    Video,
    Audio,
    Text     { text: String },
    Template { alt_text: String, template: TemplateComponent },
    Location { address: String, latitude: f64, longitude: f64 },
    Imagemap { base_url: String, alt_text: String, base_size: u64, actions: Vec<ImagemapAction> },
    File {
        #[serde(rename = "fileName")]
        file_name: String,
        #[serde(rename = "fileSize")]
        file_size: u64,
    },
    Sticker {
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "stickerId")]
        sticker_id: String
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LineMessage {
    #[serde(flatten, rename = "type")]
    pub kind: LineMessageType,
    pub id:   String,
}

impl LineMessage {
    pub fn new(id: &str, kind: LineMessageType) -> LineMessage {
        LineMessage { id: String::from(id), kind }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    
    pub fn get_text(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Text { text } => Some(text),
            _ => None
        }
    }

    pub fn get_alt_text(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Template { alt_text, template }                     => Some(alt_text),
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_url),    
            _ => None
        }
    }

    pub fn get_tempate(&self) -> Option<TemplateComponent> {
        match self.kind.clone() {
            LineMessageType::Template { alt_text, template } => Some(template),
            _ => None
        }
    }

    pub fn get_address(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Location { address, latitude, longitude } => Some(address),
            _ => None
        }
    }

    pub fn get_latitude(&self) -> Option<f64> {
        match self.kind.clone() {
            LineMessageType::Location { address, latitude, longitude } => Some(latitude),
            _ => None
        }
    }

    pub fn get_longitude(&self) -> Option<f64> {
        match self.kind.clone() {
            LineMessageType::Location { address, latitude, longitude } => Some(longitude),
            _ => None
        }
    }

    pub fn get_base_url(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_url),
            _ => None
        }
    }

    pub fn get_base_size(&self) -> Option<u64> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_size),
            _ => None
        }
    }

    pub fn get_action(&self) -> Option<Vec<ImagemapAction>> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(actions),
            _ => None
        }
    }

    pub fn get_file_name(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::File { file_name, file_size } => Some(file_name),
            _ => None
        }
    }

    pub fn get_file_size(&self) -> Option<u64> {
        match self.kind.clone() {
            LineMessageType::File { file_name, file_size } => Some(file_size),
            _ => None
        }
    }

    pub fn get_package_id(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Sticker { package_id, sticker_id } => Some(package_id),
            _ => None
        }
    }

    pub fn get_sticker_id(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Sticker { package_id, sticker_id } => Some(sticker_id),
            _ => None
        }
    }
}
