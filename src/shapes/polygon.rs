use crate::{Element, ExpandElement, ImplProperties, Point, Properties, Link, Transform, StyleTarget};

#[derive(Clone, Default)]
pub struct Polygon{
    properties: Properties,
    points: Vec<Point>,
}

impl Polygon {
    pub fn from(points: Vec<Point>) -> Self {
        Self {
            properties: Default::default(),
            points,
        }
    }
    pub fn point(mut self, x:f32, y:f32) -> Self {
        self.points.push(Point::from(x,y));
        self
    }
}

impl Element for Polygon {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = r#"<polygon"#.to_string();
        if !self.points.is_empty() {
            expr += r#" points=""#;
            expr += &self.points.iter().map(|p| format!("{} {}", p.x, p.y)).collect::<Vec<_>>().join(" ");
            expr += r#"""#;
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::All);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }
}

impl_properties!(Polygon);