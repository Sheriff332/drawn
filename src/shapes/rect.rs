use crate::*;

#[derive(Clone, Copy)]
pub struct Rect {
    style: Style,
    pos: Option<Point>,
    width: Option<f32>,
    height: Option<f32>,
    rx: Option<f32>,
    ry: Option<f32>,
}
impl Default for Rect {
    fn default() -> Self {
        Self {
            style: Style::default(),
            pos: None,
            width: None,
            height: None,
            rx: None,
            ry: None,
        }
    }
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
            expr+=format!(r#" x="{}" y="{}""#, pos.x, pos.y).as_str();
        }
        if let Some(rx) = self.rx {
            expr+=format!(r#" rx="{}""#, rx).as_str();
        }
        if let Some(ry) = self.ry {
            expr+=format!(r#" ry="{}""#, ry).as_str();
        }
        if let Some(width) = self.width {
            expr+=format!(r#" width="{}""#, width).as_str();
        }
        if let Some(height) = self.height {
            expr+=format!(r#" height="{}""#, height).as_str();
        }
        self.style.to_raw_svg(&mut expr);
        expr+=r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl Styled for Rect {
    fn get_style(&mut self) -> &mut Style {&mut self.style}
}