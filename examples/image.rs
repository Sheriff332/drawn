//The Code to generate:
/*
<svg width="5cm" height="4cm"
     xmlns="http://www.w3.org/2000/svg">
  <image href="ferris.png" x="0" y="0" height="50px" width="50px"/>
</svg>
 */
use drawn::prelude::*;
fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let cmtopx = |cm: f32| (cm * (96.0 / 2.54) * 100.0).round() / 100.0;

    let mut svg = Svg::default().size(cmtopx(5.0), cmtopx(4.0));

    svg.push(
        Image::default()
            .href("examples/ferris.png")
            .size(50.0, 50.0),
    );

    svg.write_to_file("examples/image.svg")?;
    Ok(())
}
