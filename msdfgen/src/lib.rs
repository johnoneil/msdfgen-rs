/*!
# Safe bindings to msdfgen library

## Crates

- [msdfgen-sys](https://crates.io/crates/msdfgen-sys) Low-level unsafe bindings generated using bindgen.
- [msdfgen-lib](https://crates.io/crates/msdfgen-lib) Bundled library which can be build and link with application.
- [msdfgen](https://crates.io/crates/msdfgen) High-level safe bindings which should be used by applications.

## Features

- __ttf-parse__ Enables [ttf-parser](https://crates.io/crates/ttf-parser) crate integration which allows create shapes for glyphs of specific font.
- __font__ Enables [font](https://crates.io/crates/font) crate integration which allows create shapes for glyphs of specific font.
- __freetype-rs__ Enables [freetype-rs](https://crates.io/crates/freetype-rs) crate integration which allows create shapes for glyphs of specific font.
- __png__ Enables [png](https://crates.io/crates/png) crate integration which allows load and save bitmaps from/as PNG images.

## Usage

```no_run
use msdfgen_lib; // forces linking with msdfgen library
use std::fs::File;
use material_icons::{Icon, icon_to_char, FONT};
use ttf_parser::Font;
use msdfgen::{FontExt, Bitmap, EDGE_THRESHOLD, OVERLAP_SUPPORT};

let font = Font::from_data(&FONT, 0).unwrap();

let chr = icon_to_char(Icon::Fingerprint);

let glyph = font.glyph_index(chr).unwrap();

let mut shape = font.glyph_shape(glyph).unwrap();

let framing = bounds.autoframe((32.0, 32.0), Range::Px(4.0), None).unwrap();

let mut bitmap = Bitmap::new(32, 32);

shape.edge_coloring_simple(3.0, 0);

shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);

let mut output = File::create("fingerprint-msdf.png").unwrap();
bitmap.write_png(&mut output).unwrap();
```
 */

mod vector;
mod bitmap;
mod bounds;
mod segment;
mod edge;
mod contour;
mod scanline;
mod shape;
mod generate;
mod interop;

#[cfg(test)]
use msdfgen_lib as _;

pub(crate) use msdfgen_sys as ffi;

pub use self::vector::*;
pub use self::bitmap::*;
pub use self::bounds::*;
pub use self::segment::*;
pub use self::edge::*;
pub use self::contour::*;
pub use self::scanline::*;
pub use self::shape::*;
pub use self::generate::*;
pub use self::interop::*;

#[cfg(test)]
mod test {
    use std::fs::File;
    use notosans::REGULAR_TTF as FONT;
    use ttf_parser::Font;
    use crate::{FontExt, Bitmap, Range, EDGE_THRESHOLD, OVERLAP_SUPPORT};

    #[test]
    fn test() {
        let font = Font::from_data(&FONT, 0).unwrap();
        let glyph = font.glyph_index('A').unwrap();
        let mut shape = font.glyph_shape(glyph).unwrap();

        if !shape.validate() {
            panic!("Invalid shape");
        }
        shape.normalize();

        let bounds = shape.get_bounds();
        let mut bitmap = Bitmap::new(32, 32);

        println!("bounds: {:?}", bounds);

        shape.edge_coloring_simple(3.0, 0);

        let framing = bounds.autoframe((32.0, 32.0), Range::Px(4.0), None).unwrap();

        println!("framing: {:?}", framing);

        shape.generate_msdf(&mut bitmap, &framing, EDGE_THRESHOLD, OVERLAP_SUPPORT);

        let mut output = File::create("A-msdf.png").unwrap();
        bitmap.write_png(&mut output).unwrap();

        assert!(false);
    }
}
