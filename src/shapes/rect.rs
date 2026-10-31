use crate::*;

#[derive(Clone, Default)]
pub struct Rect {
    pub(crate) properties: Properties,
    pub(crate) pos: Option<Point>,
    pub(crate) width: Option<f32>,
    pub(crate) height: Option<f32>,
    pub(crate) rx: Option<f32>,
    pub(crate) ry: Option<f32>,
}

impl Rect {
    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.pos = Some(Point::from(x, y));
        self
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    pub fn round(mut self, rx: f32, ry: f32) -> Self {
        self.rx = Some(rx);
        self.ry = Some(ry);
        self
    }
}
impl Element for Rect {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<rect"#.to_string();
        if let Some(pos) = self.pos {
            expr += format!(r#" x="{}" y="{}""#, pos.x, pos.y).as_str();
        }
        if let Some(rx) = self.rx {
            expr += format!(r#" rx="{}""#, rx).as_str();
        }
        if let Some(ry) = self.ry {
            expr += format!(r#" ry="{}""#, ry).as_str();
        }
        if let Some(width) = self.width {
            expr += format!(r#" width="{}""#, width).as_str();
        }
        if let Some(height) = self.height {
            expr += format!(r#" height="{}""#, height).as_str();
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::All);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl_properties!(Rect);
