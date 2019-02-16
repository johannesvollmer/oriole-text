use crate::atlas::{ Atlas, SerializedAtlas };
use crate::rectangle::Rectangle;
use std::io::{ Read, Write };
pub use hashbrown::HashMap;
use crate::text::LayoutGlyphs;
use serde::{ Serialize, Deserialize };

pub struct Font {
    pub atlas: Atlas,
    pub glyphs: HashMap<char, GlyphLayout>,
    pub kerning: HashMap<(char, char), f32>,
    pub layout: FontLayout
}

#[derive(Serialize, Deserialize)]
pub struct SerializedFont {
    pub atlas: SerializedAtlas,
    pub glyphs: Vec<(char, GlyphLayout)>,
    pub kerning: Vec<((char, char), f32)>,
    pub layout: FontLayout
}

#[derive(Serialize, Deserialize)]
pub struct FontLayout {
    pub advance_y: f32,
//    pub space_advance_x: f32,
//    pub tab_advance_x: f32,
    pub ascent: f32,
    pub descent: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GlyphLayout {
    pub bounds: Rectangle,
    pub advance_x: f32,
}


impl Font {

    pub fn layout_glyphs<S>(&self, chars: S) -> LayoutGlyphs<S> where S: Iterator<Item=char> {
        LayoutGlyphs::new(self, chars)
    }



    pub fn read(reader: impl Read) -> Option<Self> {
        let mut uncompressed = Vec::with_capacity(2048);
        compress::lz4::Decoder::new(reader).read_to_end(&mut uncompressed).ok()?;
        Self::read_uncompressed(uncompressed.as_slice()).ok()
    }

    pub fn write(self, writer: impl Write) -> Option<()> {
        let mut compressed = Vec::with_capacity(2048);
        self.write_uncompressed(&mut compressed).ok()?;
        compress::lz4::Encoder::new(writer).write_all(&compressed).ok()
    }

    pub fn read_uncompressed(reader: impl Read) -> bincode::Result<Self> {
        bincode::deserialize_from(reader).map(|s| Self::deserialized(s))
    }

    pub fn write_uncompressed(self, writer: impl Write) -> bincode::Result<()> {
        bincode::serialize_into(writer, &self.serialized())
    }

    pub fn deserialized(serialized: SerializedFont) -> Self {
        Font {
            atlas: Atlas::deserialized(serialized.atlas),
            glyphs: serialized.glyphs.into_iter().collect(),
            kerning: serialized.kerning.into_iter().collect(),
            layout: serialized.layout
        }
    }

    pub fn serialized(self) -> SerializedFont {
        SerializedFont {
            atlas: self.atlas.serialized(),
            glyphs: self.glyphs.into_iter().collect(),
            kerning: self.kerning.into_iter().collect(),
            layout: self.layout
        }
    }

}

