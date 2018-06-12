use models::Rectangle;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ImageActionType {
    Uri     {
        #[serde(rename = "linkUri")]
        link_uri: String,
    },
    Message {
        text : String
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImagemapAction {
    #[serde(rename="type", flatten)]
    kind: ImageActionType,
    #[serde(default)]
    label   : String,
    area: Rectangle,
}

impl ImagemapAction {
    pub fn new(kind: ImageActionType, label: &str, area: Rectangle) -> ImagemapAction {
        ImagemapAction { kind: kind, label: String::from(label), area: area}
    }

    pub fn create_imagemap_uri_action(label: &str, link_uri: &str, x: u64, y: u64, width: u64, height: u64) -> ImagemapAction {
        ImagemapAction {
            kind: ImageActionType::Uri { link_uri: String::from(link_uri) },
            label: String::from(label),
            area: Rectangle { x, y, width, height }
        }
    }

    pub fn create_imagemap_message_action(label: &str, text: &str, x: u64, y: u64, width: u64, height: u64) -> ImagemapAction {
        ImagemapAction {
            kind: ImageActionType::Message { text: String::from(text) },
            label: String::from(label),
            area: Rectangle { x, y, width, height }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TemplateActionType {
    Message { text: String },
    Uri { uri: String },
    Postback { 
        data: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        display_text: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        text: String,
    },
    Datetimepicker {
        data   : String,
        mode   : String,
        initial: String,
        max    : String,
        min    : String,
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateAction {
    #[serde(flatten)]
    kind:  TemplateActionType,
    label: String,
}

impl TemplateAction {
    pub fn new(kind: TemplateActionType, label: &str) -> TemplateAction {
        TemplateAction { kind, label: String::from(label) }
    }

    pub fn create_message(label: &str, text: &str) -> TemplateAction {
        TemplateAction { label: String::from(label), kind: TemplateActionType::Message { text: String::from(text)} }
    }

    pub fn create_uri(label: &str, uri: &str) -> TemplateAction {
        TemplateAction { label: String::from(label), kind: TemplateActionType::Uri { uri: String::from(uri)} }
    }

    pub fn create_postback(label: &str, data: &str) -> TemplateAction {
        TemplateAction {
            label: String::from(label),
            kind: TemplateActionType::Postback {
                data        : String::from(data),
                display_text: String::new(),
                text        : String::new(),
            }
        }
    }
    pub fn create_postback_with_display_text(label: &str, data: &str, display_text: &str, text: &str) -> TemplateAction {
        TemplateAction {
            label: String::from(label),
            kind: TemplateActionType::Postback {
                data        : String::from(data),
                display_text: String::from(display_text),
                text        : String::new(),
            }
        }
    }
    pub fn create_postback_with_text(label: &str, data: &str, text: &str) -> TemplateAction {
        TemplateAction {
            label: String::from(label),
            kind: TemplateActionType::Postback {
                data        : String::from(data),
                display_text: String::new(),
                text        : String::from(text),
            }
        }
    }

    pub fn create_datetimepicker(label: &str, data: &str, mode: &str, initial: &str, max: &str, min: &str) -> TemplateAction {
        TemplateAction {
            label: String::from(label),
            kind: TemplateActionType::Datetimepicker {
                data   : String::from(data),
                mode   : String::from(mode),
                initial: String::from(initial),
                max    : String::from(max),
                min    : String::from(min),
            }
        }
    }

}