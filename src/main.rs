use test_abglyph_lyon::builder::LyonPathBuilder;
use ttf_parser::{kern::Subtables, GlyphId};

const TEXT: &str = "VALUE";

fn find_kerning(subtables: Subtables, left: GlyphId, right: GlyphId) -> i16 {
    for st in subtables {
        // Do I need to also skip if the font is variable?
        if !st.is_horizontal() {
            continue;
        }

        if let Some(kern) = st.glyphs_kerning(left, right) {
            return kern;
        }
    }

    0
}

fn main() {
    let mut builder = LyonPathBuilder::new(0.01, 10.0);

    let font = ttf_parser::Face::from_slice(
        include_bytes!("/usr/share/fonts/opentype/urw-base35/P052-Italic.otf"),
        0,
    )
    .expect("expected font");

    let subtables = font.kerning_subtables();

    let mut prev_glyph: Option<GlyphId> = None;
    for c in TEXT.chars() {
        let cur_glyph = font.glyph_index(c).expect("expected valid glyph");

        if let Some(prev_glyph) = prev_glyph {
            builder.offset_x += find_kerning(subtables, prev_glyph, cur_glyph) as f32;
        }

        font.outline_glyph(cur_glyph, &mut builder);

        if let Some(ha) = font.glyph_hor_advance(cur_glyph) {
            builder.offset_x += ha as f32;
        }

        prev_glyph = Some(cur_glyph);
        builder.cur_glyph_id += 1;
    }

    // let result = builder.into_path();
    let result = builder.into_fill();
    // let result = builder.into_stroke();

    for i in 0..result.0.len() {
        println!(
            "{},{},{},{},{}",
            result.0[i], result.1[i], result.2[i], result.3[i], result.4[i]
        );
    }
}
