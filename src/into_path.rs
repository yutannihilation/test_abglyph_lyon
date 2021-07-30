use crate::builder::LyonPathBuilder;

impl LyonPathBuilder {
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
                    cur_path_id += 1;
                }
            }
        }

        (x, y, glyph_ids, path_ids)
    }
}
