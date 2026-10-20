//! `drawn` is a lightweight, easy-to-use interface for building SVGs
//! and performing unique operations on in-memory SVGs.
//!
//! Despite originally being made to create SVG-only websites,
//! it has many generic SVG methods and types suitable even for power users.
//!
//! It prioritizes making highly optimized SVGs in terms of speed and file size,
//! drawing parallels with tools like SVGO.
//! Additionally, it's power lies in custom methods and transformations,
//! alongside user defined Elements and a powerful theming system.
//!
//! ## Specification & Philosophy
//!
//! `drawn` targets a curated, high-performance subset of the **W3C SVG 2**
//! specification (with universal backward compatibility for **SVG 1.1**).
//!
//! ### Why doesn't `drawn` implement all SVG elements and properties?
//!
//! The complete SVG specification is over 800 pages long and includes decades of
//! legacy web baggage. `drawn` is designed from the ground up for high-framerate
//! interactive web canvases, generative graphics, and maximum extensibility.
//!
//! Thus, drawn does not implement methods like:
//!
//! - `<filter>`, `<mask>`: Very slow and heavy on the CPU
//!
//! - `<animate>`: No longer in use, and drawn makes static SVG's
//!
//! - `<textPath>`, `<foreignObject>`: Buggy and inconsistent behavior, rarely used
//!
//! - `stroke-miterlimit`, `stroke-dashoffset` etc.:
//!   Cases where the usage is as rare as these are omitted to keep the crate lightweight
//!
//! - No CSS support: CSS has inconsistent behavior depending on the platform
//!
//! If you want to use any such methods yourself which are not available here,
//! feel free to look at some more well-established alternatives like:
//!
//! [`svg`](https://crates.io/crates/svg)
//! : traditional 1:1 XML DOM builder
//!
//! [`usvg`](https://crates.io/crates/usvg) / [`resvg`](https://crates.io/crates/resvg)
//! : for parsing / rasterizing SVG's
//!
//! [`kurbo`](https://crates.io/crates/kurbo)
//! : dedicated 2D geometry math library
//!
//! ## Quick Start
//!
//! ```rust
//! use drawn::prelude::*;
//!
//! fn main() -> Result<(), std::io::Error> {
//!     let mut new = Svg::default().size(100.0,100.0);
//!
//!     new.add(
//!         Rect::default()
//!             .size(50.0, 50.0)
//!             .round(5.0, 5.0)
//!             .stroke_width(0.5)
//!             .at(20.0, 20.0)
//!             .fill(Color::RED)
//!             .id("hi-there"),
//!     );
//!
//!     new.write_to_file("new.svg")?;
//!     Ok(())
//! }
//! ```

#[macro_use]
pub mod macros;
pub mod group;
pub mod image;
pub mod path;
pub mod shapes;
pub mod text;

use std::fmt::{Display, Formatter};
use std::io::Write;
use std::{fs, io};

#[derive(Default)]
pub struct Svg {
    properties: Properties,
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
            let clip = element.get_clip() && element.get_id().is_some();
            if clip {
                unroll_and_erase(element);
            }
            let link_inside = if clip {
                false
            } else {
                unroll_link_check(element)
            };
            let linked = element.get_link().is_some() && !link_inside;
            if linked {
                writeln!(
                    &mut writer,
                    r#"<a href="{}"{}>"#,
                    element.get_link().unwrap().link,
                    if element.get_link().unwrap().new_tab {
                        r#" target="_blank" rel="noopener""#
                    } else {
                        ""
                    }
                )?;
            }
            if clip && let Some(id) = element.get_id() {
                writeln!(&mut writer, "<defs>")?;
                writeln!(&mut writer, r#"<clipPath id="{}">"#, id)?;
            }
            let mut lines = Vec::new();
            unroll_into(element, &mut lines, link_inside);
            for line in lines {
                writeln!(writer, "{}", line)?;
            }
            if clip {
                writeln!(&mut writer, "</clipPath>")?;
                writeln!(&mut writer, "</defs>")?;
            }
            if linked {
                writeln!(&mut writer, "</a>")?;
            }
        }
        writeln!(writer, "</svg>")?;
        Ok(())
    }
}

impl_properties!(Svg);

