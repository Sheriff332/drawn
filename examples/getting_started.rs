//The Code to generate:
/*
<svg width="300" height="200"
     xmlns="http://www.w3.org/2000/svg">

  <rect width="100%" height="100%" fill="red" />

  <circle cx="150" cy="100" r="80" fill="green" />

  <text x="150" y="125" font-size="60" text-anchor="middle" fill="white">SVG</text>

</svg>
 */
use drawn::prelude::*;
fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let mut svg = Svg::default().size(300.0, 200.0);

    svg.push(
        Rect::default()
            .size(300.0, 200.0) //may add % later? IDK
            .fill(Color::RED),
    );
    svg.push(
        Circle::default()
            .at(150.0, 100.0)
            .r(80.0)
            .fill(Color::GREEN),
    );
    svg.push(
        Text::default()
            .push(TextSpan::from("SVG"))
            .at(150.0, 125.0)
            .font_size(60.0)
            .align_center()
            .fill(Color::WHITE),
    );

    svg.write_to_file("examples/gettingstarted.svg")?;
    Ok(())
}
