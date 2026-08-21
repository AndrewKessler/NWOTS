use std::collections::HashMap;

use crate::sprites::{
    SpriteDirection,
    SpriteFrame,
};

pub struct SpriteAnimation {

    pub frame_duration: f32,

    pub ground_offset: f32,

    pub frames:
    HashMap<
        SpriteDirection,
        Vec<SpriteFrame>,
    >,
}

pub struct SpriteDefinition {

    pub name: String,

    pub radius: f32,

    pub height: f32,

    pub ground_offset: f32,

    pub scale_x: f32,

    pub scale_y: f32,

    pub health: f32,

    pub speed: f32,

    pub animations:
        HashMap<
            String,
            SpriteAnimation,
        >,

}