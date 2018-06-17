extern crate line_messaging_api_rust as line;
extern crate serde_json;

use line::flex_message::styles::{ Style, BlockStyle, BubbleStyle };
use line::flex_message::component_builder::ComponentBuilder;
use line::flex_message::components::Component;
use line::flex_message::containers::FlexContainer;
use line::messages::LineMessage;
use line::actions::{ ImagemapAction, Action };
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
    let uri: Action = Action::create_uri("View detail", "action=buy&itemid=123");
    let action1 = Action::create_postback("Buy", "action=buy&itemid=123");
    let action2 = Action::create_postback("Add to cart", "action=buy&itemid=123");
    let action3 = Action::create_postback("View detail", "action=buy&itemid=123");
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
    let message_action1: Action = Action::create_message("Yes", "yes");
    let message_action2: Action = Action::create_message("No", "no");
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
    let action1 = Action::create_postback("Buy", "action=buy&itemid=123");
    let action2 = Action::create_postback("Add to cart", "action=buy&itemid=123");
    let action3 = Action::create_postback("View detail", "action=buy&itemid=123");
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
#[ignore]
fn push_image_carousel_template_test() {
    let action1 = Action::create_postback("Buy", "action=buy&itemid=123");

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

#[test]
pub fn push_flex_text() {
    // let filler = ComponentBuilder::new()
    //                 .build_filler();
    // let header = ComponentBuilder::new()
    //                 .set_layout("vertical")
    //                 .set_contents(vec![filler.clone()])
    //                 .build_box();
    let uri_action = Action::create_uri("詳細", "https://www.google.co.jp/search?q=%E5%8C%97%E8%A6%8B+%E5%A4%A9%E6%B0%97&oq=%E5%8C%97%E8%A6%8B%E3%80%80%E5%A4%A9%E6%B0%97&aqs=chrome..69i57j0l2j69i61j0l2.3484j0j4&sourceid=chrome&ie=UTF-8");

    let mut url = String::new();
    url = String::from("https://i.imgur.com/ZxCUfhV.png");

    let hero = ComponentBuilder::new()
                .set_url(&url)
                .set_size("5xl")
                .set_aspect_ratio("2:1")
                .set_action(uri_action.clone())
                .build_image();

    let separator       = ComponentBuilder::new().build_separator();
    let tempaleture     = ComponentBuilder::new()
                            .set_text(&format!("気温:    {}度", 10))
                            .set_size("xl")
                            .set_margin("xl")
                            .build_text();
    let tempaleture_min = ComponentBuilder::new()
                            .set_text(&format!("最高気温: {}度",5))
                            .set_size("xl")
                            .build_text();
    let tempaleture_max = ComponentBuilder::new()
                            .set_text(&format!("最低気温: {}度",5))
                            .set_size("xl")
                            .build_text();


    let contents  = vec![tempaleture, tempaleture_min, tempaleture_max];
    let component = ComponentBuilder::new()
                        .set_layout("vertical")
                        .set_contents(contents)
                        .build_box();

    let button = ComponentBuilder::new()
                    .set_action(uri_action)
                    .set_style("primary")
                    .set_color("#034c76")
                    .build_button();

    let footer = ComponentBuilder::new()
                        .set_layout("vertical")
                        .set_contents(vec![button])
                        .build_box();

    let style = BlockStyle::new("#034c76", false, "");
    let styles = BubbleStyle::new(BlockStyle::create_empty(), style, BlockStyle::create_empty(), BlockStyle::create_empty());

    let bubble = FlexContainer::create_bubble("", Component::create_empty(), hero, component, footer, Style::Bubble{styles});
    let message: LineMessage = LineMessage::create_flex("", "this is a flex", bubble);
    println!("{}", serde_json::to_string(&message).unwrap());
    common::push_message(message);
}

#[test]
fn push_flex_task_test() {
    let separator = ComponentBuilder::new()
                        .set_margin("xxl")
                        .build_separator();
    let filler = ComponentBuilder::new()
                        .build_filler();
    let gropu_name = ComponentBuilder::new()
                        .set_text("group1")
                        .set_size("xxl")
                        .set_align("center")
                        .set_gravity("center")
                        .build_text();

    // let task1 = ComponentBuilder::new()
    //                     .set_text("task1")
    //                     .set_margin("xxl")
    //                     .build_text();
    let task1 = create_task_box("task1");
    let task2 = create_task_box("task2");
    let task3 = create_task_box("task3");
    let body = ComponentBuilder::new()
                .set_layout("vertical")
                .set_contents(vec![gropu_name, separator.clone(), task1, task2, task3])
                .build_box();

    let bubble = FlexContainer::create_bubble("", Component::create_empty(), Component::create_empty(), body, Component::create_empty(), Style::Empty);

    let carousel = FlexContainer::create_carusel(vec![bubble.clone(), bubble.clone()]);
    let message = LineMessage::create_flex("", "tasks", carousel);
    println!("{}", serde_json::to_string(&message).unwrap());
    
    common::push_message(message);
}

fn create_task_box(name: &str) -> Component {
    let task_name = ComponentBuilder::new()
                .set_text(name)
                .set_gravity("center")
                .set_flex(1)
                .build_text();

    let post_back_done = Action::create_postback("完了", "action=done&id=1");

    // let task_name_container = ComponentBuilder::new()
    //                             .set_layout("vertical")
    //                             .set_contents(vec![task_name])
    //                             .set_flex(1)
    //                             .build_box();


    let done_button = ComponentBuilder::new()
                        .set_action(post_back_done)
                        .set_style("primary")
                        .set_gravity("center")
                        .set_color("#034c76")
                        .build_button();

    let post_back_delete = Action::create_postback("削除", "action=delete&id=1");

    let delete_button = ComponentBuilder::new()
                        .set_action(post_back_delete)
                        .set_style("primary")
                        .set_gravity("center")
                        .set_color("#EB2142")
                        .build_button();

    let button_container = ComponentBuilder::new()
                                .set_layout("horizontal")
                                .set_contents(vec![done_button, delete_button])
                                .set_spacing("sm")
                                .set_flex(1)
                                .build_box();

    let contents = vec![task_name, button_container];

    let task = ComponentBuilder::new()
                .set_layout("horizontal")
                .set_contents(contents)
                .set_margin("xxl")
                .build_box();
    task
}

#[test]
fn push_flex_task_menu_test() {
    let separator = ComponentBuilder::new()
                        .set_margin("xxl")
                        .build_separator();
    let filler = ComponentBuilder::new()
                        .build_filler();
    let title = ComponentBuilder::new()
                        .set_text("Tasks")
                        .set_size("xxl")
                        .set_align("center")
                        .set_gravity("center")
                        .build_text();
    
    let post_back_show_tasks       = Action::create_postback("タスク一覧", "show_tasks");
    let post_back_create_task_list = Action::create_postback("新規リスト", "create_task_list");
    let show_tasks_button = ComponentBuilder::new()
                        .set_action(post_back_show_tasks)
                        .set_style("primary")
                        .set_gravity("center")
                        .set_color("#034c76")
                        .build_button();
    let create_list_button = ComponentBuilder::new()
                        .set_action(post_back_create_task_list)
                        .set_style("primary")
                        .set_gravity("center")
                        .set_color("#034c76")
                        .build_button();

    let body = ComponentBuilder::new()
                .set_layout("vertical")
                .set_contents(vec![title, separator.clone(), show_tasks_button, create_list_button])
                .set_spacing("xl")
                .build_box();

    let bubble = FlexContainer::create_bubble("", Component::create_empty(), Component::create_empty(), body, Component::create_empty(), Style::Empty);

    // let carousel = FlexContainer::create_carusel(vec![bubble.clone(), bubble.clone()]);
    let message = LineMessage::create_flex("", "tasks", bubble);
    println!("{}", serde_json::to_string(&message).unwrap());
    
    common::push_message(message);
}
