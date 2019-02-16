use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Rectangle {
    pub position: (f32, f32),
    pub dimensions: (f32, f32),
}

impl Rectangle {

    pub fn right(&self) -> f32 {
        self.position.0 + self.dimensions.0
    }

    pub fn left(&self) -> f32 {
        self.position.0
    }

    pub fn top(&self) -> f32 {
        self.position.1 + self.dimensions.1
    }

    pub fn bottom(&self) -> f32 {
        self.position.1
    }


    pub fn translated(&self, movement: (f32, f32)) -> Self {
        Rectangle {
            position: (self.position.0 + movement.0, self.position.1 + movement.1),
            dimensions: self.dimensions,
        }
    }

    /// [ bottom left, top left, top right, bottom right ]
    pub fn vertices(&self) -> [(f32, f32); 4] {
        [
            (self.left(), self.bottom()),
            (self.left(), self.top()),
            (self.right(), self.top()),
            (self.right(), self.bottom()),
        ]
    }

}