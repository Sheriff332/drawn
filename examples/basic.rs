use drawn::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut newsvg = Svg::new();
    
    let x = Rect::new()//something like this
        .size(16.0, 16.0)
        .round(5.0, 5.0);

    newsvg.add(
        ThreeBeans {
            bean1: x.at(50.0, 50.0),
            bean2: x.at(100.0, 50.0),
            bean3: x.at(150.0, 50.0),
        }
    );

    newsvg.write_to_file("abcd.svg")?;
    Ok(())
}

struct ThreeBeans{
    bean1: Rect,
    bean2: Rect,
    bean3: Rect,
}
impl Element for ThreeBeans {
    fn expand(&self) -> ExpandElement {
        ExpandElement::Children(
            vec![Box::new(self.bean1), Box::new(self.bean2), Box::new(self.bean3) ]
        )
    }
}