use flex_message::components::Component;
use flex_message::styles::Style;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FlexContainerType {
    Bubble {
        #[serde(skip_serializing_if = "String::is_empty")]
        direction: String,
        #[serde(skip_serializing_if = "Component::is_empty")]
        header   : Component,
        #[serde(skip_serializing_if = "Component::is_empty")]
        hero     : Component,
        #[serde(skip_serializing_if = "Component::is_empty")]
        body     : Component,
        #[serde(skip_serializing_if = "Component::is_empty")]
        footer   : Component,
        #[serde(skip_serializing_if = "Style::is_empty")]
        styles   : Style,
    },
    Carousel {
        contents: Vec<FlexContainer>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlexContainer {
    #[serde(flatten)]
    kind: FlexContainerType
}

impl FlexContainer {
    pub fn new(kind: FlexContainerType) -> FlexContainer{
        FlexContainer { kind }
    }

    pub fn create_bubble(direction: &str, header: Component, hero: Component, body: Component, footer: Component, styles: Style) -> FlexContainer {
        FlexContainer { kind: FlexContainerType::Bubble { direction: String::from(direction), header, hero, body, footer, styles } }
    }

    pub fn create_carusel(contents: Vec<FlexContainer>) -> FlexContainer {
        FlexContainer { kind: FlexContainerType::Carousel { contents }}
    }
}
