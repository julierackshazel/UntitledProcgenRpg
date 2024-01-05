mod health;
use bevy::prelude::*;

pub trait Entity {
    fn name() -> String;
    fn translate(dir: Vec2);
}
