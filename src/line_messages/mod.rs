pub enum LineMessageType {
    Text,
    Image,
    Video,
    Audio,
    Location,
    Sticker,
    Imagemap,
    Template,
}

pub struct LineMessage {
    id:   String,
    kind: LineMessageType,
    text: String,
}

impl LineMessage {
    pub fn new(id: &str, kind: LineMessageType, text: &str) -> LineMessage {
        LineMessage { id: String::from(id), kind: kind, text: String::from(text)}
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    
}
