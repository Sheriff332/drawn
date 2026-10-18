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
            .at(100.0, 100.0),
    );

    newsvg.add(Rect::default());

    newsvg.add(ThreeBeans {
        bean1: x.clone().at(50.0, 50.0),
        bean2: x.clone().at(100.0, 50.0),
        bean3: x.clone().at(150.0, 50.0),
    });

    newsvg.write_to_file("examples/basic.svg")?;
    Ok(())
}

struct ThreeBeans {
    bean1: Rect,
    bean2: Rect,
    bean3: Rect,
}

impl Styled for ThreeBeans {
    fn get_style(&mut self) -> &mut Style {
        unimplemented!()
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
