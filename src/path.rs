#![allow(non_snake_case)]
use crate::{Element, ExpandElement, ImplProperties, Link, Point, Properties, Transform};

pub struct Path {
    properties: Properties,
    d: Vec<PathCommand>,
}

impl_properties!(Path);

impl Element for Path {
    fn expand(&mut self) -> ExpandElement {
        todo!()
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
