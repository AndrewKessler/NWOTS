use crate::assets::Texture;

pub fn render_hud(
    frame: &mut [u8],
    hud: &Texture,
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
}