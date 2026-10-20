use crate::*;

#[derive(Default)]
pub struct Group {
    properties: Properties,
    elements: Vec<Box<dyn Element>>,
}

impl Element for Group {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children({
            let mut out = std::mem::take(&mut self.elements);
            for element in &mut out {
                element.get_properties_mut().style = self.properties.style.clone();
            }
            out
        })
    }
}

impl_properties!(Group);

impl Group {
    pub fn add<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }
}
