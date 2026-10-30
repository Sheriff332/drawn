use crate::prelude::*;

pub trait ToPath: Element {
    fn to_path(self) -> Path;
}

impl ToPath for Path {
    fn to_path(self) -> Path {
        self
    }
}

impl ToPath for Rect {
    fn to_path(mut self) -> Path {
        let propties = self.get_properties_mut().clone();
        Path::default()
            //I need to access the fields in rect bruh...
        //TODO: make getters for all the primitive elements
    }
}