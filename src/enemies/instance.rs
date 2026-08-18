use glam::Vec2;

pub struct EnemyInstance {

    pub enemy_id: String,

    pub position: Vec2,

    pub angle: f32,

    pub animation: String,

    pub animation_frame: usize,

    pub animation_timer: f32,

    pub speed: f32,
}