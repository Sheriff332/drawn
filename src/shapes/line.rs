use crate::{
    Element, ExpandElement, ImplProperties, Link, Point, Properties, StyleTarget, Transform,
};

#[derive(Clone, Default)]
pub struct Line {
    properties: Properties,
    start: Option<Point>,
    end: Option<Point>,
}

impl Line {
    pub fn start(mut self, x1: f32, y1: f32) -> Self {
        self.start = Some(Point::from(x1, y1));
        self
    }
    pub fn end(mut self, x2: f32, y2: f32) -> Self {
        self.end = Some(Point::from(x2, y2));
        self
    }
}

impl Element for Line {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<line"#.to_string();
        if let Some(pos) = self.start {
            expr += format!(r#" x1="{}" y1="{}""#, pos.x, pos.y).as_str();
        }
        if let Some(pos) = self.end {
            expr += format!(r#" x2="{}" y2="{}""#, pos.x, pos.y).as_str();
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::Line);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl_properties!(Line);
