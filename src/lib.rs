use std::fmt::{Display, Formatter};
use std::io::Write;
use std::{fs, io};

pub mod group;
pub mod shapes;

#[derive(Default)]
pub struct Svg {
    width: Option<f32>,
    height: Option<f32>,
    viewbox: Option<(Point, Point)>,
    elements: Vec<Box<dyn Element>>,
}

impl Svg {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    pub fn add<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }

    pub fn write_to_file(&mut self, filename: &str) -> Result<(), io::Error> {
        let mut writer = io::BufWriter::new(fs::File::create(filename)?);
        write!(&mut writer, "<svg")?;
        if let Some(viewbox) = self.viewbox {
            write!(
                &mut writer,
                r#" viewBox="{} {} {} {}""#,
                viewbox.0.x, viewbox.0.y, viewbox.0.x, viewbox.0.y
            )?;
        }
        if let Some(width) = self.width {
            write!(&mut writer, r#" width="{}""#, width)?;
        }
        if let Some(height) = self.height {
            write!(&mut writer, r#" height="{}""#, height)?;
        }
        writeln!(&mut writer, r#" xmlns="http://www.w3.org/2000/svg">"#)?;
        for element in &mut self.elements {
            let mut lines = Vec::new();
            unroll(element, &mut lines);
            for line in lines {
                writeln!(writer, "{}", line)?;
            }
        }
        writeln!(writer, "</svg>")?;
        Ok(())
    }
}

fn unroll(element: &mut Box<dyn Element>, out: &mut Vec<String>) {
    match element.expand() {
        ExpandElement::Children(vec) => {
            for mut element in vec {
                unroll(&mut element, out);
            }
        }
        ExpandElement::RawSvg(str) => {
            out.push(str);
        }
    }
}

#[derive(Clone, Default)]
pub struct Style {
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: Option<f32>,
    opacity: Option<f32>,
    stroke_dasharray: Vec<f32>,
    stroke_linecap: Option<LineCap>,
    stroke_linejoin: Option<LineJoin>,
    fill_opacity: Option<f32>,
    stroke_opacity: Option<f32>,
}

#[derive(PartialEq)]
pub enum StyleTarget {
    All,
    Image,
    Line,
    Ellipse,
}

impl Style {
    pub fn to_raw_svg<'a>(&self, expr: &'a mut String, target: StyleTarget) -> &'a mut String {
        if (![StyleTarget::Image, StyleTarget::Line].contains(&target)) && let Some(fill) = self.fill {
            *expr += format!(r#" fill="{}""#, fill).as_str();
        }

        if (![StyleTarget::Image].contains(&target)) && let Some(stroke) = self.stroke {
            *expr += format!(r#" stroke="{}""#, stroke).as_str();
        }

        if (![StyleTarget::Image].contains(&target)) && let Some(stroke_width) = self.stroke_width {
            *expr += format!(r#" stroke-width="{}""#, stroke_width).as_str();
        }

        if let Some(opacity) = self.opacity {
            *expr += format!(r#" opacity="{}""#, opacity).as_str();
        }

        if (![StyleTarget::Image].contains(&target)) && !self.stroke_dasharray.is_empty() {
            *expr += format!(
                r#" stroke-dasharray="{}""#,
                self.stroke_dasharray
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .as_str();
        }

        if (![StyleTarget::Image].contains(&target)) && let Some(stroke_linecap) = &self.stroke_linecap {
            *expr += format!(r#" stroke-linecap="{}""#, stroke_linecap).as_str();
        }

        if (![StyleTarget::Image, StyleTarget::Line, StyleTarget::Ellipse].contains(&target)) && let Some(stroke_linejoin) = &self.stroke_linejoin {
            *expr += format!(r#" stroke-linejoin="{}""#, stroke_linejoin).as_str();
        }

        if (![StyleTarget::Image, StyleTarget::Line].contains(&target)) && let Some(fill_opacity) = self.fill_opacity {
            *expr += format!(r#" fill-opacity="{}""#, fill_opacity).as_str();
        }

        if (![StyleTarget::Image].contains(&target)) && let Some(stroke_opacity) = self.stroke_opacity {
            *expr += format!(r#" stroke-opacity="{}""#, stroke_opacity).as_str();
        }

        expr
    }
}
pub trait Styled {
    fn get_style(&mut self) -> &mut Style;

    fn fill(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.get_style().fill = Some(color);
        self
    }
    fn stroke(mut self, stroke: Color) -> Self
    where
        Self: Sized,
    {
        self.get_style().stroke = Some(stroke);
        self
    }
    fn stroke_width(mut self, stroke_width: f32) -> Self
    where
        Self: Sized,
    {
        self.get_style().stroke_width = Some(stroke_width);
        self
    }
    fn opacity(mut self, opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.get_style().opacity = Some(opacity);
        self
    }
    fn stroke_dasharray(mut self, sd: Vec<f32>) -> Self
    where
        Self: Sized,
    {
        self.get_style().stroke_dasharray = sd;
        self
    }
    fn stroke_linecap(mut self, lc: LineCap) -> Self
    where
        Self: Sized,
    {
        self.get_style().stroke_linecap = Some(lc);
        self
    }
    fn stroke_linejoin(mut self, lj: LineJoin) -> Self
    where
        Self: Sized,
    {
        self.get_style().stroke_linejoin = Some(lj);
        self
    }
    fn fill_opacity(mut self, fill_opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.get_style().fill_opacity = Some(fill_opacity);
        self
    }
    fn stroke_opacity(mut self, stroke_opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.get_style().stroke_opacity = Some(stroke_opacity);
        self
    }
}

impl Styled for Style {
    fn get_style(&mut self) -> &mut Style {
        self
    }
}

pub trait Element: Styled {
    fn expand(&mut self) -> ExpandElement;
}

pub enum ExpandElement {
    Children(Vec<Box<dyn Element>>),
    RawSvg(String),
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

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

#[derive(Clone)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}
impl Display for LineCap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LineCap::Butt => "Butt",
                LineCap::Round => "Round",
                LineCap::Square => "Square",
            }
        )
    }
}

#[derive(Clone)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}
impl Display for LineJoin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LineJoin::Miter => "Miter",
                LineJoin::Round => "Round",
                LineJoin::Bevel => "Bevel",
            }
        )
    }
}

pub mod prelude {
    pub use crate::*;
    pub use group::*;
    pub use shapes::rect::*;
}
