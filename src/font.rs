use crate::builder::LyonPathBuilder;

use std::{error::Error, fmt::Display};
use ttf_parser::{kern::Subtables, GlyphId};

#[derive(Debug)]
pub enum FontLoadingError {
    FaceParsingError(ttf_parser::FaceParsingError),
    IOError(std::io::Error),
}

impl Display for FontLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontLoadingError::FaceParsingError(err) => err.fmt(f),
            FontLoadingError::IOError(err) => err.fmt(f),
        }
    }
}

impl Error for FontLoadingError {}

impl From<ttf_parser::FaceParsingError> for FontLoadingError {
    fn from(err: ttf_parser::FaceParsingError) -> Self {
        Self::FaceParsingError(err)
    }
}

impl From<std::io::Error> for FontLoadingError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl LyonPathBuilder {
    pub fn outline(&mut self, text: &str, font_file: &str) -> Result<(), FontLoadingError> {
        let font_data_raw = std::fs::read(font_file)?;
        let font = ttf_parser::Face::from_slice(font_data_raw.as_slice(), 0)?;

        let subtables = font.kerning_subtables();

        let mut prev_glyph: Option<GlyphId> = None;
        for c in text.chars() {
            let cur_glyph = font.glyph_index(c).expect("expected valid glyph");

            if let Some(prev_glyph) = prev_glyph {
                self.offset_x += find_kerning(subtables, prev_glyph, cur_glyph) as f32;
            }

            font.outline_glyph(cur_glyph, self);

            if let Some(ha) = font.glyph_hor_advance(cur_glyph) {
                self.offset_x += ha as f32;
            }

            prev_glyph = Some(cur_glyph);
            self.cur_glyph_id += 1;
        }

        Ok(())
    }
}

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
