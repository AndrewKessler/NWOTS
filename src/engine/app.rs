use fontdue::Font;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;

use crate::engine::GameState;
use crate::sprites::SpriteRegistry;
use crate::sprites::render_sprites;
use crate::assets::Texture;
use crate::render::render_menu;

use pixels::{
    Pixels,
    SurfaceTexture,
};

use winit::{
    dpi::LogicalSize,
    event::{
        DeviceEvent,
        ElementState,
        Event,
        MouseButton,
        WindowEvent,
    },
    event_loop::EventLoop,
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
    window::WindowBuilder,
};

use crate::assets::TextureRegistry;
use crate::config::GameConfig;
use crate::map_loader::load_map;
use crate::physics::update_player;
use crate::render::render_world;
use crate::util::constants::{
    HEIGHT,
    WIDTH,
};
use crate::world::Player;

pub struct App;

impl App {

    pub fn run() {

        println!("================================");
        println!("NWOTS Engine Starting");
        println!("================================");

        let config: GameConfig =
            toml::from_str(
                &fs::read_to_string(
                    "config/game.toml"
                )
                .expect(
                    "Failed to load config/game.toml"
                ),
            )
            .expect(
                "Failed to parse game.toml"
            );

        let font_bytes =
            fs::read(
                &config.menu.font
            )
            .unwrap();

        let menu_font =
            Font::from_bytes(
                font_bytes,
                fontdue::FontSettings::default(),
            )
            .unwrap();

let menu_background =
    Texture::load(
        &config.menu.background
    );

        let textures =
            TextureRegistry::load(
                "config/textures.txt"
            );

        let mut sprite_registry =
            SpriteRegistry::new();

            sprite_registry
                .load_test_assets();

        let map =
            load_map(
                &config
                    .episode[0]
                    .maps[0]
                    .file
            );

        let mut player =
            Player {

                position:
                    map.spawn,

                angle:
                    map.spawn_angle,

                pitch:
                    0.0,
            };

        let mut keys =
            HashSet::<KeyCode>::new();

        let mut game_state =
            GameState::Menu;

        let mut menu_index =
            0usize;
        
        let mut right_mouse =
            false;

        let event_loop =
            EventLoop::new()
                .unwrap();

        let window =
            Arc::new(
                WindowBuilder::new()
                    .with_title(
                        "NWOTS"
                    )
                    .with_inner_size(
                        LogicalSize::new(
                            WIDTH,
                            HEIGHT,
                        ),
                    )
                    .build(
                        &event_loop
                    )
                    .unwrap()
            );

        let surface_texture =
            SurfaceTexture::new(
                WIDTH,
                HEIGHT,
                window.clone(),
            );

        let mut pixels =
            Pixels::new(
                WIDTH,
                HEIGHT,
                surface_texture,
            )
            .unwrap();

        event_loop
    .run(
        move |event, target| {

            match event {

                Event::DeviceEvent {

                    event:
                        DeviceEvent::MouseMotion {

                            delta,
                        },

                    ..
                } => {

                    if game_state
                        ==
                        GameState::Playing
                        &&
                        right_mouse
                    {

                        player.angle +=
                            delta.0 as f32
                                * 0.003;

                        player.pitch +=
                            delta.1 as f32
                                * 0.50;

                        player.pitch =
                            player.pitch.clamp(
                                -80.0,
                                80.0,
                            );
                    }
                }

                Event::WindowEvent {

                    event:
                        WindowEvent::CloseRequested,

                    ..
                } => {

                    target.exit();
                }

                Event::WindowEvent {

                    event:
                        WindowEvent::MouseInput {

                            state,

                            button,

                            ..
                        },

                    ..
                } => {

                    if button
                        ==
                        MouseButton::Right
                    {

                        right_mouse =
                            state
                                ==
                                ElementState::Pressed;
                    }
                }

                Event::WindowEvent {

                    event:
                        WindowEvent::KeyboardInput {

                            event,

                            ..
                        },

                    ..
                } => {

                    if let PhysicalKey::Code(
                        keycode
                    ) =
                        event.physical_key
                    {

                        match event.state {

                            ElementState::Pressed => {

                                keys.insert(
                                    keycode
                                );

                                if game_state
                                    ==
                                    GameState::Menu
                                {

                                    match keycode {

                                        KeyCode::ArrowUp => {

                                            if menu_index > 0 {

                                                menu_index -= 1;
                                            }
                                        }

                                        KeyCode::ArrowDown => {

                                            if menu_index < 3 {

                                                menu_index += 1;
                                            }
                                        }

                                        KeyCode::Enter => {

                                            match menu_index {

                                                0 => {

                                                    println!(
                                                        "Starting game..."
                                                    );

                                                    game_state =
                                                        GameState::Playing;
                                                }

                                                1 => {

                                                    println!(
                                                        "Save Game placeholder"
                                                    );
                                                }

                                                2 => {

                                                    println!(
                                                        "Load Game placeholder"
                                                    );
                                                }

                                                3 => {

                                                    game_state =
                                                        GameState::Exit;
                                                }

                                                _ => {}
                                            }
                                        }

                                        _ => {}
                                    }
                                }

                                else {

                                    match keycode {

                                        KeyCode::Escape => {

                                            game_state =
                                                GameState::Menu;
                                        }

                                        _ => {}
                                    }
                                }
                            }

                            ElementState::Released => {

                                keys.remove(
                                    &keycode
                                );
                            }
                        }
                    }
                }

                Event::AboutToWait => {

                    if game_state
                        ==
                        GameState::Playing
                    {

                        update_player(
                            &mut player,
                            &keys,
                            &map,
                        );

                        // Keep arrow turning
                        // until mouse look is verified

                        if keys.contains(
                            &KeyCode::ArrowLeft
                        ) {

                            player.angle -=
                                0.05;
                        }

                        if keys.contains(
                            &KeyCode::ArrowRight
                        ) {

                            player.angle +=
                                0.05;
                        }
                    }

                    window
                        .request_redraw();
                }

                Event::WindowEvent {

                    event:
                        WindowEvent::RedrawRequested,

                    ..
                } => {

                    let frame =
                        pixels.frame_mut();

                    match game_state {

                        GameState::Menu => {

                            render_menu(
                                frame,
                                &menu_background,
                                &menu_font,
                                &config,
                                menu_index,
                            );
                        }

                        GameState::Playing => {

                            let zbuffer =
                                render_world(
                                    frame,
                                    &player,
                                    &map,
                                    &textures.textures,
                                );

                            render_sprites(
                                frame,
                                &player,
                                &map,
                                &sprite_registry,
                                &zbuffer,
                            );
                        }

                        GameState::Exit => {

                            target.exit();
                        }
                    }

                    pixels
                        .render()
                        .unwrap();
                }

                _ => {}
            }
        }
    )
    .unwrap();
    }
}