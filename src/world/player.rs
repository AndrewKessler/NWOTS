use glam::Vec2;

use crate::weapons::WeaponState;
use crate::world::{
    Inventory,
    PlayerStats,
};

pub struct Player {

    pub position: Vec2,

    pub angle: f32,

    pub pitch: f32,

    pub stats: PlayerStats,

    pub inventory: Inventory,

    pub weapon_state: WeaponState,

    pub weapon_timer: f32,
}

impl Player {

    pub fn new(
        position: Vec2,
        angle: f32,
    ) -> Self {

        Self {

            position,

            angle,

            pitch: 0.0,

            stats:
                PlayerStats::default(),

            inventory:
                Inventory::default(),

            weapon_state:
                WeaponState::Idle,

            weapon_timer:
                0.0,
        }
    }
}