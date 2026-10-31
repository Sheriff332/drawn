use crate::*;

#[derive(Default)]
pub struct Group {
    pub(crate) properties: Properties,
    pub(crate) elements: Vec<Box<dyn Element>>,
}

impl Element for Group {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children(std::mem::take(&mut self.elements))
    }
}

impl_properties!(Group);

impl Group {
    pub fn push<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }
}
