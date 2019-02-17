pub mod atlas;
pub mod font;
pub mod text;
pub mod rectangle;

pub mod prelude {
    pub use crate::font::Font;
//    pub use crate::text::TextCaretPositions;

}


#[cfg(test)]
pub mod test {
//    use crate::text::LayoutGlyphs;
//    use crate::font::Font;

    /*fn api(font_bytes: &[u8]){
        let font = Font::read(font_bytes).unwrap();
        let mut text_mesh: Vec<Vertex> = Vec::new();
        let mut caret_positions: Vec<(f32, f32)> = Vec::new();

        struct Vertex {
            position: (f32, f32),
            texture: (f32, f32),
        };

        for glyph in font.layout_glyphs("Hello World!") {
            caret_positions.push(glyph.layout.in_mesh.position);

            let quad_positions = glyph.layout.in_mesh.vertices();
            let quad_texture_coords = glyph.layout.in_atlas.vertices();

            for quad_vertex_index in 0..4 {
                text_mesh.push(Vertex {
                    position: quad_positions[quad_vertex_index],
                    texture: quad_texture_coords[quad_vertex_index]
                })
            }
        }
    }*/


}