use actions::Action;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type",rename_all = "snake_case")]
pub enum TemplateType {
    Confirm  { text: String, actions: Vec<Action> },    
    Buttons  {
        #[serde(rename = "thumbnailImageUrl")]
        thumbnail_image_url   : String,
        #[serde(rename = "imageAspectRatio")]
        image_aspect_ratio    : String,
        #[serde(rename = "imageSize")]
        image_size            : String,
        #[serde(rename = "imageBackgroundColor")]
        image_background_color: String,
        title                 : String,
        text                  : String,
        default_actions       : Vec<Action>,
        actions               : Vec<Action>,
    },
    Carousel {
        columns           : Vec<TemplateColumn>,
        #[serde(rename = "imageAspectRatio")]
        image_aspect_ratio: String,
        #[serde(rename = "imageSize")]
        image_size        : String
    },
    ImageCarousel {
        columns: Vec<ImageColumn>,    
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateComponent {
    #[serde(flatten)]
    kind   : TemplateType,
}

impl TemplateComponent {
    pub fn new (kind: TemplateType, alt_text: &str) -> TemplateComponent {
        TemplateComponent { kind }
    }

    pub fn create_confirm(text: &str, actions: Vec<Action>) -> TemplateComponent {
        TemplateComponent { kind: TemplateType::Confirm{ text: String::from(text), actions } }      
    }

    pub fn create_buttons(thumbnail_image_url: &str, image_aspect_ratio: &str, image_size: &str, image_background_color: &str,
        title: &str, text: &str, default_actions: Vec<Action>, actions: Vec<Action>,) -> TemplateComponent
    {
        TemplateComponent {
            kind: TemplateType::Buttons {
                thumbnail_image_url   : String::from(thumbnail_image_url),
                image_aspect_ratio    : String::from(image_aspect_ratio),
                image_background_color: String::from(image_background_color),
                image_size            : String::from(image_size),
                title                 : String::from(title),
                text                  : String::from(text),
                default_actions,
                actions,
            }
        }
    }

    pub fn create_carousel(columns: Vec<TemplateColumn>, image_aspect_ratio: &str, image_size: &str) -> TemplateComponent {
        TemplateComponent {
            kind: TemplateType::Carousel {
                image_aspect_ratio: String::from(image_aspect_ratio),
                image_size        : String::from(image_size),
                columns,                
            }
        }
    }

    pub fn create_image_carousel(columns: Vec<ImageColumn>) -> TemplateComponent {
        TemplateComponent { kind: TemplateType::ImageCarousel { columns } }
    }

    // pub fn create_image_carousel(contents: Vec<FlexMessage>) -> TemplateComponent {
    //     TemplateComponent { kind: TemplateType::Flex { contents } }
    // }


}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateColumn {
    #[derive(rename = "thumbnailImageUrl")]
    thumbnail_image_url   : String,
    #[serde(rename = "imageBackgroundColor")]
    image_background_color: String,
    title                 : String,
    text                  : String,
    #[serde(skip_serializing_if = "Vec::is_empty")]    
    default_actions       : Vec<Action>,
    actions               : Vec<Action>,
}

impl TemplateColumn {
    pub fn new(thumbnail_image_url: &str, image_background_color: &str, title: &str, text: &str, default_actions: Vec<Action>, actions: Vec<Action>) -> TemplateColumn {
        TemplateColumn {
            thumbnail_image_url   : String::from(thumbnail_image_url),
            image_background_color: String::from(image_background_color),
            title                 : String::from(title),
            text                  : String::from(text),
            default_actions,
            actions,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageColumn {
    #[serde(rename = "imageUrl")]
    image_url: String,
    action  : Action,
}

impl ImageColumn {
    pub fn new(image_url: &str, action: Action) -> ImageColumn {
        ImageColumn { image_url: String::from(image_url), action }
    }
}

