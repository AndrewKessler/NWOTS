use glam::Vec2;

use crate::world::{
    Sector,
    Wall,
};

pub struct RayHit<'a> {
    pub distance: f32,
    pub point: Vec2,
    pub wall: &'a Wall,
    pub sector: &'a Sector,
}