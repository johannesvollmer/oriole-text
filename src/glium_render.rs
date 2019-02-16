use crate::font::Font;

#[derive(Copy, Clone)]
pub struct GlyphQuadVertex {
    position: (f32, f32),
    texture_coordinate: (f32, f32),
}

glium::implement_vertex!(GlyphQuadVertex, position, texture_coordinate);


pub struct TextMesh {
    vertices: glium::VertexBuffer<GlyphQuadVertex>,
    indices: glium::IndexBuffer<u16>
}

impl TextMesh {
    pub fn new(facade: &impl glium::backend::Facade, font: &Font, text: &str) -> Option<Self> {
        let (vertices, indices) = TextMesh::compute_buffers(font, text);
        Some(TextMesh {
            vertices: glium::VertexBuffer::new(facade, &vertices).ok()?,
            indices: glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &indices).ok()?
        })
    }

    pub fn set(&mut self, font: &Font, text: &str){
        let (vertices, indices) = TextMesh::compute_buffers(font, text);
        self.vertices.write(&vertices);
        self.indices.write(&indices);
    }

    pub fn compute_buffers(font: &Font, text: &str) -> (Vec<GlyphQuadVertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for glyph in font.layout_glyphs(text) {
            let quad_positions = glyph.layout.in_mesh.vertices();
            let quad_texture_coords = glyph.layout.in_atlas.vertices();

            for quad_vertex_index in 0..4 {
                for triangle_index in [ 0,1,2,  2,3,0 ] {
                    indices.push((vertices.len() + triangle_index) as u16);
                }

                vertices.push(GlyphQuadVertex {
                    position: quad_positions[quad_vertex_index],
                    texture_coordinate: quad_texture_coords[quad_vertex_index]
                })
            }
        }

        (vertices, indices)
    }

}