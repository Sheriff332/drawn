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
        Path::default()
            .properties(self.get_properties_mut().clone())
            .M(&[self.pos.unwrap_or(Point::from(0.0, 0.0))])
    }
}
