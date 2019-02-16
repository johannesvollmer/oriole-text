use hashbrown::HashMap;
use serde::{ Serialize, Deserialize };


pub struct Atlas<I, K, V>
    where I: ImageData
{
    elements: HashMap<K, (Rectangle, V)>,
    image: I,
    width: usize,
    height: usize,
}

pub trait ImageData: Serialize + Deserialize {}
impl ImageData for &'_ [u8] {}
impl ImageData for Vec<u8> {}


#[derive(Serialize, Deserialize)]
pub struct SerializedAtlas<I, K, V> where I: ImageData {
    elements: Vec<(K, (Rectangle, V))>,
    image: I,
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}



use std::io::{ Read, Write, Result };

impl<I, K, V> Atlas<I, K, V> {

    pub fn read(reader: &mut impl Read) -> Result<Self> {
        let mut uncompressed = Vec::with_capacity(2014);
        compress::lz4::Decoder::new(reader).read_to_end(uncompressed)?;
        Self::read_uncompressed(&uncompressed)
    }

    pub fn write(self, writer: &mut impl Write) -> Result<()> {
        let mut compressed = Vec::with_capacity(2014);
        self.write_uncompressed(&mut compressed)?;
        compress::lz4::Encoder::new(writer).write_all(&compressed)
    }

    pub fn read_uncompressed(reader: &mut impl Read) -> bincode::Result<Self> {
        bincode::deserialize_from(&bytes).map(|s| Self::deserialized(s))
    }

    pub fn write_uncompressed(self, writer: impl Write) -> bincode::Result<()> {
        bincode::serialize_into(writer, self.serialized())
    }

    pub fn deserialized(serialized: SerializedAtlas<I, K, V>) -> Self {
        Atlas {
            elements: serialized.elements.iter().collect(),
            image: serialized.image,
            width: serialized.width,
            height: serialized.height
        }
    }

    pub fn serialized(self) -> SerializedAtlas<I, K, V> {
        SerializedAtlas {
            elements: self.elements.iter().collect(),
            image: self.image,
            width: self.width,
            height: self.height
        }
    }

}

