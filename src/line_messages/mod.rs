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
    id:   u32,
    kind: LineMessageType,
    text: String,
}

impl LineMessage {
    pub fn new(id: u32, kind: LineMessageType, text: &str) -> LineMessage {
        LineMessage { id: id, kind: kind, text: String::from(text)}
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}
