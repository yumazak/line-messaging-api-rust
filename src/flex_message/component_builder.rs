use flex_message::components::Component;
use actions::Action;

pub struct ComponentBuilder {
    align           : String,
    aspect_ratio    : String,
    aspect_mode     : String,
    background_color: String,
    color           : String,
    gravity         : String,
    height          : String,
    layout          : String,
    margin          : String,
    spacing         : String,
    style           : String,
    size            : String,
    text            : String,
    url             : String,
    wrap            : bool,
    weight          : String,
    flex            : u64,
    contents        : Vec<Component>,
    action          : Action,
}

impl ComponentBuilder {
    pub fn new() -> ComponentBuilder {
        ComponentBuilder {
            align           : String::new(),
            aspect_ratio    : String::new(),
            aspect_mode     : String::new(),
            background_color: String::new(),
            color           : String::new(),
            gravity         : String::new(),
            height          : String::new(),
            layout          : String::new(),
            margin          : String::new(),
            spacing         : String::new(),
            style           : String::new(),
            size            : String::new(),
            text            : String::new(),
            url             : String::new(),
            weight          : String::new(),
            wrap            : false,
            flex            : 0,
            contents        : vec![],
            action          : Action::create_empty(),
        }
    }

    pub fn build_box(&mut self) -> Component {
        Component::create_box(&self.layout.clone(), self.contents.clone(), self.flex.clone(), &self.spacing.clone(), &self.margin.clone())
    }

    pub fn build_button(&mut self) -> Component {
        Component::create_button(self.action.clone(), self.flex.clone(), &self.margin.clone(), &self.height.clone(), &self.style.clone(), &self.color.clone(), &self.gravity.clone())
    }

    pub fn build_filler(&mut self) -> Component { Component::create_filler() }

    pub fn build_icon(&mut self) ->Component {
        Component::create_icon(&self.url.clone(), &self.margin.clone(), &self.size.clone(), &self.aspect_ratio.clone())
    }

    pub fn build_image(&mut self) -> Component {
        Component::create_image(&self.url.clone(), self.flex.clone(), &self.margin.clone(), &self.align.clone(), &self.gravity.clone(), &self.size.clone(), &self.aspect_ratio.clone(), &self.aspect_mode.clone(), &self.background_color.clone(), self.action.clone())
    }

    pub fn build_separator(&mut self) -> Component {
        Component::create_separator(&self.margin.clone(), &self.color.clone())
    }

    pub fn build_spacer(&mut self) -> Component {
        Component::create_spacer(&self.size.clone())
    }

    pub fn build_text(&mut self) -> Component {
        Component::create_text(&self.text.clone(), self.flex.clone(), &self.margin.clone(), &self.size.clone(), &self.align.clone(), &self.gravity.clone(), self.wrap.clone(), &self.weight.clone(), &self.color.clone())
    }

    //set values
    pub fn set_align(&mut self, value: &str)              -> &mut ComponentBuilder { self.align = String::from(value); self }
    pub fn set_aspect_ratio(&mut self, value: &str)       -> &mut ComponentBuilder { self.aspect_ratio = String::from(value); self }
    pub fn set_aspect_mode(&mut self, value: &str)        -> &mut ComponentBuilder { self.aspect_mode = String::from(value); self }
    pub fn set_background_color(&mut self, value: &str)   -> &mut ComponentBuilder { self.background_color = String::from(value); self }
    pub fn set_color(&mut self, value: &str)              -> &mut ComponentBuilder { self.color = String::from(value); self }
    pub fn set_gravity(&mut self, value: &str)            -> &mut ComponentBuilder { self.gravity = String::from(value); self }
    pub fn set_height(&mut self, value: &str)             -> &mut ComponentBuilder { self.height = String::from(value); self }
    pub fn set_layout(&mut self, value: &str)             -> &mut ComponentBuilder { self.layout = String::from(value); self }
    pub fn set_margin(&mut self, value: &str)             -> &mut ComponentBuilder { self.margin = String::from(value); self }
    pub fn set_spacing(&mut self, value: &str)            -> &mut ComponentBuilder { self.spacing = String::from(value); self }
    pub fn set_style(&mut self, value: &str)              -> &mut ComponentBuilder { self.style = String::from(value); self }
    pub fn set_size(&mut self, value: &str)               -> &mut ComponentBuilder { self.size = String::from(value); self }
    pub fn set_text(&mut self, value: &str)               -> &mut ComponentBuilder { self.text = String::from(value); self }
    pub fn set_url(&mut self, value: &str)                -> &mut ComponentBuilder { self.url = String::from(value); self }
    pub fn set_weight(&mut self, value: &str)             -> &mut ComponentBuilder { self.weight = String::from(value); self }
    pub fn set_wrap(&mut self, value: bool)               -> &mut ComponentBuilder { self.wrap = value; self }
    pub fn set_flex(&mut self, value: u64)                -> &mut ComponentBuilder { self.flex = value; self }
    pub fn set_contents(&mut self, value: Vec<Component>) -> &mut ComponentBuilder { self.contents = value; self }
    pub fn set_action(&mut self, value: Action)           -> &mut ComponentBuilder { self.action = value; self }

}