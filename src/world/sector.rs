use crate::world::Wall;

pub struct Sector {
    pub name: String,
    pub floor_texture: String,
    pub ceiling_texture: String,
    pub walls: Vec<Wall>,
}