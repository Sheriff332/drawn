use crate::*;

#[derive(Default)]
pub struct Group {
    id: Option<String>,
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
