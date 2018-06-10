use line_actions::ImagemapAction;

pub enum LineMessageType {
    Text{text: String},
    Image{original_content_url: String, preview_image_url: String },
    Sticker{package_id: String, sticker_id: String },
    Video{ original_content_url: String, preview_image_url: String },
    Audio{ original_content_url: String, duration: u32 },
    Location{ address: String, latitude: u32, longitude: String },
    Imagemap{ base_url: String, alt_text: String, base_size: u32, actions: Vec<ImagemapAction> },
    Template{ alt_text: String, template: TemplateComponent }
}

pub struct LineMessage {
    id:   String,
    kind: LineMessageType,
}

impl LineMessage {
    pub fn new(id: &str, kind: LineMessageType) -> LineMessage {
        LineMessage { id: String::from(id), kind }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }  
}
