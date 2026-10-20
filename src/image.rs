use crate::{
    Element, ExpandElement, ImplProperties, Link, Point, Properties, Style, StyleTarget, Transform,
};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use std::fs;
use std::path::Path;

#[derive(Clone, Default)]
pub struct Image {
    properties: Properties,
    href: String,
    pos: Option<Point>,
    height: Option<f32>,
    width: Option<f32>,
}

impl_properties!(Image);

fn img_to_b64(path: &str) -> Result<String, std::io::Error> {
    let bytes = fs::read(path)?;
    let encoded = BASE64_STANDARD.encode(bytes);

    let ext = match Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "jpeg",
        "svg" => "svg+xml",
        "gif" => "gif",
        "webp" => "webp",
        _ => "png",
    };

    Ok(format!("data:image/{};base64,{}", ext, encoded))
}

impl Element for Image {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<image"#.to_string();
        if !self.href.is_empty()
            && let Ok(img) = img_to_b64(&self.href)
        {
            expr += format!(r#" href="{}""#, img).as_str();
        }
        if let Some(pos) = self.pos {
            expr += format!(r#" x="{}" y="{}""#, pos.x, pos.y).as_str();
        }
        if let Some(width) = self.width {
            expr += format!(r#" width="{}""#, width).as_str();
        }
        if let Some(height) = self.height {
            expr += format!(r#" height="{}""#, height).as_str();
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::Image);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl Image {
    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.pos = Some(Point::from(x, y));
        self
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    pub fn href(mut self, href: &str) -> Self {
        self.href = href.to_string();
        self
    }
}
