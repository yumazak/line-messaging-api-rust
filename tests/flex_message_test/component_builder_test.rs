extern crate line_messaging_api_rust as line;
extern crate serde_json;

use line::flex_message::components::Component;
use line::flex_message::component_builder::ComponentBuilder;
use line::actions::Action;

#[test]
// #[ignore]
fn box_component_test() {
    let image     = ComponentBuilder::new()
                        .set_url("https://example.com/flex/images/image.jpg")
                        .build_image();
    let separator = ComponentBuilder::new().build_separator();
    let text      = ComponentBuilder::new()
                        .set_text("Text in the box")
                        .build_text();
    let contents  = vec![image, separator, text];
    let component = ComponentBuilder::new()
                        .set_layout("vertical")
                        .set_contents(contents)
                        .build_box();

    println!("{}", serde_json::to_string(&component).unwrap());
}

#[test]
#[ignore]
fn button_component_test() {
    let action      = Action::create_uri("Tap me", "https://example.com");
    let component   = ComponentBuilder::new()
                        .set_action(action)
                        .set_style("primary")
                        .set_color("#0000ff")
                        .build_button();

    println!("{}", serde_json::to_string(&component).unwrap());
}

#[test]
#[ignore]
fn filler_component_test() {
    let component   = ComponentBuilder::new().build_filler();

    println!("{}", serde_json::to_string(&component).unwrap());
}

#[test]
#[ignore]
fn icon_component_test() {
    let component   = ComponentBuilder::new()
                        .set_url("https://example.com/icon/png/caution.png")
                        .set_size("lg")
                        .build_icon();

    println!("{}", serde_json::to_string(&component).unwrap());
}

#[test]
#[ignore]
fn separator_component_test() {
    let component   = ComponentBuilder::new()
                        .set_color("#000000")
                        .build_separator();

    println!("{}", serde_json::to_string(&component).unwrap());
}

#[test]
#[ignore]
fn spacer_component_test() {
    let component   = ComponentBuilder::new()
                        .set_size("md")
                        .build_spacer();

    println!("{}", serde_json::to_string(&component).unwrap());
}

#[test]
#[ignore]
fn text_component_test() {
    let component   = ComponentBuilder::new()
                        .set_text("Hello, world")
                        .set_size("xl")
                        .set_weight("bold")
                        .set_color("#0000ff")
                        .build_text();

    println!("{}", serde_json::to_string(&component).unwrap());
}