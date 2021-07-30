use test_abglyph_lyon::builder::LyonPathBuilder;

fn main() {
    let mut builder = LyonPathBuilder::new(0.01);

    let font =
        ttf_parser::Face::from_slice(include_bytes!("../fonts/IPAexfont00401/ipaexg.ttf"), 0)
            .expect("expected font");

    let g = font.glyph_index('あ').expect("expected valid glyph");

    let bbox = font.outline_glyph(g, &mut builder);

    // let result = builder.into_path();
    let result = builder.into_fill();

    for i in 0..result.0.len() {
        println!(
            "{},{},{},{},{}",
            result.0[i], result.1[i], result.2[i], result.3[i], result.4[i]
        );
    }
}
