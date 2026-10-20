use crate::{Element, ExpandElement, ImplProperties, Point, Properties, Link, Transform, StyleTarget};

#[derive(Clone, Default)]
pub struct Circle {
    properties: Properties,
    center: Option<Point>,
    radius: Option<f32>,
}

impl Circle {
    pub fn at(mut self, cx:f32, cy:f32) -> Self {
        self.center = Some(Point::from(cx,cy));
        self
    }
    pub fn r(mut self, r:f32) -> Self {
        self.radius = Some(r);
        self
    }
}

impl Element for Circle {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<circle"#.to_string();
        if let Some(pos) = self.center {
            expr += format!(r#" cx="{}" cy="{}""#, pos.x, pos.y).as_str();
        }
        if let Some(r) = self.radius {
            expr += format!(r#" r="{}""#, r).as_str();
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::Ellipse);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl_properties!(Circle);