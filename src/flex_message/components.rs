use actions::Action;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ComponentType {
    Box {
        layout  : String,
        contents: Vec<Component>,
        flex    : u64,
        #[serde(skip_serializing_if = "String::is_empty")]
        spacing : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        margin  : String,
    },
    Button {
        action : Action,
        flex   : u64,
        #[serde(skip_serializing_if = "String::is_empty")]
        margin : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        height : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        style  : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        color  : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        gravity: String,
    },
    Filler,
    Icon {
        #[serde(skip_serializing_if = "String::is_empty")]
        url         : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        margin      : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        size        : String,
        #[serde(rename = "aspectRatio", skip_serializing_if = "String::is_empty")]
        aspect_ratio: String,
    },
    Image {
        #[serde(skip_serializing_if = "String::is_empty")]
        url             : String,
        flex            : u64,
        #[serde(skip_serializing_if = "String::is_empty")]
        margin          : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        align           : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        gravity         : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        size            : String,
        #[serde(rename = "aspectRatio", skip_serializing_if = "String::is_empty")]
        aspect_ratio    : String,
        #[serde(rename = "aspectMode", skip_serializing_if = "String::is_empty")]
        aspect_mode     : String,
        #[serde(rename = "backgroundColor", skip_serializing_if = "String::is_empty")]
        background_color: String,
        #[serde(skip_serializing_if = "Action::is_empty")]
        action          : Action,
    },
    Separator {
        #[serde(skip_serializing_if = "String::is_empty")]
        margin: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        color : String
    },
    Spacer {
        #[serde(skip_serializing_if = "String::is_empty")]
        size: String        
    },
    Text {
        #[serde(skip_serializing_if = "String::is_empty")]
        text    : String,
        flex    : u64,
        #[serde(skip_serializing_if = "String::is_empty")]
        margin  : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        size    : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        align   : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        gravity : String,
        wrap    : bool,
        #[serde(skip_serializing_if = "String::is_empty")]
        weight  : String,
        #[serde(skip_serializing_if = "String::is_empty")]
        color   : String,
    },
    Empty,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Component {
    #[serde(flatten, rename = "type")]
    pub kind    : ComponentType,
}

impl Component {
    pub fn new(kind: ComponentType) -> Component {
        Component { kind }
    }

    pub fn create_empty() -> Component {
        Component { kind: ComponentType::Empty }
    }

    pub fn is_empty(&self) -> bool {
        match self.kind {
            ComponentType::Empty => true,
            _                    => false,
        }
    }

    //create component
    pub fn create_box(layout: &str, contents: Vec<Component>, flex: u64, spacing: &str, margin: &str) -> Component {
        Component {
            kind: ComponentType::Box {
                layout : String::from(layout),
                contents,
                flex,
                spacing: String::from(spacing),
                margin : String::from(margin),
            }
        }
    }

    pub fn create_button(action: Action, flex: u64, margin: &str, height: &str, style: &str, color: &str, gravity: &str) -> Component {
        Component {
            kind: ComponentType::Button {
                action,
                flex,
                margin : String::from(margin),
                height : String::from(height),
                style  : String::from(style),
                color  : String::from(color),
                gravity: String::from(gravity),
            }
        }
    }

    pub fn create_filler() -> Component {
        Component { kind: ComponentType::Filler }
    }

    pub fn create_icon(url: &str, margin: &str, size: &str, aspect_ratio: &str) -> Component {
        Component {
            kind: ComponentType::Icon {
                url         : String::from(url),
                margin      : String::from(margin),
                size        : String::from(size),
                aspect_ratio: String::from(aspect_ratio)
            }
        }
    }

    pub fn create_image(url: &str, flex: u64, margin: &str, align: &str, gravity: &str, size: &str, aspect_ratio: &str, aspect_mode: &str, background_color: &str, action: Action) -> Component {
        Component {
            kind: ComponentType::Image {
                url             : String::from(url),
                flex,
                margin          : String::from(margin),
                align           : String::from(align),
                gravity         : String::from(gravity),
                size            : String::from(size),
                aspect_ratio    : String::from(aspect_ratio),
                aspect_mode     : String::from(aspect_mode),
                background_color: String::from(background_color),
                action,
            }
        }
    }

    pub fn create_separator(margin: &str, color: &str) -> Component {
        Component {
            kind: ComponentType::Separator {
                margin: String::from(margin),
                color : String::from(color),
            }
        }
    }

    pub fn create_spacer(size: &str) -> Component {
        Component {
            kind: ComponentType::Spacer {
                size: String::from(size),
            }
        }
    }

    pub fn create_text(text: &str, flex: u64, margin: &str, size: &str, align: &str, gravity: &str, wrap: bool, weight: &str, color: &str) -> Component {
        Component {
            kind: ComponentType::Text {
                text   : String::from(text),
                flex,
                margin : String::from(margin),
                size   : String::from(size),
                align  : String::from(align),
                gravity: String::from(gravity),
                wrap,
                weight : String::from(weight),
                color  : String::from(color),
            }
        }
    }
}