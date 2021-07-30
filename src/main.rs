use lyon::math::point;

struct LyonPathBuilder {
    builder: lyon::path::path::Builder,
    path_ids: Vec<u32>,
    glyph_ids: Vec<u32>,
    cur_path_id: u32,
    cur_glyph_id: u32,
    offset_x: f32,
    offset_y: f32,

    tolerance: f32,
}

impl LyonPathBuilder {
    pub fn new(tolerance: f32) -> Self {
        Self {
            builder: lyon::path::Path::builder(),
            path_ids: Vec::new(),
            glyph_ids: Vec::new(),
            cur_path_id: 0,
            cur_glyph_id: 0,
            offset_x: 0.,
            offset_y: 0.,
            tolerance,
        }
    }

    // adds offsets to x and y
    pub fn point(&self, x: f32, y: f32) -> lyon::math::Point {
        point(x + self.offset_x, y + self.offset_y)
    }

    pub fn into_path(self) -> (Vec<f32>, Vec<f32>, Vec<u32>, Vec<u32>) {
        let path = self.builder.build();

        let mut x: Vec<f32> = Vec::new();
        let mut y: Vec<f32> = Vec::new();
        let mut glyph_ids: Vec<u32> = Vec::new();
        let mut path_ids: Vec<u32> = Vec::new();
        let mut cur_path_id: u32 = 0;
        for (p, gid) in path.iter().zip(self.glyph_ids) {
            match p {
                lyon::path::Event::Begin { at } => {
                    x.push(at.x);
                    y.push(at.y);
                    glyph_ids.push(gid);
                    path_ids.push(cur_path_id);
                }
                lyon::path::Event::Line { to, .. } => {
                    x.push(to.x);
                    y.push(to.y);
                    glyph_ids.push(gid);
                    path_ids.push(cur_path_id);
                }
                lyon::path::Event::Quadratic { from, ctrl, to } => {
                    let seg = lyon::geom::QuadraticBezierSegment { from, ctrl, to };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        x.push(p.x);
                        y.push(p.y);
                        glyph_ids.push(gid);
                        path_ids.push(cur_path_id);
                    }
                }
                lyon::path::Event::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    let seg = lyon::geom::CubicBezierSegment {
                        from,
                        ctrl1,
                        ctrl2,
                        to,
                    };
                    // skip the first point as it's already added
                    for p in seg.flattened(self.tolerance).skip(1) {
                        x.push(p.x);
                        y.push(p.y);
                        glyph_ids.push(gid);
                        path_ids.push(cur_path_id);
                    }
                }
                lyon::path::Event::End { .. } => {
                    cur_path_id = cur_path_id + 1;
                }
            }
        }

        (x, y, glyph_ids, path_ids)
    }
}

impl ttf_parser::OutlineBuilder for LyonPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.begin(self.point(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(self.point(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder
            .quadratic_bezier_to(self.point(x1, y1), self.point(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder
            .cubic_bezier_to(self.point(x1, y1), self.point(x2, y2), self.point(x, y));
    }

    fn close(&mut self) {
        self.glyph_ids.push(self.cur_glyph_id);
        self.path_ids.push(self.cur_path_id);
        self.builder.close();
        self.cur_path_id = self.cur_path_id + 1;
    }
}

fn main() {}
