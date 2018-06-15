extern crate line_messaging_api_rust as line;
extern crate serde_json;

use line::flex_message::styles::{ Style, BlockStyle, BubbleStyle };

#[test]
pub fn style_tst() {
    let block1 = BlockStyle::new("#00ffff", false, "");
    let block2 = BlockStyle::new("", true, "#000000");
    let block3 = BlockStyle::new("#00ffff", true, "#00000");

    let bubble = BubbleStyle::new(block1, block2, BlockStyle::create_empty(), block3);
    let style = Style::Bubble {
        styles: bubble
    };

    println!("{}", serde_json::to_string(&style).unwrap());
    
}