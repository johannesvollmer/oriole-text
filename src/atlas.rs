use hashbrown::HashMap;
use serde::{ Serialize, Deserialize };
use crate::rectangle::Rectangle;

pub struct Atlas {
    pub glyphs: HashMap<char, Rectangle>,
    pub resolution: (usize, usize),
    pub distance_field: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedAtlas {
    glyphs: Vec<(char, Rectangle)>,
    distance_field: Vec<u8>,
    resolution: (usize, usize)
}

impl Atlas {
    pub fn deserialized(serialized: SerializedAtlas) -> Self {
        Atlas {
            glyphs: serialized.glyphs.into_iter().collect(),
            resolution: serialized.resolution,
            distance_field: serialized.distance_field,
        }
    }

    pub fn serialized(self) -> SerializedAtlas {
        SerializedAtlas {
            glyphs: self.glyphs.into_iter().collect(),
            resolution: self.resolution,
            distance_field: self.distance_field,
        }
    }
}

