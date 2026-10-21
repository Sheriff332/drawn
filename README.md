# drawn-rs
[![License](https://img.shields.io/crates/l/drawn?style=flat-square&color=blue)](LICENSE)
[![Version](https://img.shields.io/crates/v/drawn?style=flat-square&color=orange
)](https://crates.io/crates/drawn/versions)
[![Downloads](https://img.shields.io/crates/dr/drawn?style=flat-square&color=green
)](https://crates.io/crates/drawn)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Sheriff332/drawn/rust.yml?style=flat-square)](https://github.com/Sheriff332/drawn/actions)

`drawn` is a lightweight, easy-to-use interface for building SVGs and performing unique operations on in-memory SVGs.

Despite originally being made to create SVG-only websites, it has many generic SVG methods and types suitable even for power users.
It prioritizes making highly optimized SVGs in terms of speed and file size, drawing parallels with tools like SVGO. Additionally, it's power lies in custom methods and transformations, alongside user defined Elements and a powerful theming system.
## Specification & Philosophy
`drawn` targets a curated, high-performance subset of the W3C SVG 2 specification (with universal backward compatibility for SVG 1.1).

**Read the docs here:**

<a href="https://docs.rs/drawn/latest/drawn"><img src="https://img.shields.io/badge/Documentation-2F81F7?style=for-the-badge&logo=readthedocs&logoColor=white" alt="Documentation"/></a>

### Why doesn't drawn implement all SVG elements and properties?
The complete SVG specification is over 800 pages long and includes decades of legacy web baggage. `drawn` is designed from the ground up for high-framerate interactive web canvases, generative graphics, and maximum extensibility.
Thus, `drawn` does not implement methods like:
- `<filter>`, `<mask>`: Very slow and heavy on the CPU
- `<animate>`: No longer in use, and drawn makes static SVG's
- `<textPath>`, `<foreignObject>`: Buggy and inconsistent behavior, rarely used
- stroke-miterlimit, stroke-dashoffset etc.: Cases where the usage is as rare as these are omitted to keep the crate lightweight
- No CSS support: CSS has inconsistent behavior depending on the platform

If you want to use any such methods yourself which are not available here, feel free to look at some more well-established alternatives like:

[`svg`](https://crates.io/crates/svg) : traditional 1:1 XML DOM builder

[`usvg`](https://crates.io/crates/usvg) / [`resvg`](https://crates.io/crates/resvg) : for parsing / rasterizing SVG's

[`kurbo`](https://crates.io/crates/kurbo) : dedicated 2D geometry math library

## Quick Start

```rust
use drawn::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut new = Svg::default().size(100.0,100.0);
    
    new.add(
        Rect::default()
            .size(50.0, 50.0)
            .round(5.0, 5.0)
            .stroke_width(0.5)
            .at(20.0, 20.0)
            .fill(Color::RED)
            .id("hi-there"),
    );
    
    new.write_to_file("new.svg")?;
    Ok(())
}
```
#### Result:


<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
<rect id="hi-there" x="20" y="20" rx="5" ry="5" width="50" height="50" fill="#ff0000ff" stroke-width="0.5" />
</svg>

```
<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
<rect id="hi-there" x="20" y="20" rx="5" ry="5" width="50" height="50" fill="#ff0000ff" stroke-width="0.5" />
</svg>
```


## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=sheriff332/drawn&type=Date)](https://star-history.com/#sheriff332/drawn&Date)

