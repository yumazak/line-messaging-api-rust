use actions::ImagemapAction;
use templates::TemplateComponent;
use flex_message::containers::FlexContainer;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseSize {
    pub height: u64,
    pub width : u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LineMessageType {
    Text     { text: String },
    Image {
        #[serde(default, rename = "originalContentUrl")]     
        original_content_url: String,
        #[serde(default, rename = "previewImageUrl")]
        preview_image_url   : String,
    },
    Video {
        #[serde(default, rename = "originalContentUrl")]        
        original_content_url: String,
        #[serde(default, rename = "previewImageUrl")]
        preview_image_url   : String,
    },
    Audio {
        #[serde(default, rename = "originalContentUrl")]        
        original_content_url: String,
        #[serde(default)]
        duration            : u64,
    },
    Location {
        #[serde(default)]
        title    : String,
        address  : String,
        latitude : f64,
        longitude: f64
    },
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
    Imagemap { 
        #[serde(rename = "baseUrl")]
        base_url: String,
        #[serde(rename = "altText")]
        alt_text: String,
        #[serde(rename = "baseSize")]
        base_size: BaseSize,
        actions: Vec<ImagemapAction>
    },
    Template {
        #[serde(default, rename = "altText")]             
        alt_text: String,
        template: TemplateComponent
    },
    Flex {
        #[serde(default, rename = "altText")]             
        alt_text: String,
        contents: FlexContainer,
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LineMessage {
    #[serde(flatten, rename = "type")]
    pub kind: LineMessageType,
    pub id  : String,
}

//builder and getter
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
            _                                                                    => None
        }
    }

    pub fn get_tempate(&self) -> Option<TemplateComponent> {
        match self.kind.clone() {
            LineMessageType::Template { alt_text, template } => Some(template),
            _                                                => None
        }
    }

    pub fn get_address(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Location { title, address, latitude, longitude } => Some(address),
            _                                                                 => None
        }
    }

    pub fn get_latitude(&self) -> Option<f64> {
        match self.kind.clone() {
            LineMessageType::Location { title, address, latitude, longitude } => Some(latitude),
            _                                                                 => None
        }
    }

    pub fn get_longitude(&self) -> Option<f64> {
        match self.kind.clone() {
            LineMessageType::Location { title, address, latitude, longitude } => Some(longitude),
            _                                                                 => None
        }
    }

    pub fn get_base_url(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_url),
            _                                                                    => None
        }
    }

    pub fn get_base_size(&self) -> Option<BaseSize> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_size),
            _                                                                    => None
        }
    }
    
    pub fn get_base_size_height(&self) -> Option<u64> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_size.height),
            _                                                                    => None
        }
    }

    pub fn get_base_size_width(&self) -> Option<u64> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(base_size.width),
            _                                                                    => None
        }
    }

    pub fn get_action(&self) -> Option<Vec<ImagemapAction>> {
        match self.kind.clone() {
            LineMessageType::Imagemap { base_url, alt_text, base_size, actions } => Some(actions),
            _                                                                    => None
        }
    }

    pub fn get_file_name(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::File { file_name, file_size } => Some(file_name),
            _                                              => None
        }
    }

    pub fn get_file_size(&self) -> Option<u64> {
        match self.kind.clone() {
            LineMessageType::File { file_name, file_size } => Some(file_size),
            _                                              => None
        }
    }

    pub fn get_package_id(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Sticker { package_id, sticker_id } => Some(package_id),
            _                                                   => None
        }
    }

    pub fn get_sticker_id(&self) -> Option<String> {
        match self.kind.clone() {
            LineMessageType::Sticker { package_id, sticker_id } => Some(sticker_id),
            _                                                   => None
        }
    }
}

//crete message
impl LineMessage {
    pub fn create_image(id: &str, original_content_url: &str, preview_image_url: &str) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Image {
                original_content_url: String::from(original_content_url),
                preview_image_url   : String::from(preview_image_url),
            }
        }
    }

    pub fn create_video(id: &str, original_content_url: &str, preview_image_url: &str) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Video {
                original_content_url: String::from(original_content_url),
                preview_image_url   : String::from(preview_image_url),
            }
        }
    }

    pub fn create_audio(id: &str, original_content_url: &str, duration: u64) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Audio {
                original_content_url: String::from(original_content_url),
                duration,
            }
        }
    }

    pub fn create_text(id: &str, text: &str) -> LineMessage {
        LineMessage { 
            id: String::from(id),
            kind: LineMessageType::Text {
                text: String::from(text)
            }
        }
    }

    pub fn create_location(id: &str, title: &str, address: &str, latitude: f64, longitude: f64) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Location {
                title: String::from(title),
                address: String::from(address),
                latitude,
                longitude,
            }
        }
    }

    pub fn create_imagemap(id: &str, base_url: &str, alt_text: &str, height: u64, width: u64, actions: Vec<ImagemapAction>) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Imagemap {
                base_url: String::from(base_url),
                alt_text: String::from(alt_text),
                base_size: BaseSize { height, width },
                actions,
            }
        }
    }

    pub fn create_file(id: &str, file_name: &str, file_size: u64) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::File {
                file_name: String::from(file_name),
                file_size
            }
        }
    }

    pub fn create_sticker(id: &str, package_id: &str, sticker_id: &str) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Sticker {
                package_id: String::from(package_id),
                sticker_id: String::from(sticker_id),
            }
        }
    }

    pub fn create_template(id: &str, alt_text: &str, template: TemplateComponent) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Template {
                alt_text: String::from(alt_text),
                template,
            }
        }
    }

    pub fn create_flex(id: &str, alt_text: &str, contents: FlexContainer) -> LineMessage {
        LineMessage {
            id: String::from(id),
            kind: LineMessageType::Flex {
                alt_text: String::from(alt_text),
                contents,
            }
        }
    }
}