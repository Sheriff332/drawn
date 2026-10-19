use crate::{Element, ExpandElement, ImplProperties, Link, Properties, Style, Transform};

pub struct Text {
    properties: Properties,
    texts: Vec<TextSpan>,
}

impl_properties!(Text);

impl Element for Text {
    fn expand(&mut self) -> ExpandElement {
        todo!()
    }
}

pub struct TextSpan {
    style: Style,
    content: String,
}

impl ImplProperties for TextSpan {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
