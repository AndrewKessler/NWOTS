use crate::assets::Texture;

pub fn render_viewmodel(
    frame: &mut [u8],
    texture: &Texture,
    screen_x: i32,
    screen_y: i32,
    scale: f32,
) {

    let width =
        (texture.width as f32
            * scale)
            as i32;

    let height =
        (texture.height as f32
            * scale)
            as i32;

    for y in 0..height {

        for x in 0..width {

            let tex_x =
                ((x as f32
                    / width as f32)
                    * texture.width as f32)
                    as usize;

            let tex_y =
                ((y as f32
                    / height as f32)
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

            let dst_x =
                screen_x + x;

            let dst_y =
                screen_y + y;

            if dst_x < 0
                || dst_x >= 640
                || dst_y < 0
                || dst_y >= 480
            {
                continue;
            }

            let idx =
                (
                    dst_y as usize
                        * 640
                    +
                    dst_x as usize
                ) * 4;

            frame[idx] =
                color[0];

            frame[idx + 1] =
                color[1];

            frame[idx + 2] =
                color[2];

            frame[idx + 3] =
                255;
        }
    }
}