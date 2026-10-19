use drawn::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut newsvg = Svg::default().size(1000.0, 1000.0);

    let x = Rect::default()
        .size(16.0, 16.0)
        .round(5.0, 5.0)
        .fill(Color::RED)
        .opacity(0.5)
        //.stroke_width(0.5)
    ;

    let mut g = Group::default().opacity(1.0);

    g.add(x.clone().at(200.0, 200.0));
    g.add(x.clone().at(150.0, 200.0));

    newsvg.add(g);

    newsvg.add(
        Rect::default()
            .size(16.0, 16.0)
            .round(5.0, 5.0)
            //.fill(Color::BLACK)
            .stroke_width(0.5)
            .at(100.0, 100.0)
            .id("hi-there"),
    );

    newsvg.add(Rect::default());

    newsvg.add(
        ThreeBeans {
            style: Style::default().opacity(1.0).fill(Color::BLACK), //inherits whatever part is needed, multiplies the opacities
            bean1: x.clone().at(50.0, 50.0).link("/abc", true),      //unrolls this!
            bean2: x.clone().at(100.0, 50.0).id("hi-hi"),
            bean3: x.clone().at(150.0, 50.0).remove_style(),
        }
        .link("def", false),
    ); //ignores this!

    let clip = Rect::default()
        .at(300.0, 300.0)
        .size(16.0, 16.0)
        .id("clip")
        .clip();

    let test = Rect::default()
        .at(300.0, 300.0)
        .size(100.0, 100.0)
        .fill(Color::GREEN)
        .clip_path(&clip);

    newsvg.add(clip);

    newsvg.add(test);

    newsvg.add(
        Rect::default()
        .at(300.0, 400.0)
        .size(50.0, 50.0)
        .transform(Transform::Rotate(10.0))
        .transform(Transform::Translate(Point::from(50.0,50.0)))
        .transform(Transform::Scale(0.9))
        .transform(Transform::Scale(1.1))
        .transform(Transform::SkewX(10.0))
        .transform(Transform::SkewY(10.0))
        .transform(Transform::Matrix([1.0,1.0,1.0,1.0,1.0,1.0])),
    );

    newsvg.write_to_file("examples/basic.svg")?;
    Ok(())
}

struct ThreeBeans {
    style: Style,
    bean1: Rect,
    bean2: Rect,
    bean3: Rect,
}

impl Styled for ThreeBeans {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Element for ThreeBeans {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children(vec![
            Box::new(self.bean1.clone()),
            Box::new(self.bean2.clone()),
            Box::new(self.bean3.clone()),
        ])
    }
}
