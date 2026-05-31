use fontdue::Font;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub fn draw_text(
    frame: &mut [u8],
    font: &Font,
    text: &str,
    start_x: i32,
    start_y: i32,
    scale: f32,
    color: [u8; 3],
) {

    let mut pen_x =
        start_x;

    for ch in text.chars() {

        let (metrics, bitmap) =
            font.rasterize(ch, scale);

        for y in 0..metrics.height {

            for x in 0..metrics.width {

                let alpha =
                    bitmap[y * metrics.width + x];

                if alpha == 0 {
                    continue;
                }

                let px =
                    pen_x + x as i32;

                let py =
                    start_y + y as i32;

                if px < 0
                    || py < 0
                    || px >= WIDTH as i32
                    || py >= HEIGHT as i32
                {
                    continue;
                }

                let idx =
                    ((py as usize * WIDTH as usize)
                        + px as usize)
                        * 4;

                frame[idx] = color[0];
                frame[idx + 1] = color[1];
                frame[idx + 2] = color[2];
                frame[idx + 3] = 255;
            }
        }

        pen_x +=
            metrics.advance_width as i32;
    }
}