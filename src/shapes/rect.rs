use crate::*;

#[derive(Clone, Default)]
pub struct Rect {
    id: Option<String>,
    link: Option<Link>,
    clip: bool,
    clip_path: Option<String>,
    style: Style,
    pos: Option<Point>,
    width: Option<f32>,
    height: Option<f32>,
    rx: Option<f32>,
    ry: Option<f32>,
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
        if !self.clip
            && let Some(id) = &self.id
        {
            expr += format!(r#" id="{}""#, id).as_str();
        }
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
        if let Some(clip_path) = &self.clip_path {
            expr += format!(r#" clip-path="url(#{})""#, clip_path).as_str();
        }
        self.style.to_raw_svg(&mut expr, StyleTarget::All);
        expr += r#" />"#;
        ExpandElement::RawSvg(expr)
    }

    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn id(mut self, _id: &str) -> Self
    where
        Self: Sized,
    {
        self.id = Some(_id.to_string());
        self
    }

    fn remove_id(&mut self) {
        self.id = None;
    }

    fn get_link(&self) -> Option<&Link> {
        Option::from(&self.link)
    }

    fn link(mut self, _link: &str, _new_tab: bool) -> Self
    where
        Self: Sized,
    {
        self.link = Some(Link::from(_link, _new_tab));
        self
    }

    fn remove_link(&mut self) {
        self.link = None;
    }

    fn get_clip(&self) -> bool {
        self.clip
    }

    fn clip(mut self) -> Self {
        self.clip = true;
        self
    }

    fn remove_clip(&mut self) {
        self.clip = false;
    }

    fn clip_path(mut self, element: &dyn Element) -> Self
    where
        Self: Sized,
    {
        if let Some(id) = element.get_id() {
            self.clip_path = Some(id.to_string());
        }
        self
    }
}

impl Styled for Rect {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
