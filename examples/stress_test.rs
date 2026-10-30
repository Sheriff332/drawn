use drawn::prelude::*;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let mut root_svg = Svg::default()
        .size(2400.0, 1600.0)
        // Root Theme Cascade:
        .stroke(Color::BLACK)
        .stroke_width(1.0);

    // =========================================================================
    // 1. THE PROCEDURAL HIVE (Volume & Math Stress: ~400 shapes)
    // Tests loop unrolling speed and property cloning/inheritance.
    // =========================================================================
    let mut hive = Group::default().opacity(0.85);

    for row in 0..20 {
        for col in 0..20 {
            let x = 50.0 + (col as f32) * 45.0;
            let y = 50.0 + (row as f32) * 45.0;

            if (row + col) % 3 == 0 {
                hive.push(
                    Rect::default()
                        .at(x, y)
                        .size(35.0, 35.0)
                        .round(6.0, 6.0)
                        .fill(if row % 2 == 0 { Color::RED } else { Color::BLUE }),
                );
            } else if (row + col) % 3 == 1 {
                hive.push(
                    Circle::default()
                        .at(x + 17.5, y + 17.5)
                        .r(16.0)
                        .stroke_width(2.0), // Inherits root stroke color!
                );
            } else {
                hive.push(
                    Line::default()
                        .start(x, y)
                        .end(x + 35.0, y + 35.0)
                        .stroke_width(2.5),
                );
            }
        }
    }
    root_svg.push(hive);

    // =========================================================================
    // 2. THE TRANSFORMED CLIP MASK
    // Tests .clip(), <defs><clipPath>, and masked rendering.
    // =========================================================================
    let mut mask_svg = Svg::default()
        .id("stress-mask")
        .clip();

    // The mask stencil shape (Transformed polygon + rect):
    mask_svg.push(
        Polygon::default()
            .point(1100.0, 100.0)
            .point(1300.0, 150.0)
            .point(1250.0, 400.0)
            .point(1050.0, 350.0),
    );
    root_svg.push(mask_svg);

    // The content being masked:
    let mut masked_content = Group::default();
    masked_content.push(
        Rect::default()
            .at(1000.0, 50.0)
            .size(400.0, 400.0)
            .fill(Color::GREEN)
            .clip_path("stress-mask"),
    );
    root_svg.push(masked_content);

    // =========================================================================
    // 3. THE LINK COLLISION MINEFIELD
    // Tests nested link pruning.
    // Parent has link, Child 1 has link (must prune Parent's link for Child 1).
    // Child 2 has NO link (must receive Parent's link!).
    // =========================================================================
    let cluster = NestedCluster {
        // Child 1: Overrides link (must emit /child-link)
        properties: Default::default(),
        child_with_link: Rect::default()
            .at(1000.0, 550.0)
            .size(120.0, 60.0)
            .fill(Color::RED)
            .link("/child-link", true),

        // Child 2: No link (must inherit /parent-link without crashing)
        child_unlinked: Rect::default()
            .at(1150.0, 550.0)
            .size(120.0, 60.0)
            .fill(Color::BLUE),
    };

    root_svg.push(
        cluster.link("/parent-link", false),
    );

    // =========================================================================
    // 4. COMPLEX BÉZIER WAVE WALL
    // Tests path commands (M, Q, T) and stroke join rendering.
    // =========================================================================
    let mut wave_group = Group::default().stroke_width(3.0);
    for i in 0..10 {
        let y_offset = 700.0 + (i as f32) * 25.0;
        wave_group.push(
            Path::default()
                .M(&[Point::from(50.0, y_offset)])
                .Q(250.0, y_offset - 40.0, 450.0, y_offset)
                .T(850.0, y_offset)
                .fill(Color::TRANSPARENT)
                .stroke(Color::BLUE),
        );
    }
    root_svg.push(wave_group);

    // =========================================================================
    // 5. INLINE TYPOGRAPHY STRESS
    // Tests font fallbacks, spans, decorations, and styles.
    // =========================================================================
    root_svg.push(
        Text::default()
            .at(50.0, 1050.0)
            .font("Virgil, cursive, sans-serif")
            .font_size(28.0)
            .push(
                TextSpan::from("DRAWN ENGINE ")
                    .bold()
                    .fill(Color::BLACK),
            )
            .push(
                TextSpan::from("v0.2.1 ")
                    .italic()
                    .fill(Color::RED)
                    .underline(),
            )
            .push(
                TextSpan::from("[STRESS TEST COMPLETED]")
                    .letter_spacing(4.0)
                    .fill(Color::GREEN),
            ),
    );

    // =========================================================================
    // EXECUTION & TIMING
    // =========================================================================
    let file_path = "examples/stress_output.svg";
    root_svg.write_to_file(file_path)?;

    Ok(())
}

// -----------------------------------------------------------------------------
// Custom Component for testing Link Collisions inside the AST
// -----------------------------------------------------------------------------
struct NestedCluster {
    properties: Properties,
    child_with_link: Rect,
    child_unlinked: Rect,
}

impl_properties!(NestedCluster);

impl Element for NestedCluster {
    fn expand(&mut self) -> ExpandElement {
        ExpandElement::Children(vec![
            Box::new(self.child_with_link.clone()),
            Box::new(self.child_unlinked.clone()),
        ])
    }
}