use crate::rectangle::Rectangle;
use std::io::{ Read, Write };
pub use hashbrown::HashMap;
use crate::text::LayoutGlyphs;
use serde::{ Serialize, Deserialize };
use lz4_compression::prelude::{ decompress, compress };

pub struct Font {
    pub glyphs: HashMap<char, GlyphLayout>,
    pub kerning: HashMap<(char, char), f32>,
    pub layout: FontLayout,
    pub atlas: Atlas,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedFont {
    pub glyphs: Vec<(char, GlyphLayout)>,
    pub kerning: Vec<((char, char), f32)>,
    pub layout: FontLayout,
    pub atlas: Atlas,
}

#[derive(Serialize, Deserialize)]
pub struct FontLayout {
    pub advance_y: f32,
    pub ascent: f32,
    pub descent: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Atlas {
    pub resolution: (usize, usize),
    pub distance_field: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GlyphLayout {
    /// ' ' and '\t' will not have any geometry, but an advance_x
    pub quad: Option<GlyphQuad>,
    pub advance_x: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GlyphQuad {
    pub geometry: Rectangle,
    pub texture: Rectangle,
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Decompress(lz4_compression::decompress::Error),
    Bincode(bincode::Error)
}


impl Font {
    pub fn layout_glyphs<S>(&self, chars: S) -> LayoutGlyphs<S> where S: Iterator<Item=char> {
        LayoutGlyphs::new(self, chars)
    }

    pub fn deserialized(serialized: SerializedFont) -> Self {
        Font {
            atlas: serialized.atlas,
            glyphs: serialized.glyphs.into_iter().collect(),
            kerning: serialized.kerning.into_iter().collect(),
            layout: serialized.layout
        }
    }

    pub fn serialized(self) -> SerializedFont {
        SerializedFont {
            atlas: self.atlas,
            glyphs: self.glyphs.into_iter().collect(),
            kerning: self.kerning.into_iter().collect(),
            layout: self.layout
        }
    }

    pub fn read(reader: impl Read) -> Result<Self, Error> {
        Ok(Self::deserialized(SerializedFont::read(reader)?))
    }

    pub fn write(self, writer: impl Write) -> Result<(), Error> {
        Ok(self.serialized().write(writer)?)
    }
}

impl SerializedFont {
    pub fn read(mut reader: impl Read) -> Result<Self, Error> {
        let mut compressed = Vec::with_capacity(2048);
        reader.read_to_end(&mut compressed)?;

        // TODO without conversion between readers and byte arrays? ...
        let decompressed = decompress(&compressed)?;

        Ok(Self::read_uncompressed(decompressed.as_slice())?)
    }

    pub fn write(&self, mut writer: impl Write) -> Result<(), Error> {
        let mut uncompressed = Vec::with_capacity(2048);
        self.write_uncompressed(&mut uncompressed)?;

        // TODO without conversion between readers and byte arrays? ...
        let compressed = compress(&uncompressed);

        writer.write_all(&compressed)?;
        Ok(())
    }

    pub fn read_uncompressed(reader: impl Read) -> bincode::Result<Self> {
        bincode::deserialize_from(reader)
    }

    pub fn write_uncompressed(&self, writer: impl Write) -> bincode::Result<()> {
        bincode::serialize_into(writer, self)
    }
}


use std::convert::From;

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Self {
        Error::Bincode(error)
    }
}

impl From<lz4_compression::decompress::Error> for Error {
    fn from(error: lz4_compression::decompress::Error) -> Self {
        Error::Decompress(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}
