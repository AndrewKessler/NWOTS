use crate::assets::Texture;
use crate::util::constants::{
    HEIGHT,
    TEXTURE_SIZE,
};

pub fn render_wall_column(
    frame: &mut [u8],
    x: usize,
    wall_top: i32,
    wall_bottom: i32,
    wall_height: f32,
    texture_x: usize,
    wall_texture: &Texture,
) {

    for y in wall_top..wall_bottom {

        if y < 0
            || y >= HEIGHT as i32
        {
            continue;
        }

        let wall_y =
            y as f32
                - wall_top as f32;

        let texture_y =
            ((wall_y / wall_height)
                * TEXTURE_SIZE as f32)
                .clamp(0.0, 63.0)
                as usize;

        let color =
            wall_texture.sample(
                texture_x,
                texture_y,
            );

        let idx =
            ((y as usize
                * crate::util::constants::WIDTH as usize)
                + x)
                * 4;

        frame[idx] = color[0];
        frame[idx + 1] = color[1];
        frame[idx + 2] = color[2];
        frame[idx + 3] = 255;
    }
}