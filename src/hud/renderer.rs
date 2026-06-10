use crate::assets::Texture;
use crate::world::Player;

pub fn render_hud(
    frame: &mut [u8],
    hud: &Texture,
    colt_icon: &Texture,
    player: &Player,
) {

    let width =
        hud.width;

    let height =
        hud.height;

    for y in 0..height {

        for x in 0..width {

            let color =
                hud.sample(
                    x,
                    y,
                );

            let alpha =
                color[3] as f32
                    / 255.0;

            if alpha <= 0.0 {
                continue;
            }

            let idx =
                (y * width + x) * 4;

            let dst_r =
                frame[idx] as f32;

            let dst_g =
                frame[idx + 1] as f32;

            let dst_b =
                frame[idx + 2] as f32;

            frame[idx] =
                (
                    color[0] as f32
                        * alpha
                    +
                    dst_r
                        * (1.0 - alpha)
                ) as u8;

            frame[idx + 1] =
                (
                    color[1] as f32
                        * alpha
                    +
                    dst_g
                        * (1.0 - alpha)
                ) as u8;

            frame[idx + 2] =
                (
                    color[2] as f32
                        * alpha
                    +
                    dst_b
                        * (1.0 - alpha)
                ) as u8;

            frame[idx + 3] = 255;
        }
    }

            if player
        .inventory
        .has_item(
            "colt"
        )
    {

        draw_icon(
            frame,
            colt_icon,
            158,
            435,
            32,
        );
    }
}

fn draw_icon(
    frame: &mut [u8],
    icon: &Texture,
    screen_x: usize,
    screen_y: usize,
    size: usize,
) {

    for y in 0..size {

        for x in 0..size {

            let tex_x =
                x
                    * icon.width
                    / size;

            let tex_y =
                y
                    * icon.height
                    / size;

            let color =
                icon.sample(
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

            if dst_x >= 640
                || dst_y >= 480
            {

                continue;
            }

            let idx =
                (
                    dst_y * 640
                    + dst_x
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