use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};

pub struct Svg {
    width: f32,
    height: f32,
    viewbox: (Point, Point),
    elements: Vec<Box<dyn Element>>,
}

impl Svg {
    pub fn new() -> Self {
        Self {
            width: 1000.0,
            height: 1000.0,
            viewbox: (Point::from(0.0, 0.0), Point::from(1000.0, 1000.0)),
            elements: Vec::new()
        }
    }
    pub fn size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn add<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut writer = io::BufWriter::new(fs::File::create(filename)?);
        writeln!(writer,
                 r#"<svg viewBox="{} {} {} {}" width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
                 self.viewbox.0.x, self.viewbox.0.y, self.viewbox.1.x, self.viewbox.1.y,
                 self.height, self.width
        )?;
        for element in &self.elements {
            let mut lines = Vec::new();
            unroll(&element, &mut lines);
            for line in lines {
                writeln!(writer, "{}", line)?;
            }
        }
        writeln!(writer, "</svg>")?;
        Ok(())
    }
}

fn unroll(element: &Box<dyn Element>, out:&mut Vec<String>) {
    match element.expand() {
        ExpandElement::Children(vec) => {
            for element in vec {
                unroll(&element, out);
            }
        }
        ExpandElement::RawSvg(str) => {
            out.push(str);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Style {
    pub fill: Color,
    pub stroke: Color,
    pub stroke_width: f32,
    pub opacity: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            fill: Color::rgba(0, 0, 0, 0),
            stroke: Color::rgb(0, 0, 0),
            stroke_width: 2.0,
            opacity: 100.0,
        }
    }
}

pub trait Element {
    fn expand(&self) -> ExpandElement;
}

pub enum ExpandElement {
    Children(Vec<Box<dyn Element>>),
    RawSvg(String),
}

// pub struct Text {
//     pub style: Style,
//     pub text: String,
//     pub font: Option<String>,
//     pub font_size: f32,
//     pub text_anchor: String,//might add enum
// }

#[derive(Clone, Copy)]
pub struct Rect {
    pub style: Style,
    pub pos: Point,
    pub width: f32,
    pub height: f32,
    pub rx: f32,
    pub ry: f32,
}
impl Rect {
    pub fn new() -> Self {
        Self {
            style: Style::new(),
            pos: Point::from(0.0,0.0),
            width: 0.0,
            height: 0.0,
            rx: 0.0,
            ry: 0.0,
        }
    }
    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.pos = Point::from(x, y);
        self
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn round(mut self, rx: f32, ry: f32) -> Self {
        self.rx = rx;
        self.ry = ry;
        self
    }
}
impl Element for Rect {
    fn expand(&self) -> ExpandElement {
        let expr =
            format!(r#"<rect x="{}" y="{}" rx="{}" ry="{}" width="{}" height="{}" fill="{}" stroke = "{}" stroke-width = "{}" opacity = "{}%" />"#,
            self.pos.x, self.pos.y, self.rx, self.ry, self.width, self.height,
            self.style.fill, self.style.stroke, self.style.stroke_width, self.style.opacity);
        ExpandElement::RawSvg(expr)
    }
}

pub struct Circle {
    pub style: Style,
    pub center: Point,
    pub radius: f32,
}

pub struct Line {
    pub style: Style,
    pub start: Point,
    pub end: Point,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::rgb(r, g, b))
        } else {
            None
        }
    }
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn from(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub mod prelude {
    pub use crate::*;
}