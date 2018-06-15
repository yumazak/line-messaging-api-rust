#[derive(Serialize, Deserialize, Clone)]
pub struct BlockStyle {
    #[serde(rename = "backgroundColor", skip_serializing_if = "String::is_empty")]
    background_color: String,
    separator       : bool,
    #[serde(rename = "separatorColor", skip_serializing_if = "String::is_empty")]
    separator_color : String,
    #[serde(skip)]
    is_empty        : bool,
}

impl BlockStyle {
    pub fn new(background_color: &str, separator: bool, separator_color: &str) -> BlockStyle {
        BlockStyle {
            background_color: String::from(background_color),
            separator,
            separator_color : String::from(separator_color),
            is_empty        : false,
        }
    }

    pub fn create_empty() -> BlockStyle {
        BlockStyle {
            background_color: String::new(),
            separator       : false,
            separator_color : String::new(),
            is_empty        : true,
        }
    }

    pub fn is_empty(&self) -> bool { self.is_empty }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BubbleStyle {
    #[serde(skip_serializing_if = "BlockStyle::is_empty")]
    header  : BlockStyle,
    #[serde(skip_serializing_if = "BlockStyle::is_empty")]
    hero    : BlockStyle,
    #[serde(skip_serializing_if = "BlockStyle::is_empty")]
    body    : BlockStyle,
    #[serde(skip_serializing_if = "BlockStyle::is_empty")]
    footer  : BlockStyle,
    #[serde(skip)]
    is_empty: bool,
}

impl BubbleStyle {
    pub fn new(header: BlockStyle, hero: BlockStyle, body: BlockStyle, footer: BlockStyle) -> BubbleStyle {
        BubbleStyle { header, hero, body, footer , is_empty: false }
    }

    pub fn create_empty() -> BubbleStyle {
        BubbleStyle {
            header  : BlockStyle::create_empty(),
            hero    : BlockStyle::create_empty(),
            body    : BlockStyle::create_empty(),
            footer  : BlockStyle::create_empty(),
            is_empty: true,
        }
    }

    pub fn is_empty(&self) -> bool { self.is_empty }    
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Style {
    Bubble {
        #[serde(flatten)]
        styles: BubbleStyle
    },
    BlockStyle {
        #[serde(flatten)]        
        styles: BlockStyle
    },
    Empty,
}

impl Style {
    pub fn is_empty(&self) -> bool {
        match self {
            Style::Empty => true,
            _            => false
        }
    }
}