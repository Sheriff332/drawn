use crate::*;

pub struct Group {
    style: Style,
    elements: Vec<Box<dyn Element>>,
}

impl Element for Group {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children({
            let mut out = std::mem::take(&mut self.elements);
            for element in &mut out {
                *element.get_style() = self.style;
            }
            out
        })
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            style: Style::default(),
            elements: Vec::new(),
        }
    }
}

impl Styled for Group {
    fn get_style(&mut self) -> &mut Style { &mut self.style }
}

impl Group {
    pub fn add<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }
}