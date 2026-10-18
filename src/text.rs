use crate::{Element, ExpandElement, Link, Style, Styled};

pub struct Text {
    id: Option<String>,
    link: Option<Link>,
    style: Style,
    texts: Vec<TextSpan>
}

impl Styled for Text {
    fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Element for Text {
    fn expand(&mut self) -> ExpandElement {
        todo!()
    }

    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn set_id(mut self, id: &str) -> Self where Self: Sized {
        self.id = Some(id.to_string());
        self
    }

    fn get_link(&self) -> Option<&Link> {
        Option::from(&self.link)
    }

    fn set_link(mut self, link: &str, new_tab: bool) -> Self where Self: Sized {
        self.link = Some(Link::from(link, new_tab));
        self
    }
}

pub struct TextSpan {
    style: Style,
    content: String,
}

impl Styled for TextSpan {
    fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }
}