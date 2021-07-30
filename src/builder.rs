use lyon::math::point;

pub struct LyonPathBuilder {
    pub builder: lyon::path::path::Builder,
    pub glyph_ids: Vec<u32>,
    pub path_ids: Vec<u32>,
    pub cur_glyph_id: u32,
    pub cur_path_id: u32,
    offset_x: f32,
    offset_y: f32,

    pub tolerance: f32,
}

impl LyonPathBuilder {
    pub fn new(tolerance: f32) -> Self {
        Self {
            builder: lyon::path::Path::builder(),
            glyph_ids: Vec::new(),
            path_ids: Vec::new(),
            cur_glyph_id: 0,
            cur_path_id: 0,
            offset_x: 0.,
            offset_y: 0.,
            tolerance,
        }
    }

    // adds offsets to x and y
    pub(crate) fn point(&self, x: f32, y: f32) -> lyon::math::Point {
        point(x + self.offset_x, y + self.offset_y)
    }
}

impl ttf_parser::OutlineBuilder for LyonPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.glyph_ids.push(self.cur_glyph_id);
        self.builder.begin(self.point(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.glyph_ids.push(self.cur_glyph_id);
        self.builder.line_to(self.point(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.glyph_ids.push(self.cur_glyph_id);
        self.builder
            .quadratic_bezier_to(self.point(x1, y1), self.point(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.glyph_ids.push(self.cur_glyph_id);
        self.builder
            .cubic_bezier_to(self.point(x1, y1), self.point(x2, y2), self.point(x, y));
    }

    fn close(&mut self) {
        self.glyph_ids.push(self.cur_glyph_id);
        self.builder.close();
    }
}
