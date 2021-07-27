use ab_glyph::{Font, FontRef, Glyph};
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::*;

// Let's use our own custom vertex type instead of the default one.
#[derive(Copy, Clone, Debug)]
struct MyVertex {
    position: [f32; 2],
    // id: u32,
    // glyph_id: u32,
}

struct VertexCtor {
    pub prim_id: u32,
}

impl StrokeVertexConstructor<MyVertex> for VertexCtor {
    fn new_vertex(&mut self, mut vertex: StrokeVertex) -> MyVertex {
        let pos = vertex.position().to_array();
        let ids = vertex.interpolated_attributes();
        MyVertex {
            position: pos,
            // id: ids[0] as _,
            // glyph_id: ids[1] as _,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(lyon::math::Point);

impl From<ab_glyph::Point> for Point {
    fn from(p: ab_glyph::Point) -> Self {
        Self(point(p.x, p.y))
    }
}

impl From<Point> for lyon::math::Point {
    fn from(p: Point) -> Self {
        p.0
    }
}

pub struct MyBuilder {
    builder: lyon::path::path::Builder,
    last_point: Option<Point>,
}

impl MyBuilder {
    fn new() -> Self {
        Self {
            builder: Path::builder(),
            last_point: None,
        }
    }

    fn end(&mut self, close: bool) {
        self.builder.end(close);
    }

    /// This is a workaround for that ab_glyph doesn't expose which is the start
    /// point and the end point.
    /// 1. If `at` is the first point, do `begin()`.
    /// 2. If `at` is the same as the last point, do nothing.
    /// 3. Otherwise, `end()` first, and `begin()`.
    fn maybe_begin(&mut self, at: Point) {
        if let Some(p) = self.last_point {
            if p.0 == at.0 {
                return;
            }
            self.end(true);
        }
        self.builder.begin(at.into());
    }

    fn line_to(&mut self, to: Point) -> lyon::path::EndpointId {
        self.last_point = Some(to);
        self.builder.line_to(to.into())
    }

    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) -> lyon::path::EndpointId {
        self.last_point = Some(to);
        self.builder.quadratic_bezier_to(ctrl.into(), to.into())
    }

    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) -> lyon::path::EndpointId {
        self.last_point = Some(to);
        self.builder
            .cubic_bezier_to(ctrl1.into(), ctrl2.into(), to.into())
    }

    fn to_path(self, tolerance: f32) -> (Vec<f32>, Vec<f32>, Vec<u32>) {
        let mut x: Vec<f32> = Vec::new();
        let mut y: Vec<f32> = Vec::new();
        let mut ids: Vec<u32> = Vec::new();
        let mut cur_id: u32 = 0;

        let path = self.builder.build();

        for e in path.iter() {
            match e {
                path::Event::Begin { at } => {
                    x.push(at.x);
                    y.push(at.y);
                    ids.push(cur_id);
                }
                path::Event::Line { to, .. } => {
                    x.push(to.x);
                    y.push(to.y);
                    ids.push(cur_id);
                }
                path::Event::Quadratic { from, ctrl, to } => {
                    let seg = lyon::geom::QuadraticBezierSegment { from, ctrl, to };
                    // skip the first point as it's already added
                    for p in seg.flattened(tolerance).skip(1) {
                        x.push(p.x);
                        y.push(p.y);
                        ids.push(cur_id);
                    }
                }
                path::Event::Cubic {
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
                    for p in seg.flattened(tolerance).skip(1) {
                        x.push(p.x);
                        y.push(p.y);
                        ids.push(cur_id);
                    }
                }
                path::Event::End { last, .. } => {
                    x.push(last.x);
                    y.push(last.y);
                    ids.push(cur_id);
                    cur_id = cur_id + 1;
                }
            }
        }

        (x, y, ids)
    }
}

fn main() {
    let mut builder = MyBuilder::new();

    let font =
        FontRef::try_from_slice(include_bytes!("../fonts/IPAexfont00401/ipaexg.ttf")).unwrap();

    if let Some(outline) = font.outline(font.glyph_id('眠')) {
        // println!("{:?}", outline.bounds);
        for c in outline.curves {
            // println!("{:?}", c);
            match c {
                ab_glyph::OutlineCurve::Line(from, to) => {
                    builder.maybe_begin(from.into());
                    builder.line_to(to.into());
                }
                ab_glyph::OutlineCurve::Quad(from, ctrl, to) => {
                    builder.maybe_begin(from.into());
                    builder.quadratic_bezier_to(ctrl.into(), to.into());
                }
                ab_glyph::OutlineCurve::Cubic(from, ctrl1, ctrl2, to) => {
                    builder.maybe_begin(from.into());
                    builder.cubic_bezier_to(ctrl1.into(), ctrl2.into(), to.into());
                }
            }
        }
        builder.end(true);

        let path = builder.builder.build();

        // Will contain the result of the tessellation.
        let mut geometry: VertexBuffers<MyVertex, usize> = VertexBuffers::new();
        let mut tessellator = StrokeTessellator::new();
        {
            // Compute the tessellation.
            tessellator
                .tessellate_path(
                    &path,
                    &StrokeOptions::tolerance(0.2).with_line_width(80.0),
                    &mut BuffersBuilder::new(&mut geometry, VertexCtor { prim_id: 0 }),
                )
                .unwrap();
        }

        for i in geometry.indices {
            if let Some(v) = geometry.vertices.get(i) {
                println!("{},{}", v.position[0], v.position[1]);
            } else {
                println!(",");
            }
        }
    }
}
