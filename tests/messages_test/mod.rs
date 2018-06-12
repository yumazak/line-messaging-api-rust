extern crate line_messaging_api_rust as line;
extern crate serde_json;

use line::messages::LineMessage;
use line::actions::{ ImagemapAction, TemplateAction };
use line::templates::{ TemplateComponent, TemplateColumn, ImageColumn };
use common;

#[test]
#[ignore]
fn push_text_test() {
    let message: LineMessage = LineMessage::create_text("", "hello");
    common::push_message(message);
}

#[test]
#[ignore]
fn push_sticker_test() {
    let message: LineMessage = LineMessage::create_sticker("", "1", "1");
    common::push_message(message);
}

#[test]
#[ignore]
fn push_image_test() {
    let message: LineMessage = LineMessage::create_image("", "https://i.imgur.com/bkZg1vFs.png", "https://i.imgur.com/bkZg1vFt.png");
    common::push_message(message);
}

#[test]
#[ignore]
fn push_video_test() {
    let message: LineMessage = LineMessage::create_video("", "https://example.com/original.jpg", "https://example.com/preview.jpg");
    common::push_message(message);
}

#[test]
#[ignore]
fn push_audio_test() {
    let message: LineMessage = LineMessage::create_audio("", "https://example.com/original.m4a", 60000);
    common::push_message(message);
}

#[test]
#[ignore]
fn push_location_test() {
    let message: LineMessage = LineMessage::create_location("", "test", "〒150-0002 東京都渋谷区渋谷２丁目２１−１", 35.65910807942215, 139.70372892916203);
    common::push_message(message);
}

#[test]
#[ignore]
fn push_imagemap_uri_test() {
    let image_map = ImagemapAction::create_imagemap_uri_action("https://example.com/", "https://example.com/", 0, 0, 520, 1040);
    println!("{}", serde_json::to_string(&image_map).unwrap());
    
    let message: LineMessage = LineMessage::create_imagemap("", "https://example.com/bot/images/rm001", "This is an imagemap", 1040, 1040, vec![image_map]);
    common::push_message(message);
}

#[test]
#[ignore]
fn push_imagemap_message_test() {
    let image_map = ImagemapAction::create_imagemap_message_action("hello", "hello", 0, 0, 520, 1040);
    println!("{}", serde_json::to_string(&image_map).unwrap());
    
    let message: LineMessage = LineMessage::create_imagemap("", "https://example.com/bot/images/rm001", "This is an imagemap", 1040, 1040, vec![image_map]);
    common::push_message(message);
}

#[test]
#[ignore]
fn push_button_template_test() {
    let uri: TemplateAction = TemplateAction::create_uri("View detail", "action=buy&itemid=123");
    let action1 = TemplateAction::create_postback("Buy", "action=buy&itemid=123");
    let action2 = TemplateAction::create_postback("Add to cart", "action=buy&itemid=123");
    let action3 = TemplateAction::create_postback("View detail", "action=buy&itemid=123");
    println!("uri: {}\n", serde_json::to_string(&uri).unwrap());
    println!("action1: {}\n", serde_json::to_string(&action1).unwrap());


    let default_action = vec![uri];
    let actions = vec![action1, action2, action3];
    println!("default_action: {}\n", serde_json::to_string(&default_action).unwrap());

    let template = 
        TemplateComponent::create_buttons("https://example.com/bot/images/image.jpg", "rectangle", "cover", "#FFFFFF", "Menu", "Please select", default_action, actions);
    
    println!("template: {}\n", serde_json::to_string(&template).unwrap());
    
    let message: LineMessage = LineMessage::create_template("", "This is a buttons template", template);

    println!("message: {}\n", serde_json::to_string(&message).unwrap());
    
    common::push_message(message);
}

#[test]
#[ignore]
fn push_confirm_template_test() {
    let message_action1: TemplateAction = TemplateAction::create_message("Yes", "yes");
    let message_action2: TemplateAction = TemplateAction::create_message("No", "no");
    let actions = vec![message_action1, message_action2];
    println!("default_action: {}\n", serde_json::to_string(&actions).unwrap());

    let template = 
        TemplateComponent::create_confirm("Are you sure?", actions);
    
    println!("template: {}\n", serde_json::to_string(&template).unwrap());
    
    let message: LineMessage = LineMessage::create_template("", "This is a buttons template", template);

    println!("message: {}\n", serde_json::to_string(&message).unwrap());
    
    common::push_message(message);
}

#[test]
#[ignore]
fn push_carousel_template_test() {
    let action1 = TemplateAction::create_postback("Buy", "action=buy&itemid=123");
    let action2 = TemplateAction::create_postback("Add to cart", "action=buy&itemid=123");
    let action3 = TemplateAction::create_postback("View detail", "action=buy&itemid=123");
    let actions = vec![action1, action2, action3];
    let column1 = TemplateColumn::new("https://example.com/bot/images/item1.jpg", "#FFFFFF", "this is menu", "description", Vec::new(), actions);

    let column2 = column1.clone();

    println!("default_action: {}\n", serde_json::to_string(&column2).unwrap());

    let columns = vec![column1, column2];
    let template = 
        TemplateComponent::create_carousel(columns, "rectangle", "cover");
    
    println!("template: {}\n", serde_json::to_string(&template).unwrap());
    
    let message: LineMessage = LineMessage::create_template("", "this is a carousel template", template);

    println!("message: {}\n", serde_json::to_string(&message).unwrap());
    
    common::push_message(message);
}

#[test]
// #[ignore]
fn push_image_carousel_template_test() {
    let action1 = TemplateAction::create_postback("Buy", "action=buy&itemid=123");

    let column1 = ImageColumn::new("https://example.com/bot/images/item1.jpg", action1);

    let column2 = column1.clone();
    let column3 = column1.clone();

    println!("default_action: {}\n", serde_json::to_string(&column2).unwrap());

    let columns = vec![column1, column2, column3];

    let template = TemplateComponent::create_image_carousel(columns);
    
    println!("template: {}\n", serde_json::to_string(&template).unwrap());
    
    let message: LineMessage = LineMessage::create_template("", "this is a image carousel template", template);

    println!("message: {}\n", serde_json::to_string(&message).unwrap());
    
    common::push_message(message);
}