impl Element for Svg {
    fn expand(&mut self) -> ExpandElement {
        let mut out = String::new();
        out += "<svg";
        if let Some(viewbox) = self.viewbox {
            out += format!(
                r#" viewBox="{} {} {} {}""#,
                viewbox.0.x, viewbox.0.y, viewbox.0.x, viewbox.0.y
            )
            .as_str();
        }
        if let Some(width) = self.width {
            out += format!(r#" width="{}""#, width).as_str();
        }
        if let Some(height) = self.height {
            out += format!(r#" height="{}""#, height).as_str();
        }
        out += r#" xmlns="http://www.w3.org/2000/svg">"#;
        for element in &mut self.elements {
            let clip = element.get_clip() && element.get_id().is_some();
            if clip {
                unroll_and_erase(element);
            }
            let link_inside = if clip {
                false
            } else {
                unroll_link_check(element)
            };
            let linked = element.get_link().is_some() && !link_inside;
            if linked {
                out += format!(
                    r#"<a href="{}"{}>"#,
                    element.get_link().unwrap().link,
                    if element.get_link().unwrap().new_tab {
                        r#" target="_blank" rel="noopener""#
                    } else {
                        ""
                    }
                )
                .as_str();
            }
            if clip && let Some(id) = element.get_id() {
                out += "<defs>";
                out += format!(r#"<clipPath id="{}">"#, id).as_str();
            }
            let mut lines = Vec::new();
            unroll_into(element, &mut lines, link_inside);
            for line in lines {
                out += line.as_str();
            }
            if clip {
                out += "</clipPath>";
                out += "</defs>";
            }
            if linked {
                out += "</a>";
            }
        }
        out += "</svg>";
        ExpandElement::RawSvg(out)
    }
}

fn unroll_into(element: &mut Box<dyn Element>, out: &mut Vec<String>, check: bool) {
    let link_inside;
    if check {
        link_inside = unroll_link_check(element);
        if element.get_link().is_some() && !link_inside {
            out.push(format!(
                r#"<a href="{}"{}>"#,
                element.get_link().unwrap().link,
                if element.get_link().unwrap().new_tab {
                    r#" target="_blank" rel="noopener""#
                } else {
                    ""
                }
            ));
        }
    } else {
        link_inside = false;
    }
    match element.expand() {
        ExpandElement::Children(vec) => {
            let pstyle = &element.get_properties_mut().style;
            for mut element in vec {
                element.get_properties_mut().style.inherit_from(pstyle);
                unroll_into(&mut element, out, link_inside);
            }
        }
        ExpandElement::RawSvg(str) => {
            out.push(str);
        }
    }
    if element.get_link().is_some() && !link_inside {
        out.push("</a>".to_string());
    }
}

fn unroll_link_check(element: &mut Box<dyn Element>) -> bool {
    if let ExpandElement::Children(vec) = element.expand() {
        for mut element in vec {
            if element.get_link().is_some() || unroll_link_check(&mut element) {
                return true;
            }
        }
    }
    false
}

fn unroll_and_erase(element: &mut Box<dyn Element>) {
    if let ExpandElement::Children(vec) = element.expand() {
        for mut element in vec {
            *element.get_properties_mut() = Properties::default();
            unroll_and_erase(&mut element);
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
pub(crate) enum StyleTarget {
    All,
    Image,
    Line,
    Ellipse,
    Polyline,
}

impl Style {
    pub fn inherit_from(&mut self, parent: &Style) {
        // Only inherit if the child's field is None!
        if self.fill.is_none() {
            self.fill = parent.fill;
        }
        if self.stroke.is_none() {
            self.stroke = parent.stroke;
        }
        if self.stroke_width.is_none() {
            self.stroke_width = parent.stroke_width;
        }
        if self.stroke_dasharray.is_empty() {
            self.stroke_dasharray = parent.stroke_dasharray.clone();
        }
        if self.stroke_linecap.is_none() {
            self.stroke_linecap = parent.stroke_linecap;
        }
        if self.stroke_linejoin.is_none() {
            self.stroke_linejoin = parent.stroke_linejoin;
        }

        // Opacities multiply
        self.opacity = match (self.opacity, parent.opacity) {
            (Some(c), Some(p)) => Some(c * p),
            (None, Some(p)) => Some(p),
            (Some(c), None) => Some(c),
            (None, None) => None,
        };
    }
}

impl Properties {
    fn to_raw_svg<'a>(&self, expr: &'a mut String, target: StyleTarget) -> &'a mut String {
        if !self.clip
            && let Some(id) = &self.id
        {
            *expr += format!(r#" id="{}""#, id).as_str();
        }
        if let Some(clip_path) = &self.clip_path {
            *expr += format!(r#" clip-path="url(#{})""#, clip_path).as_str();
        }
        if !self.transforms.is_empty() {
            *expr += r#" transform=""#;
            for t in &self.transforms {
                *expr += format!(r#"{} "#, t).as_str();
            }
            *expr += r#"""#;
        }
        if (![StyleTarget::Image, StyleTarget::Line, StyleTarget::Polyline].contains(&target))
            && let Some(fill) = self.style.fill
        {
            *expr += format!(r#" fill="{}""#, fill).as_str();
        }

        if (![StyleTarget::Image].contains(&target))
            && let Some(stroke) = self.style.stroke
        {
            *expr += format!(r#" stroke="{}""#, stroke).as_str();
        }

        if (![StyleTarget::Image].contains(&target))
            && let Some(stroke_width) = self.style.stroke_width
        {
            *expr += format!(r#" stroke-width="{}""#, stroke_width).as_str();
        }

        if let Some(opacity) = self.style.opacity {
            *expr += format!(r#" opacity="{}""#, opacity).as_str();
        }

        if (![StyleTarget::Image].contains(&target)) && !self.style.stroke_dasharray.is_empty() {
            *expr += format!(
                r#" stroke-dasharray="{}""#,
                self.style
                    .stroke_dasharray
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .as_str();
        }

        if (![StyleTarget::Image].contains(&target))
            && let Some(stroke_linecap) = &self.style.stroke_linecap
        {
            *expr += format!(r#" stroke-linecap="{}""#, stroke_linecap).as_str();
        }

        if (![StyleTarget::Image, StyleTarget::Line, StyleTarget::Ellipse].contains(&target))
            && let Some(stroke_linejoin) = &self.style.stroke_linejoin
        {
            *expr += format!(r#" stroke-linejoin="{}""#, stroke_linejoin).as_str();
        }

        if (![StyleTarget::Image, StyleTarget::Line, StyleTarget::Polyline].contains(&target))
            && let Some(fill_opacity) = self.style.fill_opacity
        {
            *expr += format!(r#" fill-opacity="{}""#, fill_opacity).as_str();
        }

        if (![StyleTarget::Image].contains(&target))
            && let Some(stroke_opacity) = self.style.stroke_opacity
        {
            *expr += format!(r#" stroke-opacity="{}""#, stroke_opacity).as_str();
        }

        expr
    }
}

#[derive(Clone, Default)]
pub struct Properties {
    pub id: Option<String>,
    pub link: Option<Link>,
    pub clip: bool,
    pub clip_path: Option<String>,
    pub transforms: Vec<Transform>,
    pub style: Style,
}

pub trait ImplProperties {
    fn get_properties_mut(&mut self) -> &mut Properties;
    fn remove_properties(mut self) -> Self
    where
        Self: Sized,
    {
        *self.get_properties_mut() = Properties::default();
        self
    }
    fn remove_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style = Style::default();
        self
    }
    fn fill(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.fill = Some(color);
        self
    }
    fn remove_fill(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.fill = None;
        self
    }
    fn stroke(mut self, stroke: Color) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke = Some(stroke);
        self
    }
    fn remove_stroke(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke = None;
        self
    }
    fn stroke_width(mut self, stroke_width: f32) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_width = Some(stroke_width);
        self
    }
    fn remove_stroke_width(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_width = None;
        self
    }
    fn opacity(mut self, opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.opacity = Some(opacity);
        self
    }
    fn remove_opacity(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.opacity = None;
        self
    }
    fn stroke_dasharray(mut self, sd: Vec<f32>) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_dasharray = sd;
        self
    }
    fn remove_stroke_dasharray(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_dasharray = Vec::new();
        self
    }
    fn stroke_linecap(mut self, lc: LineCap) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linecap = Some(lc);
        self
    }
    fn remove_stroke_linecap(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linecap = None;
        self
    }
    fn stroke_linejoin(mut self, line_join: LineJoin) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linejoin = Some(line_join);
        self
    }
    fn remove_stroke_linejoin(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linejoin = None;
        self
    }
    fn fill_opacity(mut self, fill_opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.fill_opacity = Some(fill_opacity);
        self
    }
    fn remove_fill_opacity(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.fill_opacity = None;
        self
    }
    fn stroke_opacity(mut self, stroke_opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_opacity = Some(stroke_opacity);
        self
    }
    fn remove_stroke_opacity(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_opacity = None;
        self
    }
    fn get_id(&self) -> Option<&str> {
        None
    }
    fn id(self, _id: &str) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn remove_id(&mut self) {}
    fn get_link(&self) -> Option<&Link> {
        None
    }
    fn link(self, _link: &str, _new_tab: bool) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn remove_link(&mut self) {}
    fn get_clip(&self) -> bool {
        false
    }
    fn clip(self) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn remove_clip(&mut self) {}
    fn clip_path(self, _element: &dyn Element) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn get_transforms(&self) -> &[Transform] {
        &[]
    }
    fn transform(self, _transform: Transform) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn remove_transforms(&mut self) {}
}

impl ImplProperties for Properties {
    fn get_properties_mut(&mut self) -> &mut Properties {
        self
    }
}

pub trait Element: ImplProperties {
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Default, Clone)]
pub struct Link {
    link: String,
    new_tab: bool,
}

impl Link {
    pub fn from(link: &str, new_tab: bool) -> Link {
        Link {
            link: link.to_string(),
            new_tab,
        }
    }
}

#[derive(Clone)]
pub enum Transform {
    Translate(Point),
    Scale(f32),
    SkewX(f32),
    SkewY(f32),
    Rotate(f32),
    Matrix([f32; 6]),
}
impl Display for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Transform::Translate(pt) => format!("translate({}, {})", pt.x, pt.y),
                Transform::Scale(v) => format!("scale({})", v),
                Transform::SkewX(v) => format!("skewX({})", v),
                Transform::SkewY(v) => format!("skewY({})", v),
                Transform::Rotate(v) => format!("rotate({})", v),
                Transform::Matrix(m) => format!(
                    "matrix({},{},{},{},{},{})",
                    m[0], m[1], m[2], m[3], m[4], m[5]
                ),
            }
        )?;
        Ok(())
    }
}

pub mod prelude {
    pub use crate::*;
    pub use group::*;
    pub use image::*;
    pub use text::*;
    pub use path::*;
    pub use shapes::{rect::*, circle::*, ellipse::*, line::*, polygon::*, polyline::*};
}
