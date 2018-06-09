use structs::Rectangle;

pub enum LineActionType {
    Uri,
    Message,
    Postback
}

pub struct ImagemapAction {
    kind: String,
    area: Rectangle
}

impl ImagemapAction {
    pub fn new(kind: String, area: Rectangle) -> ImagemapAction {
        ImagemapAction { kind: kind, area: area}
    }

    // createFromObject(params: String) {

    // }
}

