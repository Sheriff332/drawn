use crate::{
    Element, ExpandElement, ImplProperties, Link, Point, Properties, StyleTarget, Transform,
};
use std::fmt::Display;

#[derive(Default)]
pub struct Text {
    properties: Properties,
    font_properties: FontProperties,
    texts: Vec<TextSpan>,
    pos: Option<Point>,
}

impl Text {
    pub fn at(mut self, x:f32, y:f32) -> Self {
        self.pos = Some(Point::from(x,y));
        self
    }
    pub fn add(mut self, span: TextSpan) -> Self {
        self.texts.push(span);
        self
    }
}

#[derive(Default)]
pub struct TextSpan {
    properties: Properties,
    font_properties: FontProperties,
    content: Option<String>,
}

impl TextSpan {
    pub fn from(content: &str) -> Self {
        Self {
            properties: Default::default(),
            font_properties: Default::default(),
            content: Some(content.to_string()),
        }
    }
    pub fn text(mut self, text: &str) -> Self {
        self.content = Some(text.to_string());
        self
    }
}

impl_properties!(TextSpan);

impl Element for TextSpan {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::RawSvg(String::new())
    }
}

impl_properties!(Text);

impl Element for Text {
    fn expand(&mut self) -> ExpandElement {
        let mut expr = String::new();
        expr += "<text";
        if let Some(pos) = self.pos {
            expr += format!(r#" x="{}" y="{}""#, pos.x, pos.y).as_str();
        }
        self.properties.to_raw_svg(&mut expr, StyleTarget::All);
        self.font_properties.to_raw_svg(&mut expr);
        expr += ">\n";
        for span in &self.texts {
            let mut spanexpr = String::new();
            spanexpr += "<tspan";
            span.properties.to_raw_svg(&mut spanexpr, StyleTarget::All);
            span.font_properties.to_raw_svg(&mut spanexpr);
            spanexpr += ">";
            if let Some(content) = &span.content {
                spanexpr += content.as_str();
            }
            spanexpr += "</tspan>\n";
            expr += spanexpr.as_str();
        }
        expr += "</text>";
        ExpandElement::RawSvg(expr)
    }
}

#[derive(Default)]
pub struct FontProperties {
    font_family: Option<String>,
    font_size: Option<f32>,
    font_weight: Option<FontWeight>,
    font_style: Option<FontStyle>,
    text_decoration: Option<TextDecoration>,
    letter_spacing: Option<f32>,
    text_anchor: Option<TextAnchor>,
    dominant_baseline: Option<DominantBaseline>,
}

pub enum FontWeight {
    Bold,
    Bolder,
    Lighter,
    Num(f32),
}
impl Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let FontWeight::Num(n) = self {
            write!(f, "{}", n)?;
        }
        write!(
            f,
            "{}",
            match self {
                FontWeight::Bold => "bold",
                FontWeight::Bolder => "bolder",
                FontWeight::Lighter => "lighter",
                _ => "",
            }
        )?;
        Ok(())
    }
}

pub enum FontStyle {
    Italic,
    Oblique,
}
impl Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FontStyle::Italic => "italic",
                FontStyle::Oblique => "oblique",
            }
        )?;
        Ok(())
    }
}

#[derive(Default)]
pub struct TextDecoration {
    underline: bool,
    line_through: bool,
    overline: bool,
}
impl Display for TextDecoration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            if self.underline { "underline" } else { "" },
            if self.line_through {
                "line-through"
            } else {
                ""
            },
            if self.overline { "overline" } else { "" },
        )?;
        Ok(())
    }
}

pub enum TextAnchor {
    Center,
    Right,
}
impl Display for TextAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAnchor::Center => "center",
                TextAnchor::Right => "right",
            }
        )?;
        Ok(())
    }
}

pub enum DominantBaseline {
    Middle,
    Hanging,
}
impl Display for DominantBaseline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DominantBaseline::Middle => "middle",
                DominantBaseline::Hanging => "hanging",
            }
        )?;
        Ok(())
    }
}

