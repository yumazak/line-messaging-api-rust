use actions::TemplateAction;

#[derive(Serialize, Deserialize)]
pub enum LineTemplateType {
    TemplateButtons  { thumbnail_image_url: String, title: String, text: String, actions: Vec<TemplateAction> },
    TemplateConfirm  { text: String, actions: Vec<TemplateAction> },
    TemplateCarousel { columns: Vec<TemplateColumn> }
}

#[derive(Serialize, Deserialize)]
pub struct TemplateComponent {
    kind: LineTemplateType
}

impl TemplateComponent {
    pub fn new (kind: LineTemplateType) -> TemplateComponent {
        TemplateComponent { kind }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TemplateColumn {
    thumbnail_image_url: String,
    title:               String,
    text:                String,
    actions:             Vec<TemplateAction>
}

impl TemplateColumn {
    pub fn new (thumbnail_image_url: String, title: String, text: String, actions: Vec<TemplateAction>) -> TemplateColumn {
        TemplateColumn { thumbnail_image_url, title, text, actions } 
    }
}