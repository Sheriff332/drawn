use crate::{Element, ExpandElement, Link, Point, Style, Styled};

pub struct Path {
    id: Option<String>,
    link: Option<Link>,
    clip: bool,
    clip_path: Option<String>,
    style: Style,
    d: Vec<PathCommand>
}

impl Styled for Path {
    fn get_style_mut(&mut self) -> &mut Style {
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