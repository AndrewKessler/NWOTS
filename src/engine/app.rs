use fontdue::Font;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;

use crate::engine::GameState;
use crate::audio::AudioManager;
use crate::sprites::SpriteRegistry;
use crate::sprites::render_sprites;
use crate::assets::Texture;
use crate::render::render_menu;
use crate::hud::render_hud;
use crate::gameplay::pickup_items;
use crate::weapons::
    render_viewmodel;

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

fn set_game_state(
    game_state: &mut GameState,
    new_state: GameState,
    audio: &mut AudioManager,
    config: &GameConfig,
) {

    if *game_state == new_state {

        return;
    }

    match new_state {

        GameState::Cutscene => {

            if let Some(
                cutscene
            ) =
                &config.cutscene
            {

                if !cutscene
                    .music
                    .trim()
                    .is_empty()
                {

                    audio.play_music(
                        &cutscene.music
                    );

                } else {

                    audio.stop_music();
                }
            }
        }

        GameState::Menu => {

            audio.play_music(
                &config.menu.music
            );
        }

        GameState::Playing => {

            audio.play_music(
                &config
                    .episode[0]
                    .maps[0]
                    .music
            );
        }

        GameState::Exit => {}
    }

    *game_state =
        new_state;
}

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
            .load_registry(
                "config/items.txt"
            );

        let mut weapon_registry =
            crate::weapons::
                WeaponRegistry::new();

        weapon_registry
            .load_registry(
                "config/weapons.txt"
            );

        let colt_weapon =
            weapon_registry
                .get("colt")
                .unwrap()
                .clone();

        let mut audio =
            AudioManager::new();

        let hud_texture =
            Texture::load(
                "assets/hud/default/hud.png"
            );

        let colt_icon =
            Texture::load(
                "assets/items/weapons/colt/icon.png"
            );

        let colt_idle =
            Texture::load(
                "assets/items/weapons/colt/idle_0.png"
            );

        let colt_fire_0 =
            Texture::load(
                "assets/items/weapons/colt/fire_0.png"
            );

        let colt_fire_1 =
            Texture::load(
                "assets/items/weapons/colt/fire_1.png"
            );

        let colt_fire_2 =
            Texture::load(
                "assets/items/weapons/colt/fire_2.png"
            );

        let colt_fire_3 =
            Texture::load(
                "assets/items/weapons/colt/fire_3.png"
            );

        let mut cutscene_player =

        if let Some(
            cutscene
        ) =
            &config.cutscene
        {

            if cutscene
                .path
                .trim()
                .is_empty()
            {

                None

            } else {

                Some(

                    crate::cutscene::
                        CutscenePlayer::new(
                            &cutscene.path,
                            cutscene.fps,
                        )
                )
            }

        } else {

            None
        };

        let mut map =
            load_map(
                &config
                    .episode[0]
                    .maps[0]
                    .file
            );

        let mut player =
            Player::new(
                map.spawn,
                map.spawn_angle,
            );


        let mut keys =
            HashSet::<KeyCode>::new();

        let mut game_state =

        if let Some(cutscene) =
            &config.cutscene
        {

            if cutscene
                .path
                .trim()
                .is_empty()
            {

                GameState::Menu

            } else {

                GameState::Cutscene
            }

        } else {

            GameState::Menu
        };

        if game_state
            ==
            GameState::Menu
        {

                audio.play_music(
                    &config.menu.music
                );
            }

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

                    if button
                        ==
                        MouseButton::Left
                        &&
                        state
                            ==
                            ElementState::Pressed
                    {

                        if game_state
                            ==
                            GameState::Playing
                        {

                            if player
                                .inventory
                                .equipped_weapon
                                .is_some()
                                &&
                                player
                                    .weapon_state
                                    ==
                                    crate::weapons::
                                        WeaponState::Idle
                            {

                                if player
                                    .stats
                                    .ammo
                                    > 0
                                {

                                    player
                                        .stats
                                        .ammo
                                        -= 1;

                                    player.weapon_state =
                                        crate::weapons::
                                            WeaponState::Firing;

                                    audio.play_sound(
                                        &colt_weapon
                                            .fire_sound
                                    );

                                    println!(
                                        "Bang!"
                                    );
                                }

                                else {

                                    println!(
                                        "Click!"
                                    );
                                }
                            }
                        }
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
                                    GameState::Cutscene
                                {

                                    if keycode
                                        ==
                                        KeyCode::Space
                                    {

                                        set_game_state(
                                            &mut game_state,
                                            GameState::Menu,
                                            &mut audio,
                                            &config,
                                        );
                                    }
                                }

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

                                                    set_game_state(
                                                        &mut game_state,
                                                        GameState::Playing,
                                                        &mut audio,
                                                        &config,
                                                    );
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

                                            set_game_state(
                                                &mut game_state,
                                                GameState::Menu,
                                                &mut audio,
                                                &config,
                                            );
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

                        pickup_items(
                            &mut player,
                            &mut map,
                            &weapon_registry,
                        );

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

                        match player.weapon_state {

                            crate::weapons::
                                WeaponState::Idle => {}

                            crate::weapons::
                                WeaponState::Firing => {

                                //println!("Entering Cooldown");

                                player.weapon_frame = 0;

                                player.weapon_anim_timer =
                                    colt_weapon
                                        .fire_rate
                                        /
                                        colt_weapon
                                            .fire_frames
                                            as f32;

                                player.weapon_timer =
                                    colt_weapon
                                        .fire_rate;

                                player.weapon_state =
                                    crate::weapons::
                                        WeaponState::Cooldown;
                            }

                            crate::weapons::
                                WeaponState::Cooldown => {

                               // println!(
                               //     "Cooldown: {}",
                               //     player.weapon_timer
                               // );

                                player.weapon_timer -=
                                    1.0 / 60.0;

                                player.weapon_anim_timer -=
                                    1.0 / 60.0;

                                if player
                                    .weapon_anim_timer
                                    <= 0.0
                                {

                                    player.weapon_anim_timer =
                                        colt_weapon
                                            .fire_rate
                                            /
                                            colt_weapon
                                                .fire_frames
                                                as f32;

                                    if player.weapon_frame + 1
                                        <
                                        colt_weapon
                                            .fire_frames
                                    {

                                        player.weapon_frame += 1;
                                    }
                                }

                                if player
                                    .weapon_timer
                                    <= 0.0
                                {

                                //    println!("Ready");

                                    player.weapon_state =
                                        crate::weapons::
                                            WeaponState::Idle;

                                    player.weapon_frame =
                                        0;
                                }
                            }
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

                            GameState::Cutscene => {

                                frame.fill(0);

                                if let Some(
                                    cutscene
                                ) =
                                    &mut cutscene_player
                                {

                                    cutscene.update();

                                    if cutscene.finished()
                                    {

                                        set_game_state(
                                            &mut game_state,
                                            GameState::Menu,
                                            &mut audio,
                                            &config,
                                        );
                                    }

                                    else if let Some(
                                        path
                                    ) =
                                        cutscene.current_path()
                                    {

                                        let texture =
                                            Texture::load(
                                                path
                                                    .to_str()
                                                    .unwrap()
                                            );

                                        let copy_len =
                                            frame.len()
                                                .min(
                                                    texture
                                                        .data
                                                        .len()
                                                );

                                        frame[..copy_len]
                                            .copy_from_slice(
                                                &texture.data[
                                                    ..copy_len
                                                ]
                                            );
                                    }
                                }
                            }

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

                            if player
                                .inventory
                                .equipped_weapon
                                .is_some()
                            {

                                let current_viewmodel =

                                    match player.weapon_state {

                                        crate::weapons::
                                            WeaponState::Idle => {

                                            &colt_idle
                                        }

                                        crate::weapons::
                                            WeaponState::Firing
                                        |
                                        crate::weapons::
                                            WeaponState::Cooldown => {

                                            match player.weapon_frame {

                                                0 => &colt_fire_0,
                                                1 => &colt_fire_1,
                                                2 => &colt_fire_2,
                                                _ => &colt_fire_3,
                                            }
                                        }
                                    };

                                render_viewmodel(
                                    frame,
                                    current_viewmodel,
                                    235,
                                    290,
                                    0.7,
                                );
                            }

                            render_hud(
                                frame,
                                &hud_texture,
                                &colt_icon,
                                &player,
                                &menu_font,
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