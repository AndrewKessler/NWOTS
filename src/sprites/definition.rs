use std::collections::HashMap;

use crate::sprites::{
    SpriteDirection,
    SpriteFrame,
};

pub struct SpriteDefinition {

    pub name: String,

    pub radius: f32,

    pub height: f32,

    pub scale_x: f32,
    pub scale_y: f32,

    pub frames:
        HashMap<
            SpriteDirection,
            SpriteFrame,
        >,
}