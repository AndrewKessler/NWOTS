use fontdue::Font;
use glam::Vec2;

use crate::render::text::draw_text;
use crate::util::constants::{
    WIDTH,
    FOV,
};
use crate::world::Player;

pub fn draw_world_text(
    frame: &mut [u8],
    font: &Font,
    text: &str,
    world_pos: Vec2,
    player: &Player,
) {

    let dx =
        world_pos.x
            - player.position.x;

    let dy =
        world_pos.y
            - player.position.y;

    let angle_to_text =
        dy.atan2(dx);

    let mut relative_angle =
        angle_to_text
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

    draw_text(
        frame,
        font,
        text,
        screen_x as i32 - 80,
        120,
        20.0,
        [255, 255, 0],
    );
}