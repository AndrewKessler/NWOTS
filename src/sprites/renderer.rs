use glam::Vec2;

use crate::assets::Texture;
use crate::sprites::{
    SpriteDirection,
    SpriteRegistry,
};
use crate::util::constants::{
    WIDTH,
    HEIGHT,
    FOV,
};
use crate::world::{
    Map,
    Player,
};

pub fn render_sprites(
    frame: &mut [u8],
    player: &Player,
    map: &Map,
    registry: &SpriteRegistry,
    zbuffer: &[f32],
) {

    for item in &map.items {

        let definition =
            match registry.get(
                &item.sprite_id
            ) {

                Some(def) => def,

                None => continue,
            };

        let sprite_frame =
            match definition.frames.get(
                &SpriteDirection::F
            ) {

                Some(frame) => frame,

                None => continue,
            };

        render_sprite(
            frame,
            player,
            item.position,
            &sprite_frame.image,
            definition.height,
            definition.scale_x,
            definition.scale_y,
            sprite_frame.offset_x,
            sprite_frame.offset_y,
            zbuffer,
        );
    }
}

fn render_sprite(
    frame: &mut [u8],
    player: &Player,
    sprite_pos: Vec2,
    texture: &Texture,
    world_height: f32,
    scale_x: f32,
    scale_y: f32,
    offset_x: i32,
    offset_y: i32,
    zbuffer: &[f32],
) {

    let dx =
        sprite_pos.x
            - player.position.x;

    let dy =
        sprite_pos.y
            - player.position.y;

    let distance =
        (dx * dx + dy * dy)
            .sqrt();

    if distance < 1.0 {
        return;
    }

    let angle_to_sprite =
        dy.atan2(dx);

    let mut relative_angle =
        angle_to_sprite
            - player.angle;

    while relative_angle
        > std::f32::consts::PI
    {
        relative_angle -=
            std::f32::consts::TAU;
    }

    while relative_angle
        < -std::f32::consts::PI
    {
        relative_angle +=
            std::f32::consts::TAU;
    }

    if relative_angle.abs()
        > FOV / 2.0
    {
        return;
    }

    let screen_x =
        ((relative_angle + FOV / 2.0)
            / FOV)
            * WIDTH as f32;

    let sprite_height =
        ((HEIGHT as f32
            * world_height)
            / distance)
            * scale_y;

    let sprite_width =
        (sprite_height
            * texture.width as f32
            / texture.height as f32)
            * scale_x;

    let left =
        (screen_x
            - sprite_width / 2.0)
            as i32
            + offset_x;

    let top =
        ((HEIGHT as f32
            - sprite_height)
            / 2.0
            + player.pitch)
            as i32
            + offset_y;

    for sx in 0..sprite_width as i32 {

        let screen_col =
            left + sx;

        if screen_col < 0
            || screen_col
                >= WIDTH as i32
        {
            continue;
        }

        let col =
            screen_col as usize;

        if distance
            >= zbuffer[col]
        {
            continue;
        }

        let tex_x =
            ((sx as f32
                / sprite_width)
                * texture.width as f32)
                as usize;

        for sy in 0..sprite_height as i32 {

            let screen_row =
                top + sy;

            if screen_row < 0
                || screen_row
                    >= HEIGHT as i32
            {
                continue;
            }

            let tex_y =
                ((sy as f32
                    / sprite_height)
                    * texture.height as f32)
                    as usize;

            let color =
                texture.sample(
                    tex_x,
                    tex_y,
                );

            if color[3] == 0 {
                continue;
            }

            let idx =
                ((screen_row as usize
                    * WIDTH as usize)
                    + col)
                    * 4;

            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }
    }
}