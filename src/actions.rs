use models::Rectangle;

#[derive(Serialize, Deserialize, Clone)]
pub enum ImageActionType {
    ImagemapURIAction     { link_url: String },
    ImagemapMessageAction { text: String },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImagemapAction {
    kind: ImageActionType,
    area: Rectangle,
}

impl ImagemapAction {
    pub fn new(kind: ImageActionType, area: Rectangle) -> ImagemapAction {
        ImagemapAction { kind: kind, area: area}
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TemplateActionType {
    TemplateURIAction { uri: String },
    TemplateMessageAction { text: String },
    TemplatePostbackAction { text: String, data: String }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateAction {
    kind:  TemplateActionType,
    label: String,
}

impl TemplateAction {
    pub fn new(kind: TemplateActionType, label: String) -> TemplateAction {
        TemplateAction { kind, label }
    }
}