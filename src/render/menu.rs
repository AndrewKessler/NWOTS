use fontdue::Font;

use crate::assets::Texture;
use crate::config::GameConfig;
use crate::render::text::draw_text;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub fn render_menu(
    frame: &mut [u8],
    background: &Texture,
    font: &Font,
    config: &GameConfig,
    menu_index: usize,
) {

    for y in 0..HEIGHT as usize {

        for x in 0..WIDTH as usize {

            let idx =
                (y * WIDTH as usize + x)
                    * 4;

            let color =
                background.sample(x, y);

            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }
    }

    draw_text(
        frame,
        font,
        "SELECT EPISODE",
        120,
        80,
        40.0,
        [255,255,0],
    );

    let title_color =
        if menu_index == 0 {
            [255,255,0]
        }
        else {
            [255,255,255]
        };

    let exit_color =
        if menu_index == 1 {
            [255,255,0]
        }
        else {
            [255,255,255]
        };

    draw_text(
        frame,
        font,
        &config.episode[0].title,
        180,
        220,
        30.0,
        title_color,
    );

    draw_text(
        frame,
        font,
        &config.menu.exit_message,
        180,
        280,
        30.0,
        exit_color,
    );
}