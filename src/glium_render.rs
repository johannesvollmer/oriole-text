use crate::font::Font;

use glium::Surface;
use glium::Program;
use glium::DrawError;
use glium::ProgramCreationError;
use glium::backend::Facade;
use glium::DrawParameters;
use glium::uniforms::uniform;
use glium::uniform;
use glium::texture::Texture2d;
use glium::texture::TextureCreationError;
use glium::texture::RawImage2d;
use std::borrow::Cow;
use glium::texture::ClientFormat;
use crate::atlas::Atlas;


pub struct SolidTextProgram {
    pub program: glium::Program,
}

#[derive(Copy, Clone)]
pub struct GlyphQuadVertex {
    position: (f32, f32),
    texture_coordinate: (f32, f32),
}

#[derive(Copy, Clone)]
pub struct TextMesh {
    vertices: glium::VertexBuffer<GlyphQuadVertex>,
    indices: glium::IndexBuffer<u16>,
    width: f32,
}

#[derive(Display, Debug)]
pub enum TextMeshCreationError {
    Vertex(glium::vertex::BufferCreationError),
    Index(glium::index::BufferCreationError),
}


glium::implement_vertex!(GlyphQuadVertex, position, texture_coordinate);


pub fn atlas_texture(facade: &impl Facade, atlas: &Atlas)
     -> Result<glium::texture::Texture2d, TextureCreationError>
{
    atlas_texture(facade, &atlas.distance_field, atlas.resolution)
}

pub fn raw_u8_texture(facade: &impl Facade, atlas: &[u8], dimensions: (usize, usize))
    -> Result<glium::texture::Texture2d, TextureCreationError>
{
    glium::texture::Texture2d::new(
        facade, RawImage2d {
            data: Cow::Borrowed(atlas),
            width: dimensions.0 as u32,
            height: dimensions.1 as u32,
            format: ClientFormat::U8
        }
    )
}

impl SolidTextProgram {
    pub fn new(facade: &impl Facade) -> Result<Self, ProgramCreationError> {
        let program = glium::Program::from_source(
            facade,

            r#"version 330
                in vec2 position;
                in vec2 texture_coordinate;
                out vec2 texture_position;

                uniform mat4 transform;

                void main(){
                    gl_Position = (transform * vec4(position, 1.0, 1.0));
                    texture_position = texture_coordinate;
                }
            "#,

            r#"version 330
                in vec2 texture_position;
                out vec4 color;

                uniform vec4 fill;
                uniform texture2d distance_field;

                void main(){
                    float distance = texture(distance_field, texture_position).r;
                    distance = distance > 0.5? 1.0 : 0.0; // TODO

                    color = fill * vec4(vec3(1.0), distance);
                }
            "#,

            None
        );

        program.map(|program| SolidTextProgram { program })
    }

    pub fn draw(
        &self,
        surface: &mut impl Surface,
        font_distance_field: &glium::texture::Texture2d,
        mesh: &TextMesh,
        fill: &(f32, f32, f32, f32),
        transform_matrix: &[[f32; 4]; 4],
        draw_parameters: &DrawParameters,
    )
        -> Result<(), DrawError>
    {
        surface.draw(
            &mesh.vertices,
            &mesh.indices,
            &self.program,

            uniform! {
                fill: fill,
                transform: transform_matrix,
                distance_field: font_distance_field,
            },

            draw_parameters
        )
    }
}

impl TextMesh {
    pub fn new(facade: &impl Facade, font: &Font, text: &str) -> Result<Self, TextMeshCreationError> {
        let (vertices, indices, width) = TextMesh::compute_buffers(font, text);

        Ok(TextMesh {
            vertices: glium::VertexBuffer::new(facade, &vertices)
                .map_err(|e| TextMeshCreationError::Vertex(e))?,

            indices: glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &indices)
                .map_err(|e| TextMeshCreationError::Index(e))?,

            width
        })
    }

    pub fn set(&mut self, font: &Font, text: &str){
        let (vertices, indices, width) = TextMesh::compute_buffers(font, text);
        self.vertices.write(&vertices);
        self.indices.write(&indices);
        self.width = width;
    }

    pub fn compute_buffers(font: &Font, text: &str) -> (Vec<GlyphQuadVertex>, Vec<u16>, f32) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut width = 0.0;

        for glyph in font.layout_glyphs(text) {
            let quad_positions = glyph.layout.in_mesh.vertices();
            let quad_texture_coords = glyph.layout.in_atlas.vertices();

            for quad_vertex_index in 0..4 {
                for triangle_index in [ 0,1,2,  2,3,0 ] {
                    indices.push((vertices.len() + triangle_index) as u16);
                }

                width = glyph.layout.in_mesh.right();
                vertices.push(GlyphQuadVertex {
                    position: quad_positions[quad_vertex_index],
                    texture_coordinate: quad_texture_coords[quad_vertex_index]
                });
            }
        }

        (vertices, indices, width)
    }

    pub fn vertices(&self) -> &glium::VertexBuffer<GlyphQuadVertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &glium::IndexBuffer<u16> {
        &self.indices
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}