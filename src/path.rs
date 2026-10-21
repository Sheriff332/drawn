#![allow(non_snake_case)]

use crate::{
    Element, ExpandElement, ImplProperties, Link, Point, Properties, StyleTarget, Transform,
};
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct Path {
    properties: Properties,
    d: Vec<PathCommand>,
}

impl Path {
    pub fn from(d: Vec<PathCommand>) -> Self {
        Self {
            properties: Default::default(),
            d,
        }
    }
    pub fn M(mut self, points: &[Point]) -> Self {
        self.d.push(PathCommand::M(Vec::from(points)));
        self
    }
    pub fn m(mut self, points: &[Point]) -> Self {
        self.d.push(PathCommand::Md(Vec::from(points)));
        self
    }
    pub fn L(mut self, points: &[Point]) -> Self {
        self.d.push(PathCommand::L(Vec::from(points)));
        self
    }
    pub fn l(mut self, points: &[Point]) -> Self {
        self.d.push(PathCommand::Ld(Vec::from(points)));
        self
    }
    pub fn H(mut self, points: &[f32]) -> Self {
        self.d.push(PathCommand::H(Vec::from(points)));
        self
    }
    pub fn h(mut self, points: &[f32]) -> Self {
        self.d.push(PathCommand::Hd(Vec::from(points)));
        self
    }
    pub fn V(mut self, points: &[f32]) -> Self {
        self.d.push(PathCommand::V(Vec::from(points)));
        self
    }
    pub fn v(mut self, points: &[f32]) -> Self {
        self.d.push(PathCommand::Vd(Vec::from(points)));
        self
    }
    pub fn z(mut self) -> Self {
        self.d.push(PathCommand::Z);
        self
    }
    pub fn C(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.d.push(PathCommand::C((
            Point::from(x1, y1),
            Point::from(x2, y2),
            Point::from(x, y),
        )));
        self
    }
    pub fn c(mut self, dx1: f32, dy1: f32, dx2: f32, dy2: f32, x: f32, y: f32) -> Self {
        self.d.push(PathCommand::Cd((
            Point::from(dx1, dy1),
            Point::from(dx2, dy2),
            Point::from(x, y),
        )));
        self
    }
    pub fn S(mut self, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.d
            .push(PathCommand::S((Point::from(x2, y2), Point::from(x, y))));
        self
    }
    pub fn s(mut self, dx2: f32, dy2: f32, x: f32, y: f32) -> Self {
        self.d
            .push(PathCommand::Sd((Point::from(dx2, dy2), Point::from(x, y))));
        self
    }
    pub fn Q(mut self, x1: f32, y1: f32, x: f32, y: f32) -> Self {
        self.d
            .push(PathCommand::Q((Point::from(x1, y1), Point::from(x, y))));
        self
    }
    pub fn q(mut self, dx1: f32, dy1: f32, x: f32, y: f32) -> Self {
        self.d
            .push(PathCommand::Qd((Point::from(dx1, dy1), Point::from(x, y))));
        self
    }
    pub fn T(mut self, x: f32, y: f32) -> Self {
        self.d.push(PathCommand::T(Point::from(x, y)));
        self
    }
    pub fn t(mut self, x: f32, y: f32) -> Self {
        self.d.push(PathCommand::Td(Point::from(x, y)));
        self
    }
    #[allow(clippy::too_many_arguments)]
    pub fn A(
        mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
        x: f32,
        y: f32,
    ) -> Self {
        self.d.push(PathCommand::A((
            Point::from(rx, ry),
            x_axis_rotation,
            large_arc_flag,
            sweep_flag,
            Point::from(x, y),
        )));
        self
    }
    #[allow(clippy::too_many_arguments)]
    pub fn a(
        mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
        dx: f32,
        dy: f32,
    ) -> Self {
        self.d.push(PathCommand::Ad((
            Point::from(rx, ry),
            x_axis_rotation,
            large_arc_flag,
            sweep_flag,
            Point::from(dx, dy),
        )));
        self
    }
}

impl Element for Path {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<path"#.to_string();
        if !self.d.is_empty() {
            expr += r#" d=""#;
            expr += &self
                .d
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<_>>()
                .join(" ");
            expr += r#"""#;
        }
        expr += r#" fill="none""#;
        self.properties.to_raw_svg(&mut expr, StyleTarget::Polyline);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl_properties!(Path);

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
    Q((Point, Point)),
    Qd((Point, Point)),
    T(Point),
    Td(Point),
    A((Point, f32, bool, bool, Point)),
    Ad((Point, f32, bool, bool, Point)),
}
impl Display for PathCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathCommand::M(q) => {
                write!(
                    f,
                    "M{}",
                    q.iter()
                        .map(|p| format!("{} {}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::Md(q) => {
                write!(
                    f,
                    "m{}",
                    q.iter()
                        .map(|p| format!("{} {}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::L(q) => {
                write!(
                    f,
                    "L{}",
                    q.iter()
                        .map(|p| format!("{} {}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::Ld(q) => {
                write!(
                    f,
                    "l{}",
                    q.iter()
                        .map(|p| format!("{} {}", p.x, p.y))
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::H(q) => {
                write!(
                    f,
                    "H{}",
                    q.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::Hd(q) => {
                write!(
                    f,
                    "h{}",
                    q.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::V(q) => {
                write!(
                    f,
                    "V{}",
                    q.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::Vd(q) => {
                write!(
                    f,
                    "v{}",
                    q.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
            PathCommand::Z => write!(f, "z")?,
            PathCommand::C(q) => {
                write!(
                    f,
                    "C{} {}, {} {}, {} {}",
                    q.0.x, q.0.y, q.1.x, q.1.y, q.2.x, q.2.y
                )?;
            }
            PathCommand::Cd(q) => {
                write!(
                    f,
                    "c{} {}, {} {}, {} {}",
                    q.0.x, q.0.y, q.1.x, q.1.y, q.2.x, q.2.y
                )?;
            }
            PathCommand::S(q) => {
                write!(f, "S{} {}, {} {}", q.0.x, q.0.y, q.1.x, q.1.y)?;
            }
            PathCommand::Sd(q) => {
                write!(f, "s{} {}, {} {}", q.0.x, q.0.y, q.1.x, q.1.y)?;
            }
            PathCommand::Q(q) => {
                write!(f, "Q{} {}, {} {}", q.0.x, q.0.y, q.1.x, q.1.y)?;
            }
            PathCommand::Qd(q) => {
                write!(f, "q{} {}, {} {}", q.0.x, q.0.y, q.1.x, q.1.y)?;
            }
            PathCommand::T(q) => write!(f, "T{} {}", q.x, q.y)?,
            PathCommand::Td(q) => write!(f, "t{} {}", q.x, q.y)?,
            PathCommand::A(q) => {
                write!(
                    f,
                    "A{} {} {} {} {} {} {}",
                    q.0.x, q.0.y, q.1, q.2 as u8, q.3 as u8, q.4.x, q.4.y
                )?;
            }
            PathCommand::Ad(q) => {
                write!(
                    f,
                    "a{} {} {} {} {} {} {}",
                    q.0.x, q.0.y, q.1, q.2 as u8, q.3 as u8, q.4.x, q.4.y
                )?;
            }
        }
        Ok(())
    }
}
