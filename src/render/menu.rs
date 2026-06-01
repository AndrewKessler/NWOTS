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

    // ============================================================
    // BACKGROUND
    // ============================================================

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

    // ============================================================
    // TITLE
    // ============================================================

    draw_text(
        frame,
        font,
        "NWOTS",
        240,
        40,
        50.0,
        [255, 255, 0],
    );

    // ============================================================
    // EPISODE TITLE
    // ============================================================

    draw_text(
        frame,
        font,
        &config.episode[0].title,
        180,
        120,
        36.0,
        [255, 255, 255],
    );

    // ============================================================
    // MENU ITEMS
    // ============================================================

    let start_color =
        if menu_index == 0 {
            [255,255,0]
        } else {
            [255,255,255]
        };

    let save_color =
        if menu_index == 1 {
            [255,255,0]
        } else {
            [255,255,255]
        };

    let load_color =
        if menu_index == 2 {
            [255,255,0]
        } else {
            [255,255,255]
        };

    let exit_color =
        if menu_index == 3 {
            [255,255,0]
        } else {
            [255,255,255]
        };

    draw_text(
        frame,
        font,
        &config.menu.start_message,
        180,
        220,
        30.0,
        start_color,
    );

    draw_text(
        frame,
        font,
        &config.menu.save_message,
        180,
        270,
        30.0,
        save_color,
    );

    draw_text(
        frame,
        font,
        &config.menu.load_message,
        180,
        320,
        30.0,
        load_color,
    );

    draw_text(
        frame,
        font,
        &config.menu.exit_message,
        180,
        370,
        30.0,
        exit_color,
    );
}