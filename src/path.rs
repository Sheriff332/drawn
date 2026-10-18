use crate::{Element, ExpandElement, Link, Point, Style, Styled};

pub struct Path {
    id: Option<String>,
    link: Option<Link>,
    style: Style,
    d: Vec<PathCommand>
}

impl Styled for Path {
    fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Element for Path {
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

pub enum PathCommand {
    M(Vec<Point>),
    Md(Vec<Point>),
    L(Vec<Point>),
    Ld(Vec<Point>),
    H(Vec<f32>),
    Hd(Vec<f32>),
    V(Vec<f32>),
    Vd(Vec<f32>),
    Z,
    C((Point, Point, Point)),
    Cd((Point, Point, Point)),
    S((Point, Point)),
    Sd((Point, Point)),
    T(Point),
    Td(Point),
    A((Point, f32, f32, f32, Point)),
    Ad((Point, f32, f32, f32, Point)),
}