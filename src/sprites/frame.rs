use crate::assets::Texture;

pub struct SpriteFrame {
    pub image: Texture,

    pub offset_x: i32,
    pub offset_y: i32,

    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
}