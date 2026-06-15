use glam::Vec2;

use crate::world::Sector;
use crate::sprites::SpriteInstance;
use crate::world::ExitTrigger;

pub struct Map {

    pub sectors: Vec<Sector>,

    pub spawn: Vec2,

    pub spawn_angle: f32,

    pub items: Vec<SpriteInstance>,

    pub exits: Vec<ExitTrigger>,
}