use bevy::prelude::*;

pub struct Entity {
    pub name: String,
    pub position: Vec2,
}

impl Entity {
   pub fn new(name: String, position: Vec2) -> Entity {
       Entity { name, position }
   }
}
