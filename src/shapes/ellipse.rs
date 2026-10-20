use crate::{Element, ExpandElement, ImplProperties, Point, Properties, Link, Transform, StyleTarget};

#[derive(Clone, Default)]
pub struct Ellipse {
    properties: Properties,
    center: Option<Point>,
    radii: Option<Point>,
}

impl Ellipse {
    pub fn at(mut self, cx:f32, cy:f32) -> Self {
        self.center = Some(Point::from(cx,cy));
        self
    }
    pub fn r(mut self, rx:f32, ry:f32) -> Self {
        self.radii = Some(Point::from(rx,ry));
        self
    }
}

impl Element for Ellipse {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<ellipse"#.to_string();
        if let Some(pos) = self.center {
            expr += format!(r#" cx="{}" cy="{}""#, pos.x, pos.y).as_str();
        }
        if let Some(radii) = self.radii {
            expr += format!(r#" rx="{}" ry="{}""#, radii.x, radii.y).as_str();
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::Ellipse);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl_properties!(Ellipse);