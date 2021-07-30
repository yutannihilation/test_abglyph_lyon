use crate::builder::LyonPathBuilder;

use lyon::tessellation::*;

// Let's use our own custom vertex type instead of the default one.
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: lyon::math::Point,
}

struct VertexCtor {
    pub prim_id: u32,
}

impl FillVertexConstructor<Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: vertex.position(),
        }
    }
}

impl LyonPathBuilder {
    pub fn into_fill(self) -> (Vec<f32>, Vec<f32>, Vec<u32>, Vec<u32>, Vec<u32>) {
        let path = self.builder.build();

        // Will contain the result of the tessellation.
        let mut geometry: VertexBuffers<Vertex, usize> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::tolerance(self.tolerance);

        {
            // Compute the tessellation.
            tessellator
                .tessellate_path(
                    &path,
                    &options,
                    &mut BuffersBuilder::new(&mut geometry, VertexCtor { prim_id: 0 }),
                )
                .unwrap();
        }

        let mut x: Vec<f32> = Vec::new();
        let mut y: Vec<f32> = Vec::new();
        let mut glyph_ids: Vec<u32> = Vec::new();
        let mut path_ids: Vec<u32> = Vec::new();
        let mut triangle_id: Vec<u32> = Vec::new();

        for (n, &i) in geometry.indices.iter().enumerate() {
            if let Some(v) = geometry.vertices.get(i) {
                x.push(v.position.x);
                y.push(v.position.y);
                glyph_ids.push(0);
                path_ids.push(0);
                triangle_id.push(n as u32 / 3);
            }
        }

        (x, y, glyph_ids, path_ids, triangle_id)
    }
}
