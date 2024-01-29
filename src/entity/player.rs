use crate::entity::*;

pub struct Player {
    pub name: String,
    pub position: Vec3,
    pub health: health::Health,
}

// Movement implementation