impl FontProperties {
    fn to_raw_svg<'a>(&self, expr: &'a mut String) -> &'a mut String {
        if let Some(font_family) = &self.font_family {
            *expr += format!(r#" font-family="{}""#, font_family).as_str();
        }
        if let Some(font_size) = &self.font_size {
            *expr += format!(r#" font-size="{}""#, font_size).as_str();
        }
        if let Some(font_weight) = &self.font_weight {
            *expr += format!(r#" font-weight="{}""#, font_weight).as_str();
        }
        if let Some(font_style) = &self.font_style {
            *expr += format!(r#" font-style="{}""#, font_style).as_str();
        }
        if let Some(text_decoration) = &self.text_decoration {
            *expr += format!(r#" text-decoration="{}""#, text_decoration).as_str();
        }
        if let Some(letter_spacing) = &self.letter_spacing {
            *expr += format!(r#" letter-spacing="{}""#, letter_spacing).as_str();
        }
        if let Some(text_anchor) = &self.text_anchor {
            *expr += format!(r#" text-anchor="{}""#, text_anchor).as_str();
        }
        if let Some(dominant_baseline) = &self.dominant_baseline {
            *expr += format!(r#" dominant-baseline="{}""#, dominant_baseline).as_str();
        }

        expr
    }
}

pub trait TextElement {
    fn get_font_properties_mut(&mut self) -> &mut FontProperties;
    fn remove_font_properties(&mut self) {
        *self.get_font_properties_mut() = FontProperties::default();
    }

    fn font(mut self, value: &str) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_family = Some(value.to_string());
        self
    }
    fn default_font_family(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_family = None;
        self
    }

    fn font_size(mut self, value: f32) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_size = Some(value);
        self
    }
    fn default_font_size(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_size = None;
        self
    }

    fn bold(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_weight = Some(FontWeight::Bold);
        self
    }
    fn bolder(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_weight = Some(FontWeight::Bolder);
        self
    }
    fn lighter(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_weight = Some(FontWeight::Lighter);
        self
    }
    fn font_weight(mut self, value: f32) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_weight = Some(FontWeight::Num(value));
        self
    }
    fn default_font_weight(mut self) -> Self
    where
        Self: Sized,
    {
        self.get_font_properties_mut().font_weight = None;
        self
    }

    fn italic(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().font_style = Some(FontStyle::Italic);
        self
    }
    fn oblique(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().font_style = Some(FontStyle::Oblique);
        self
    }
    fn default_font_style(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().font_style = None;
        self
    }

    fn underline(mut self) -> Self where Self: Sized {
        if self.get_font_properties_mut().text_decoration.is_none() {
            self.get_font_properties_mut().text_decoration = Some(TextDecoration::default());
        }
        self.get_font_properties_mut().text_decoration.as_mut().unwrap().underline = true;
        self
    }
    fn remove_underline(mut self) -> Self where Self: Sized {
        if self.get_font_properties_mut().text_decoration.is_some() {
            self.get_font_properties_mut().text_decoration.as_mut().unwrap().underline = false;
        }
        self
    }
    fn line_through(mut self) -> Self where Self: Sized {
        if self.get_font_properties_mut().text_decoration.is_none() {
            self.get_font_properties_mut().text_decoration = Some(TextDecoration::default());
        }
        self.get_font_properties_mut().text_decoration.as_mut().unwrap().line_through = true;
        self
    }
    fn remove_line_through(mut self) -> Self where Self: Sized {
        if self.get_font_properties_mut().text_decoration.is_some() {
            self.get_font_properties_mut().text_decoration.as_mut().unwrap().line_through = false;
        }
        self
    }
    fn overline(mut self) -> Self where Self: Sized {
        if self.get_font_properties_mut().text_decoration.is_none() {
            self.get_font_properties_mut().text_decoration = Some(TextDecoration::default());
        }
        self.get_font_properties_mut().text_decoration.as_mut().unwrap().overline = true;
        self
    }
    fn remove_overline(mut self) -> Self where Self: Sized {
        if self.get_font_properties_mut().text_decoration.is_some() {
            self.get_font_properties_mut().text_decoration.as_mut().unwrap().overline = false;
        }
        self
    }
    fn remove_text_decoration(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().text_decoration = None;
        self
    }

    fn letter_spacing(mut self, value: f32) -> Self where Self: Sized {
        self.get_font_properties_mut().letter_spacing = Some(value);
        self
    }
    fn remove_letter_spacing(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().letter_spacing = None;
        self
    }

    fn align_center(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().text_anchor = Some(TextAnchor::Center);
        self
    }
    fn align_right(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().text_anchor = Some(TextAnchor::Right);
        self
    }
    fn align_left(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().text_anchor = None;
        self
    }

    fn baseline_middle(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().dominant_baseline = Some(DominantBaseline::Middle);
        self
    }
    fn baseline_hanging(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().dominant_baseline = Some(DominantBaseline::Hanging);
        self
    }
    fn baseline_auto(mut self) -> Self where Self: Sized {
        self.get_font_properties_mut().dominant_baseline = None;
        self
    }
}

impl TextElement for Text {
    fn get_font_properties_mut(&mut self) -> &mut FontProperties {
        &mut self.font_properties
    }
}
impl TextElement for TextSpan {
    fn get_font_properties_mut(&mut self) -> &mut FontProperties {
        &mut self.font_properties
    }
}