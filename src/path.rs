#![allow(non_snake_case)]
use crate::{Element, ExpandElement, Link, Point, Style, Styled, Transform};

pub struct Path {
    id: Option<String>,
    link: Option<Link>,
    clip: bool,
    clip_path: Option<String>,
    transforms: Vec<Transform>,
    style: Style,
    d: Vec<PathCommand>,
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

    fn id(mut self, _id: &str) -> Self
    where
        Self: Sized,
    {
        self.id = Some(_id.to_string());
        self
    }

    fn remove_id(&mut self) {
        self.id = None;
    }

    fn get_link(&self) -> Option<&Link> {
        Option::from(&self.link)
    }

    fn link(mut self, _link: &str, _new_tab: bool) -> Self
    where
        Self: Sized,
    {
        self.link = Some(Link::from(_link, _new_tab));
        self
    }

    fn remove_link(&mut self) {
        self.link = None;
    }

    fn get_clip(&self) -> bool {
        self.clip
    }

    fn clip(mut self) -> Self {
        self.clip = true;
        self
    }

    fn remove_clip(&mut self) {
        self.clip = false;
    }

    fn clip_path(mut self, element: &dyn Element) -> Self
    where
        Self: Sized,
    {
        if let Some(id) = element.get_id() {
            self.clip_path = Some(id.to_string());
        }
        self
    }

    fn get_transforms(&self) -> &[Transform] {
        &self.transforms
    }

    fn transform(mut self, transform: Transform) -> Self {
        self.transforms.push(transform);
        self
    }

    fn remove_transforms(&mut self) {
        self.transforms = Vec::new();
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
    A((Point, f32, bool, bool, Point)),
    Ad((Point, f32, bool, bool, Point)),
}

impl Path {
    pub fn M(mut self, points: Vec<Point>) -> Self {
        self.d.push(PathCommand::M(points));
        self
    }
    pub fn m(mut self, points: Vec<Point>) -> Self {
        self.d.push(PathCommand::Md(points));
        self
    }
    pub fn L(mut self, points: Vec<Point>) -> Self {
        self.d.push(PathCommand::L(points));
        self
    }
    pub fn l(mut self, points: Vec<Point>) -> Self {
        self.d.push(PathCommand::Ld(points));
        self
    }
    pub fn H(mut self, points: Vec<f32>) -> Self {
        self.d.push(PathCommand::H(points));
        self
    }
    pub fn h(mut self, points: Vec<f32>) -> Self {
        self.d.push(PathCommand::Hd(points));
        self
    }
    pub fn V(mut self, points: Vec<f32>) -> Self {
        self.d.push(PathCommand::V(points));
        self
    }
    pub fn v(mut self, points: Vec<f32>) -> Self {
        self.d.push(PathCommand::Vd(points));
        self
    }
    pub fn z(mut self) -> Self {
        self.d.push(PathCommand::Z);
        self
    }
    pub fn C(mut self, points: (Point, Point, Point)) -> Self {
        self.d.push(PathCommand::C(points));
        self
    }
    pub fn c(mut self, points: (Point, Point, Point)) -> Self {
        self.d.push(PathCommand::Cd(points));
        self
    }
    pub fn S(mut self, points: (Point, Point)) -> Self {
        self.d.push(PathCommand::S(points));
        self
    }
    pub fn s(mut self, points: (Point, Point)) -> Self {
        self.d.push(PathCommand::Sd(points));
        self
    }
    pub fn T(mut self, point: Point) -> Self {
        self.d.push(PathCommand::T(point));
        self
    }
    pub fn t(mut self, point: Point) -> Self {
        self.d.push(PathCommand::Td(point));
        self
    }
    pub fn A(mut self, points: (Point, f32, bool, bool, Point)) -> Self {
        self.d.push(PathCommand::A(points));
        self
    }
    pub fn a(mut self, points: (Point, f32, bool, bool, Point)) -> Self {
        self.d.push(PathCommand::Ad(points));
        self
    }
}
