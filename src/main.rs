use test_abglyph_lyon::builder::LyonPathBuilder;

const TEXT: &str = "VALUE";
const FONT_FILE: &str = "/home/yutani/Downloads/SourceCodePro-Black.otf";

fn main() {
    let mut builder = LyonPathBuilder::new(0.01, 10.0);

    builder.outline(TEXT, FONT_FILE).unwrap();

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
