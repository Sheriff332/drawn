use crate::*;

#[derive(Default)]
pub struct Group {
    id: Option<String>,
    link: Option<Link>,
    clip: bool,
    clip_path: Option<String>,
    style: Style,
    elements: Vec<Box<dyn Element>>,
}

impl Element for Group {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children({
            let mut out = std::mem::take(&mut self.elements);
            for element in &mut out {
                *element.get_style_mut() = self.style.clone();
            }
            out
        })
    }

    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn id(mut self, _id: &str) -> Self where Self: Sized {
        self.id = Some(_id.to_string());
        self
    }

    fn remove_id(&mut self) {
        self.id = None;
    }

    fn get_link(&self) -> Option<&Link> {
        Option::from(&self.link)
    }

    fn link(mut self, _link: &str, _new_tab: bool) -> Self where Self: Sized {
        self.link = Some(Link::from(_link, _new_tab));
        self
    }

    fn remove_link(&mut self) {
        self.link = None;
    }

    fn get_clip(&self) -> bool { self.clip }

    fn clip(mut self) -> Self {
        self.clip = true;
        self
    }

    fn remove_clip(&mut self) {
        self.clip = false;
    }

    fn clip_path(mut self, element: &dyn Element) -> Self where Self: Sized {
        if let Some(id) = element.get_id() {
            self.clip_path = Some(id.to_string());
        }
        self
    }
}

impl Styled for Group {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Group {
    pub fn add<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }
}
