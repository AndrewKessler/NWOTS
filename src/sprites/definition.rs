use std::collections::HashMap;

use crate::sprites::{
    SpriteDirection,
    SpriteFrame,
};

pub struct SpriteDefinition {
    pub name: String,

    pub default_scale_x: f32,
    pub default_scale_y: f32,

    pub frames:
        HashMap<
            SpriteDirection,
            SpriteFrame,
        >,
}