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
//! fn main() -> std::io::Result<()> {
//!     let mut new = Svg::default().size(100.0,100.0);
//!
//!     new.push(
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
//!
//! ## Debug
//!
//! `drawn` has support for debugging through the [`tracing`](https://crates.io/crates/tracing) crate.
//! `tracing` is added to the project by default; to enable logging use the
//! [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber)
//! crate:
//!
//! ```rust
//! use drawn::prelude::*;
//!
//! fn main() -> std::io::Result<()> {
//!     tracing_subscriber::fmt()
//!         .with_max_level(tracing::Level::DEBUG)
//!         .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
//!         .init();
//!
//!     let mut new = Svg::default().size(100.0,100.0);
//!     /*--snip--*/
//!     Ok(())
//! }
//! ```

#[macro_use]
pub mod macros;
pub mod group;
pub mod image;
pub mod optimizer;
pub mod path;
pub mod shapes;
pub mod text;

use std::fmt::{Display, Formatter};
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
    pub fn push<E: Element + 'static>(&mut self, element: E) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }

    pub fn write(&mut self) -> String {
        let mut out = String::new();
        out += "<svg";
        if let Some(viewbox) = self.viewbox {
            out += format!(
                r#" viewBox="{} {} {} {}""#,
                viewbox.0.x, viewbox.0.y, viewbox.1.x, viewbox.1.y
            )
            .as_str();
        } else {
            tracing::debug!("No viewbox set, using default")
        }
        if let Some(width) = self.width {
            out += format!(r#" width="{}""#, width).as_str();
        } else {
            tracing::debug!("No width set, using default")
        }
        if let Some(height) = self.height {
            out += format!(r#" height="{}""#, height).as_str();
        } else {
            tracing::debug!("No height set, using default")
        }
        out += concat!(r#" xmlns="http://www.w3.org/2000/svg">"#, "\n");
        if self.get_clip() {
            tracing::warn!("Using the Root SVG as a clip is not supported, skipping...");
            tracing::info!("Consider using a Group instead (Group::default().id(...).clip())");
        }
        if self.get_link().is_some() {
            tracing::warn!("Wrapping a link over the Root SVG is not supported, skipping...");
            tracing::info!("Consider using a Group instead (Group::default().link(...))");
        }
        let mut properties = String::new();
        self.properties
            .to_raw_svg(&mut properties, StyleTarget::All);
        if !properties.is_empty() {
            out += format!("<g {}>", properties).as_str();
        }
        for element in &mut self.elements {
            let has_clip = element.get_clip();
            let element = element.as_mut();
            let mut lines = Vec::new();
            unroll_into(element, &mut lines, has_clip);
            for line in lines {
                out += (line + "\n").as_str();
            }
        }
        if !properties.is_empty() {
            out += "</g>";
        }
        out += "</svg>";
        out
    }

    #[tracing::instrument(skip(self), err)]
    pub fn write_to_file(&mut self, filename: &str) -> io::Result<()> {
        fs::write(filename, self.write())
    }
}

impl_properties!(Svg);

impl Element for Svg {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children(std::mem::take(&mut self.elements))
    }
}

fn unroll_into(element: &mut dyn Element, out: &mut Vec<String>, check_clip: bool) -> bool {
    //return is link check
    let clip = element.get_clip();
    if check_clip {
        element.get_properties_mut().style = Style::default();
        element.get_properties_mut().link = None;
    }
    let linked = element.get_link().is_some();
    let mut child_linked = false;
    let mut child_lines = Vec::new();

    match element.expand() {
        ExpandElement::Children(vec) => {
            let style = &element.get_properties_mut().style.clone();
            for mut child_element in vec {
                if child_element.get_properties_mut().link.is_none() {
                    child_element.get_properties_mut().link =
                        element.get_properties().get_link().cloned();
                }
                if child_element.get_properties_mut().clip_path.is_none() {
                    child_element.get_properties_mut().clip_path =
                        element.get_properties().get_clip_path();
                }
                for t in element.get_transforms() {
                    child_element
                        .get_properties_mut()
                        .transforms
                        .insert(0, t.clone());
                }
                child_element.get_properties_mut().style.inherit_from(style);
                child_linked |=
                    unroll_into(&mut *child_element, &mut child_lines, check_clip || clip);
            }
        }
        ExpandElement::RawSvg(line) => {
            child_lines.push(line);
        }
    }

    if linked && !child_linked {
        out.push(format!(
            r#"<a href="{}"{}>"#,
            element.get_link().unwrap().link,
            if element.get_link().unwrap().new_tab {
                r#" target="_blank" rel="noopener""#
            } else {
                ""
            }
        ));
        out.extend(child_lines);
        out.push("</a>".to_string());
        true
    } else {
        if clip && let Some(id) = element.get_id() {
            out.push("<defs>".to_string());
            tracing::info!("Using ID for clipPath");
            out.push(format!(r#"<clipPath id="{}">"#, id));
        }
        if linked && child_linked {
            tracing::warn!("Element contains link while child element also contains link");
            tracing::warn!("Nested links are not allowed, skipping...");
        }
        out.extend(child_lines);
        if clip && element.get_id().is_some() {
            out.push("</clipPath>".to_string());
            out.push("</defs>".to_string());
        }
        child_linked
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
    fn to_raw_svg(&self, expr: &mut String, target: StyleTarget) {
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
        if let Some(fill) = self.style.fill {
            if [StyleTarget::Image, StyleTarget::Line, StyleTarget::Polyline].contains(&target) {
                tracing::warn!(
                    "Property fill cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" fill="{}""#, fill).as_str();
            }
        }

        if let Some(stroke) = self.style.stroke {
            if [StyleTarget::Image].contains(&target) {
                tracing::warn!(
                    "Property stroke cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" stroke="{}""#, stroke).as_str();
            }
        }

        if let Some(stroke_width) = self.style.stroke_width {
            if [StyleTarget::Image].contains(&target) {
                tracing::warn!(
                    "Property stroke-width cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" stroke-width="{}""#, stroke_width).as_str();
            }
        }

        if let Some(opacity) = self.style.opacity {
            *expr += format!(r#" opacity="{}""#, opacity).as_str();
        }

        if !self.style.stroke_dasharray.is_empty() {
            if [StyleTarget::Image].contains(&target) {
                tracing::warn!(
                    "Property stroke-dasharray cannot be used by the following element, skipping..."
                );
            } else {
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
        }

        if let Some(stroke_linecap) = &self.style.stroke_linecap {
            if [StyleTarget::Image].contains(&target) {
                tracing::warn!(
                    "Property stroke-linecap cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" stroke-linecap="{}""#, stroke_linecap).as_str();
            }
        }

        if let Some(stroke_linejoin) = &self.style.stroke_linejoin {
            if [StyleTarget::Image, StyleTarget::Line, StyleTarget::Ellipse].contains(&target) {
                tracing::warn!(
                    "Property stroke-linejoin cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" stroke-linejoin="{}""#, stroke_linejoin).as_str();
            }
        }

        if let Some(fill_opacity) = self.style.fill_opacity {
            if [StyleTarget::Image, StyleTarget::Line, StyleTarget::Polyline].contains(&target) {
                tracing::warn!(
                    "Property fill-opacity cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" fill-opacity="{}""#, fill_opacity).as_str();
            }
        }

        if let Some(stroke_opacity) = self.style.stroke_opacity {
            if [StyleTarget::Image].contains(&target) {
                tracing::warn!(
                    "Property stroke-opacity cannot be used by the following element, skipping..."
                );
            } else {
                *expr += format!(r#" stroke-opacity="{}""#, stroke_opacity).as_str();
            }
        }
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
    fn get_properties(&self) -> &Properties;
    fn get_properties_mut(&mut self) -> &mut Properties;
    fn remove_properties(mut self) -> Self
    where
        Self: Sized,
    {
        *self.get_properties_mut() = Properties::default();
        self
    }
    fn properties(mut self, properties: Properties) -> Self
    where
        Self: Sized,
    {
        *self.get_properties_mut() = properties;
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
    fn dasharray(mut self, sd: Vec<f32>) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_dasharray = sd;
        self
    }
    fn remove_dasharray(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_dasharray = Vec::new();
        self
    }
    fn linecap_butt(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linecap = Some(LineCap::Butt);
        self
    }
    fn linecap_round(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linecap = Some(LineCap::Round);
        self
    }
    fn linecap_square(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linecap = Some(LineCap::Square);
        self
    }
    fn remove_linecap(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linecap = None;
        self
    }
    fn linejoin_bevel(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linejoin = Some(LineJoin::Bevel);
        self
    }
    fn linejoin_miter(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linejoin = Some(LineJoin::Miter);
        self
    }
    fn linejoin_round(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_properties_mut().style.stroke_linejoin = Some(LineJoin::Round);
        self
    }
    fn remove_linejoin(mut self) -> Self
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
    fn get_clip_path(&self) -> Option<String> {
        None
    }
    fn clip_path(self, _element: &str) -> Self
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
    fn get_properties(&self) -> &Properties {
        self
    }
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
    none: bool,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
        none: false,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
        none: false,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
        none: false,
    };
    pub const NONE: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
        none: true,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
        none: false,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
        none: false,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
        none: false,
    };

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
            none: false,
        }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a,
            none: false,
        }
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
        if self.none {
            write!(f, "none")?;
        } else {
            write!(f, "{}", self.to_hex())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub fn from(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum LineCap {
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

pub(crate) type TMat = [f32; 6];
pub(crate) const ID_MAT: TMat = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
#[derive(Clone)]
pub enum Transform {
    Translate(Point),
    Scale(f32),
    SkewX(f32),
    SkewY(f32),
    Rotate(f32),
    Matrix(TMat),
}
impl Transform {
    pub(crate) fn to_matrix(&self) -> TMat {
        match self {
            Transform::Translate(translate) => [1.0, 0.0, 0.0, 1.0, translate.x, translate.y],
            Transform::Scale(scale) => [*scale, 0.0, 0.0, *scale, 0.0, 0.0],
            Transform::SkewX(skew_x) => {
                let r = skew_x.to_radians();
                [1.0, 0.0, r.tan(), 1.0, 0.0, 0.0]
            }
            Transform::SkewY(skew_y) => {
                let r = skew_y.to_radians();
                [1.0, r.tan(), 0.0, 1.0, 0.0, 0.0]
            }
            Transform::Rotate(rotate) => {
                let r = rotate.to_radians();
                [r.cos(), r.sin(), -r.sin(), r.cos(), 0.0, 0.0]
            }
            Transform::Matrix(matrix) => *matrix,
        }
    }
}
impl std::ops::Mul<TMat> for Transform {
    type Output = TMat;

    fn mul(self, rhs: TMat) -> Self::Output {
        let lhs = self.to_matrix();
        [
            (lhs[0] * rhs[0]) + (lhs[2] * rhs[1]),          // a
            (lhs[1] * rhs[0]) + (lhs[3] * rhs[1]),          // b
            (lhs[0] * rhs[2]) + (lhs[2] * rhs[3]),          // c
            (lhs[1] * rhs[2]) + (lhs[3] * rhs[3]),          // d
            (lhs[0] * rhs[4]) + (lhs[2] * rhs[5]) + lhs[4], // e
            (lhs[1] * rhs[4]) + (lhs[3] * rhs[5]) + lhs[5], // f
        ]
    }
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
    pub use path::*;
    pub use shapes::{circle::*, ellipse::*, line::*, polygon::*, polyline::*, rect::*};
    pub use text::*;
}
