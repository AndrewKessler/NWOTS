use glam::Vec2;

#[derive(Clone)]
pub enum WallType {
    Solid,
    Portal(String),
}

#[derive(Clone)]
pub struct Wall {
    pub start: Vec2,
    pub end: Vec2,
    pub texture: String,
    pub wall_type: WallType,
}