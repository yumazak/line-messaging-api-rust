extern crate line_messaging_api_rust as line;
extern crate serde_json;

use line::flex_message::styles::{ Style, BlockStyle, BubbleStyle };
use line::flex_message::component_builder::ComponentBuilder;
use line::flex_message::components::Component;
use line::flex_message::containers::FlexContainer;

#[test]
#[ignore]
pub fn bubble_container_test() {
    let text1 = ComponentBuilder::new()
                    .set_text("Header text")
                    .build_text();
    let box1  = ComponentBuilder::new()
                    .set_layout("vertical")
                    .set_contents(vec![text1.clone()])
                    .build_box();
    let image = ComponentBuilder::new()
                    .set_url("https://example.com/flex/images/image.jpg")
                    .build_image();
    let box2  = ComponentBuilder::new()
                    .set_layout("vertical")
                    .set_contents(vec![text1.clone()])
                    .build_box();
    let box3  = ComponentBuilder::new()
                    .set_layout("vertical")
                    .set_contents(vec![text1.clone()])
                    .build_box();
    
    let bubble = FlexContainer::create_bubble("", box1, image, box2, box3, Style::Empty);

    println!("{}", serde_json::to_string(&bubble).unwrap());
    
}

#[test]
pub fn carousel_container_test() {
    let text = ComponentBuilder::new()
                    .set_text("first_bubble")
                    .build_text();
    let box1  = ComponentBuilder::new()
                    .set_layout("vertical")
                    .set_contents(vec![text.clone()])
                    .build_box();
    let bubble = FlexContainer::create_bubble("", Component::create_empty(), Component::create_empty(), box1, Component::create_empty(), Style::Empty);

    let carousel = FlexContainer::create_carusel(vec![bubble.clone(), bubble.clone()]);

    println!("{}", serde_json::to_string(&carousel).unwrap());
}