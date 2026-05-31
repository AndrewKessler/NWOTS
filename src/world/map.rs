use glam::Vec2;

use crate::world::Sector;

pub struct Map {
    pub sectors: Vec<Sector>,
    pub spawn: Vec2,
    pub spawn_angle: f32,
}