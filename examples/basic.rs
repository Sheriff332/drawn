use drawn::prelude::*;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let mut newsvg = Svg::default().size(1000.0, 1000.0);

    let x = Rect::default()
        .size(16.0, 16.0)
        .round(5.0, 5.0)
        .fill(Color::RED)
        .opacity(0.5)
        //.stroke_width(0.5)
    ;

    let mut g = Group::default().opacity(1.0);

    g.push(x.clone().at(200.0, 200.0));
    g.push(x.clone().at(150.0, 200.0));

    newsvg.push(g);

    newsvg.push(Rect::default());

    newsvg.push(
        ThreeBeans {
            properties: Properties::default().opacity(1.0).fill(Color::BLACK),
            bean1: x.clone().at(50.0, 50.0).link("/abc", true), //unrolls this!
            bean2: x.clone().at(100.0, 50.0).id("hi-hi"),
            bean3: x.clone().at(150.0, 50.0).remove_style(),
        }
        .link("def", false), //ignores this!
    );

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

    newsvg.push(clip);

    newsvg.push(test);

    newsvg.push(
        Rect::default()
            .at(300.0, 400.0)
            .size(50.0, 50.0)
            .transform(Transform::Rotate(10.0))
            .transform(Transform::Translate(Point::from(50.0, 50.0)))
            .transform(Transform::Scale(0.9))
            .transform(Transform::Scale(1.1))
            .transform(Transform::SkewX(10.0))
            .transform(Transform::SkewY(10.0))
            .transform(Transform::Matrix([1.0, 1.0, 1.0, 1.0, 1.0, 1.0])),
    );

    newsvg.push(
        Image::default()
            .href("examples/img.png")
            .at(500.0, 500.0)
            .size(100.0, 100.0)
            .opacity(0.5),
    );

    let mut insertsvg = Svg::default().size(500.0, 500.0);
    insertsvg.push(
        Rect::default()
            .size(16.0, 16.0)
            .round(5.0, 5.0)
            .at(200.0, 100.0)
            .fill(Color::BLUE),
    );
    newsvg.push(insertsvg);

    newsvg.push(
        Text::default()
            .at(400.0, 100.0)
            .push(
                TextSpan::from("abc")
                    .fill(Color::RED)
                    .oblique()
                    .letter_spacing(10.0),
            )
            .push(
                TextSpan::from("Hello")
                    .text("Hello Now")
                    .bold()
                    .italic()
                    .underline(),
            )
            .stroke_width(0.5),
    );

    newsvg.write_to_file("examples/basic.svg")?;
    Ok(())
}

struct ThreeBeans {
    properties: Properties,
    bean1: Rect,
    bean2: Rect,
    bean3: Rect,
}

impl_properties!(ThreeBeans);

impl Element for ThreeBeans {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children(vec![
            Box::new(self.bean1.clone()),
            Box::new(self.bean2.clone()),
            Box::new(self.bean3.clone()),
        ])
    }
}
