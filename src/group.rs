use crate::*;

#[derive(Default)]
pub struct Group {
    id: Option<String>,
    link: Option<Link>,
    style: Style,
    elements: Vec<Box<dyn Element>>,
}

impl Element for Group {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children({
            let mut out = std::mem::take(&mut self.elements);
            for element in &mut out {
                *element.get_style() = self.style.clone();
            }
            out
        })
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

impl Styled for Group {
    fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Group {
    pub fn add<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }
}
