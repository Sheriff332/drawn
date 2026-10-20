use drawn::prelude::*;

//The Code to generate:
/*
<svg width="200" height="250" xmlns="http://www.w3.org/2000/svg">

  <rect x="10" y="10" width="30" height="30" stroke="black" fill="none" stroke-width="5"/>
  <rect x="60" y="10" rx="10" ry="10" width="30" height="30" stroke="black" fill="none" stroke-width="5"/>

  <circle cx="25" cy="75" r="20" stroke="red" fill="none" stroke-width="5"/>
  <ellipse cx="75" cy="75" rx="20" ry="5" stroke="red" fill="none" stroke-width="5"/>

  <line x1="10" x2="50" y1="110" y2="150" stroke="orange" stroke-width="5"/>
  <polyline points="60 110 65 120 70 115 75 130 80 125 85 140 90 135 95 150 100 145"
      stroke="orange" fill="none" stroke-width="5"/>

  <polygon points="50 160 55 180 70 180 60 190 65 205 50 195 35 205 40 190 30 180 45 180"
      stroke="green" fill="none" stroke-width="5"/>

  <path d="M20,230 Q40,205 50,230 T90,230" fill="none" stroke="blue" stroke-width="5"/>
</svg>
*/

fn main() -> Result<(), std::io::Error> {
    let mut shapesvg = Svg::default().size(200.0,250.0);

    shapesvg.add(
        Rect::default()
            .size(30.0,30.0)
            .at(10.0,10.0)
            .stroke(Color::BLACK)
            .fill(Color::TRANSPARENT)
            .stroke_width(5.0),
    );
    shapesvg.add(
        Rect::default()
            .size(30.0,30.0)
            .at(60.0,10.0)
            .round(10.0, 10.0)
            .stroke(Color::BLACK)
            .fill(Color::TRANSPARENT)
            .stroke_width(5.0),
    );
    shapesvg.add(
        Circle::default()
            .at(25.0,75.0)
            .r(20.0)
            .stroke(Color::RED)
            .fill(Color::TRANSPARENT)
            .stroke_width(5.0),
    );
    shapesvg.add(
        Ellipse::default()
            .at(75.0,75.0)
            .r(20.0,5.0)
            .stroke(Color::RED)
            .fill(Color::TRANSPARENT)
            .stroke_width(5.0),
    );
    shapesvg.add(
        Line::default()
            .start(10.0,110.0)
            .end(50.0,150.0)
            .stroke(Color::from_hex("#ffa500").unwrap()) //Orange
            .stroke_width(5.0),
    );
    shapesvg.add(
        Polyline::default()
            .point(60.0, 110.0)
            .point(65.0, 120.0)
            .point(70.0, 115.0)
            .point(75.0, 130.0)
            .point(80.0, 125.0)
            .point(85.0, 140.0)
            .point(90.0, 135.0)
            .point(95.0, 150.0)
            .point(100.0, 145.0)
            .stroke(Color::from_hex("#ffa500").unwrap()) //Orange
            .fill(Color::TRANSPARENT) // dont even need this lol
            .stroke_width(5.0),
    );
    shapesvg.add(
        Polygon::default()
            .point(50.0, 160.0)
            .point(55.0, 180.0)
            .point(70.0, 180.0)
            .point(60.0, 190.0)
            .point(65.0, 205.0)
            .point(50.0, 195.0)
            .point(35.0, 205.0)
            .point(40.0, 190.0)
            .point(30.0, 180.0)
            .point(45.0, 180.0)
            .stroke(Color::GREEN) //Orange
            .fill(Color::TRANSPARENT) // dont even need this lol
            .stroke_width(5.0),
    );
    //no path yet
    shapesvg.write_to_file("examples/shapesvg.svg")?;
    Ok(())
}