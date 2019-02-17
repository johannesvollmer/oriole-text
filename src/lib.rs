pub mod atlas;
pub mod font;
pub mod text;
pub mod rectangle;

#[cfg(feature = "glium-render")]
pub mod glium_render;

pub mod prelude {
    pub use crate::font::Font;
//    pub use crate::text::TextCaretPositions;

    #[cfg(feature = "glium-render")]
    pub use crate::glium_render as gl;
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

    #[cfg(feature = "glium-render")]
    pub fn glium(font: crate::font::SerializedFont){
        use glium::{glutin, Surface};
        use crate::prelude::*;

        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let font = Font::deserialized(font);
        let font_texture = crate::glium_render::atlas_texture(&display, &font.atlas).unwrap();
        let text_mesh = crate::glium_render::TextMesh::new(&display, &font, "Hello World").unwrap();
        let solid_text_program = crate::glium_render::SolidTextProgram::new(&display).unwrap();

        let mut closed = false;
        while !closed {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.1, 1.0);

            {
                let transform = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                let fill = (1.0, 0.8, 0.2, 1.0);
                let draw_parameters = glium::DrawParameters {
                    ..Default::default()
                };

                solid_text_program.draw(&mut target, &font_texture, &text_mesh, fill, transform, &draw_parameters).unwrap();
            }

            target.finish().unwrap();

            events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }

}