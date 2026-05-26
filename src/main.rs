// ============================================================================
// MINERVA BASE ENGINE
// FIXED VERSION
// - Fixed menu navigation
// - Fixed EXIT rendering
// - Fixed movement
// - Fixed wall scaling bug
// - Restored floor/ceiling rendering
// - Proper map loading
// - Proper wall texture tiling
// ============================================================================

use fontdue::Font;
use glam::Vec2;
use image::GenericImageView;
use pixels::{Pixels, SurfaceTexture};
use serde::Deserialize;
use std::collections::HashSet;
use std::f32::consts::PI;
use std::fs;
use std::sync::Arc;

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
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

// ============================================================================
// CONSTANTS
// ============================================================================

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const FOV: f32 = PI / 3.0;

const WALK_SPEED: f32 = 3.0;
const RUN_MULTIPLIER: f32 = 2.0;

const TEXTURE_SIZE: usize = 64;

const MOUSE_SENSITIVITY_X: f32 = 0.003;
const MOUSE_SENSITIVITY_Y: f32 = 0.3;

const MAX_PITCH: f32 = 120.0;

// ============================================================================
// CONFIG
// ============================================================================

#[derive(Debug, Deserialize)]
struct GameConfig {
    menu: MenuConfig,
    episode: Vec<EpisodeConfig>,
}

#[derive(Debug, Deserialize)]
struct MenuConfig {
    font: String,
    background: String,
    exit_message: String,
}

#[derive(Debug, Deserialize)]
struct EpisodeConfig {
    title: String,
    maps: Vec<MapConfig>,
}

#[derive(Debug, Deserialize)]
struct MapConfig {
    title: String,
    file: String,
}

// ============================================================================
// GAME STATES
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    MainMenu,
    EpisodeMenu,
    Playing,
    Paused,
}

// ============================================================================
// PLAYER
// ============================================================================

struct Player {
    position: Vec2,
    angle: f32,
    pitch: f32,
}

// ============================================================================
// WALL
// ============================================================================

#[derive(Clone)]
struct Wall {
    start: Vec2,
    end: Vec2,
    texture: String,
}

// ============================================================================
// MAP
// ============================================================================

struct Map {
    walls: Vec<Wall>,
    floor_texture: String,
    ceiling_texture: String,
    spawn: Vec2,
    spawn_angle: f32,
}

// ============================================================================
// TEXTURE
// ============================================================================

struct Texture {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Texture {

    fn load(path: &str) -> Self {

        let img = image::open(path)
            .expect("failed to load texture")
            .to_rgba8();

        let (w, h) = img.dimensions();

        Self {
            width: w as usize,
            height: h as usize,
            data: img.into_raw(),
        }
    }

    fn sample(
        &self,
        x: usize,
        y: usize,
    ) -> [u8; 4] {

        let tx = x % self.width;
        let ty = y % self.height;

        let idx =
            (ty * self.width + tx) * 4;

        [
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        ]
    }
}

// ============================================================================
// KEY BINDINGS
// ============================================================================

struct KeyBindings {

    forward: KeyCode,
    backward: KeyCode,

    left: KeyCode,
    right: KeyCode,

    run: KeyCode,

    menu: KeyCode,

    up: KeyCode,
    down: KeyCode,

    select: KeyCode,
}

impl KeyBindings {

    fn load(path: &str) -> Self {

        let content =
            fs::read_to_string(path)
                .expect("failed to read keyboard.txt");

        let mut bindings = Self {

            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,

            left: KeyCode::KeyA,
            right: KeyCode::KeyD,

            run: KeyCode::ShiftLeft,

            menu: KeyCode::Escape,

            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,

            select: KeyCode::Enter,
        };

        for line in content.lines() {

            let line = line.trim();

                if line.is_empty()
                || line.starts_with("#")
                {
                    continue;
                }

            let parts: Vec<&str> =
                line.split('=').collect();

            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();

            let value =
                parts[1].trim();

            let parsed =
                parse_keycode(value);

            match key {

                "forward" =>
                    bindings.forward = parsed,

                "backward" =>
                    bindings.backward = parsed,

                "left" =>
                    bindings.left = parsed,

                "right" =>
                    bindings.right = parsed,

                "run" =>
                    bindings.run = parsed,

                "menu" =>
                    bindings.menu = parsed,

                "up" =>
                    bindings.up = parsed,

                "down" =>
                    bindings.down = parsed,

                "select" =>
                    bindings.select = parsed,

                _ => {}
            }
        }

        bindings
    }
}

// ============================================================================
// KEY PARSER
// ============================================================================

fn parse_keycode(name: &str) -> KeyCode {

    match name.to_lowercase().as_str() {

        "w" => KeyCode::KeyW,
        "a" => KeyCode::KeyA,
        "s" => KeyCode::KeyS,
        "d" => KeyCode::KeyD,

        "shift" =>
            KeyCode::ShiftLeft,

        "esc" =>
            KeyCode::Escape,

        "up" =>
            KeyCode::ArrowUp,

        "down" =>
            KeyCode::ArrowDown,

        "enter" =>
            KeyCode::Enter,

        _ =>
            KeyCode::Space,
    }
}

// ============================================================================
// MAP LOADER
// ============================================================================

fn load_map(path: &str) -> Map {

    let content =
        fs::read_to_string(path)
            .expect("failed to load map");

    let mut walls = Vec::new();

    let mut floor_texture =
        "textureD".to_string();

    let mut ceiling_texture =
        "textureU".to_string();

    let mut spawn =
        Vec2::new(256.0, 256.0);

    let mut spawn_angle = 0.0;

    for line in content.lines() {

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> =
            line.split_whitespace()
                .collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {

            "floor" => {
                floor_texture =
                    parts[1].to_string();
            }

            "ceiling" => {
                ceiling_texture =
                    parts[1].to_string();
            }

            "wall" => {

                let x1: f32 =
                    parts[1].parse().unwrap();

                let y1: f32 =
                    parts[2].parse().unwrap();

                let x2: f32 =
                    parts[3].parse().unwrap();

                let y2: f32 =
                    parts[4].parse().unwrap();

                walls.push(
                    Wall {
                        start:
                            Vec2::new(x1, y1),

                        end:
                            Vec2::new(x2, y2),

                        texture:
                            parts[5].to_string(),
                    }
                );
            }

            "spawn" => {

                spawn.x =
                    parts[1].parse().unwrap();

                spawn.y =
                    parts[2].parse().unwrap();

                spawn_angle =
                    parts[3].parse().unwrap();
            }

            _ => {}
        }
    }

    Map {
        walls,
        floor_texture,
        ceiling_texture,
        spawn,
        spawn_angle,
    }
}

// ============================================================================
// FONT DRAWING
// ============================================================================

fn draw_text(
    frame: &mut [u8],
    font: &Font,
    text: &str,
    start_x: i32,
    start_y: i32,
    scale: f32,
    color: [u8; 3],
) {

    let mut pen_x = start_x;

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
                frame[idx + 3] = alpha;
            }
        }

        pen_x +=
            metrics.advance_width as i32;
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {

    let config: GameConfig =
        toml::from_str(
            &fs::read_to_string(
                "config/game.toml"
            )
            .unwrap(),
        )
        .unwrap();

    let current_map =
        load_map(
            &config
                .episode[0]
                .maps[0]
                .file
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

    let texture_n =
        Texture::load(
            "assets/textures/textureN.png"
        );

    let texture_s =
        Texture::load(
            "assets/textures/textureS.png"
        );

    let texture_e =
        Texture::load(
            "assets/textures/textureE.png"
        );

    let texture_w =
        Texture::load(
            "assets/textures/textureW.png"
        );

    let texture_u =
        Texture::load(
            "assets/textures/textureU.png"
        );

    let texture_d =
        Texture::load(
            "assets/textures/textureD.png"
        );

    let bindings =
        KeyBindings::load(
            "config/keyboard.txt"
        );

    let event_loop =
        EventLoop::new().unwrap();

    let window = Arc::new(

        WindowBuilder::new()

            .with_title(
                "Minerva Base"
            )

            .with_inner_size(
                LogicalSize::new(
                    WIDTH,
                    HEIGHT,
                ),
            )

            .build(&event_loop)
            .unwrap(),
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

    let mut pressed_keys =
        HashSet::new();

    let mut game_state =
        GameState::MainMenu;

    let mut menu_index = 0usize;

    let mut right_mouse_held =
        false;

    let mut player =
        Player {

            position:
                current_map.spawn,

            angle:
                current_map.spawn_angle,

            pitch: 0.0,
        };

    event_loop.run(
        move |event, target| {

            match event {

                Event::DeviceEvent {

                    event:
                        DeviceEvent::MouseMotion {
                            delta,
                        },

                    ..
                } => {

                    if right_mouse_held {

                        if game_state
                            == GameState::Playing
                        {

                            player.angle +=
                                delta.0 as f32
                                    * MOUSE_SENSITIVITY_X;

                            player.pitch +=
                                delta.1 as f32
                                    * MOUSE_SENSITIVITY_Y;

                            player.pitch =
                                player.pitch.clamp(
                                    -MAX_PITCH,
                                    MAX_PITCH,
                                );
                        }
                    }
                }

                Event::WindowEvent {
                    event,
                    ..
                } => {

                    match event {

                        WindowEvent::CloseRequested => {
                            target.exit();
                        }

                        WindowEvent::MouseInput {
                            state,
                            button,
                            ..
                        } => {

                            if button
                                == MouseButton::Right
                            {

                                right_mouse_held =
                                    state
                                    ==
                                    ElementState::Pressed;
                            }
                        }

                        WindowEvent::KeyboardInput {
                            event,
                            ..
                        } => {

                            if let PhysicalKey::Code(code)
                                =
                                event.physical_key
                            {

                                match event.state {

                                    ElementState::Pressed => {
                                        pressed_keys.insert(code);
                                    }

                                    ElementState::Released => {
                                        pressed_keys.remove(&code);
                                    }
                                }

                                if event.state
                                    ==
                                    ElementState::Pressed
                                {

                                    if game_state
                                        ==
                                        GameState::MainMenu
                                    {

                                        if code
                                            ==
                                            bindings.up
                                        {

                                            if menu_index > 0 {
                                                menu_index -= 1;
                                            }
                                        }

                                        if code
                                            ==
                                            bindings.down
                                        {

                                            if menu_index < 1 {
                                                menu_index += 1;
                                            }
                                        }

                                        if code
                                            ==
                                            bindings.select
                                        {

                                            if menu_index == 0 {

                                                game_state =
                                                    GameState::Playing;
                                            }

                                            else {

                                                target.exit();
                                            }
                                        }
                                    }

                                    else if code
    ==
    bindings.menu
{

    game_state =
        match game_state {

            GameState::Playing =>
                GameState::MainMenu,

            GameState::MainMenu =>
                GameState::Playing,

            GameState::Paused =>
                GameState::Playing,

            _ =>
                game_state,
        };
}
                                }
                            }
                        }

                        WindowEvent::RedrawRequested => {

                            let frame =
                                pixels.frame_mut();

                            match game_state {

                                GameState::MainMenu => {

                                    for y in 0..HEIGHT as usize {

                                        for x in 0..WIDTH as usize {

                                            let idx =
                                                (y * WIDTH as usize + x)
                                                    * 4;

                                            let color =
                                                menu_background.sample(x, y);

                                            frame[idx] = color[0];
                                            frame[idx + 1] = color[1];
                                            frame[idx + 2] = color[2];
                                            frame[idx + 3] = 255;
                                        }
                                    }

                                    draw_text(
                                        frame,
                                        &menu_font,
                                        "SELECT EPISODE",
                                        140,
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
                                        &menu_font,
                                        &config
                                            .episode[0]
                                            .title,
                                        180,
                                        220,
                                        30.0,
                                        title_color,
                                    );

                                    draw_text(
                                        frame,
                                        &menu_font,
                                        &config.menu.exit_message,
                                        180,
                                        280,
                                        30.0,
                                        exit_color,
                                    );
                                }

                                GameState::Playing |
                                GameState::Paused => {

                                    update_player(
                                        &mut player,
                                        &bindings,
                                        &pressed_keys,
                                        &current_map,
                                    );

                                    render_world(
                                        frame,
                                        &player,
                                        &current_map,
                                        &texture_n,
                                        &texture_s,
                                        &texture_e,
                                        &texture_w,
                                        &texture_u,
                                        &texture_d,
                                    );
                                }

                                _ => {}
                            }

                            pixels.render().unwrap();
                        }

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    window.request_redraw();
                }

                _ => {}
            }
        }
    ).unwrap();
}

// ============================================================================
// PLAYER UPDATE
// ============================================================================

fn update_player(
    player: &mut Player,
    bindings: &KeyBindings,
    pressed_keys: &HashSet<KeyCode>,
    map: &Map,
) {

    let mut speed = WALK_SPEED;

    if pressed_keys.contains(
        &bindings.run
    ) {

        speed *= RUN_MULTIPLIER;
    }

    let forward =
        Vec2::new(
            player.angle.cos(),
            player.angle.sin(),
        );

    let right =
        Vec2::new(
            -player.angle.sin(),
            player.angle.cos(),
        );

    let mut new_position =
        player.position;

    if pressed_keys.contains(
        &bindings.forward
    ) {

        new_position +=
            forward * speed;
    }

    if pressed_keys.contains(
        &bindings.backward
    ) {

        new_position -=
            forward * speed;
    }

    if pressed_keys.contains(
        &bindings.left
    ) {

        new_position -=
            right * speed;
    }

    if pressed_keys.contains(
        &bindings.right
    ) {

        new_position +=
            right * speed;
    }

    if collision_check(
        new_position,
        map,
    ) {

        player.position =
            new_position;
    }
}

// ============================================================================
// COLLISION
// ============================================================================

fn collision_check(
    position: Vec2,
    map: &Map,
) -> bool {

    for wall in &map.walls {

        let wall_dir =
            wall.end - wall.start;

        let wall_length =
            wall_dir.length();

        let wall_normal =
            wall_dir.normalize();

        let to_player =
            position - wall.start;

        let projection =
            to_player.dot(wall_normal);

        if projection >= 0.0
            &&
            projection <= wall_length
        {

            let closest =
                wall.start
                    + wall_normal
                        * projection;

            let distance =
                (position - closest)
                    .length();

            if distance < 10.0 {
                return false;
            }
        }
    }

    true
}

// ============================================================================
// RENDER WORLD
// ============================================================================

fn render_world(
    frame: &mut [u8],
    player: &Player,
    map: &Map,
    texture_n: &Texture,
    texture_s: &Texture,
    texture_e: &Texture,
    texture_w: &Texture,
    texture_u: &Texture,
    texture_d: &Texture,
) {

    for y in 0..HEIGHT as usize {

        let ray_angle_step =
            FOV / WIDTH as f32;

        let p =
            y as f32 - HEIGHT as f32 / 2.0
                - player.pitch;

        // FLOOR + CEILING CASTING

        if p.abs() > 0.1 {

            let row_distance =
                (HEIGHT as f32 / 2.0) / p.abs();

            for x in 0..WIDTH as usize {

                let ray_angle =
                    player.angle
                        - FOV / 2.0
                        + x as f32 * ray_angle_step;

                let ray_dir =
                    Vec2::new(
                        ray_angle.cos(),
                        ray_angle.sin(),
                    );

                let world_x =
                    player.position.x
                        + ray_dir.x
                            * row_distance
                            * 64.0;

                let world_y =
                    player.position.y
                        + ray_dir.y
                            * row_distance
                            * 64.0;

                let tex_x =
                    world_x as usize % TEXTURE_SIZE;

                let tex_y =
                    world_y as usize % TEXTURE_SIZE;

                let idx =
                    (y * WIDTH as usize + x)
                        * 4;

                // FLOOR

                if y > HEIGHT as usize / 2 {

                    let color =
                        texture_d.sample(
                            tex_x,
                            tex_y,
                        );

                    frame[idx] = color[0];
                    frame[idx + 1] = color[1];
                    frame[idx + 2] = color[2];
                    frame[idx + 3] = 255;
                }

                // CEILING

                else {

                    let color =
                        texture_u.sample(
                            tex_x,
                            tex_y,
                        );

                    frame[idx] = color[0];
                    frame[idx + 1] = color[1];
                    frame[idx + 2] = color[2];
                    frame[idx + 3] = 255;
                }
            }
        }
    }

    // WALL RENDERING

    for x in 0..WIDTH as usize {

        let ray_angle =
            player.angle
                - FOV / 2.0
                + (x as f32 / WIDTH as f32)
                    * FOV;

        let ray_dir =
            Vec2::new(
                ray_angle.cos(),
                ray_angle.sin(),
            );

        let mut closest_distance =
            999999.0;

        let mut hit_texture =
            texture_n;

        let mut hit_u = 0.0;

        for wall in &map.walls {

            if let Some((distance, hit_point)) =
                raycast_wall(
                    player.position,
                    ray_dir,
                    wall,
                )
            {

                if distance
                    < closest_distance
                {

                    closest_distance =
                        distance;

                    let dx =
                       (wall.end.x - wall.start.x).abs();

                    let dy =
                       (wall.end.y - wall.start.y).abs();

                    if dx > dy {

                        hit_u = hit_point.x;

                    } else {

                        hit_u = hit_point.y;
                    }

                    hit_texture =
                        match wall.texture.as_str() {

                            "textureN" =>
                                texture_n,

                            "textureS" =>
                                texture_s,

                            "textureE" =>
                                texture_e,

                            "textureW" =>
                                texture_w,

                            _ =>
                                texture_n,
                        };
                }
            }
        }

        let corrected_distance =
            closest_distance
                * (player.angle - ray_angle)
                    .cos();

        let wall_height =
            (HEIGHT as f32 * 64.0)
                / corrected_distance.max(1.0);

        let wall_top =
            ((HEIGHT as f32 / 2.0)
                - wall_height / 2.0
                + player.pitch)
                as i32;

        let wall_bottom =
            ((HEIGHT as f32 / 2.0)
                + wall_height / 2.0
                + player.pitch)
                as i32;

        // PROPER WALL TILING

        let texture_x =
            hit_u.abs() as usize
            % TEXTURE_SIZE;

        for y in wall_top..wall_bottom {

            if y < 0 || y >= HEIGHT as i32 {
                continue;
            }

            let wall_y =
                y as f32 - wall_top as f32;

            let texture_y =
                ((wall_y / wall_height)
               * TEXTURE_SIZE as f32)
               .clamp(0.0, 63.0)
                as usize;

            let color =
                hit_texture.sample(
                    texture_x,
                    texture_y,
                );

            let idx =
                ((y as usize * WIDTH as usize)
                    + x)
                    * 4;

            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }
    }
}

// ============================================================================
// RAYCAST
// ============================================================================

fn raycast_wall(
    origin: Vec2,
    dir: Vec2,
    wall: &Wall,
) -> Option<(f32, Vec2)> {

    let x1 = wall.start.x;
    let y1 = wall.start.y;

    let x2 = wall.end.x;
    let y2 = wall.end.y;

    let x3 = origin.x;
    let y3 = origin.y;

    let x4 = origin.x + dir.x;
    let y4 = origin.y + dir.y;

    let denominator =
        (x1 - x2) * (y3 - y4)
            -
        (y1 - y2) * (x3 - x4);

    if denominator.abs() < 0.0001 {
        return None;
    }

    let t =
        ((x1 - x3) * (y3 - y4)
            -
        (y1 - y3) * (x3 - x4))
            / denominator;

    let u =
        -(
            (x1 - x2) * (y1 - y3)
                -
            (y1 - y2) * (x1 - x3)
        )
            / denominator;

    if t >= 0.0
        &&
        t <= 1.0
        &&
        u > 0.0
    {

        let hit_point =
            origin + dir * u;

        let distance =
            (hit_point - origin)
                .length();

        Some((
            distance,
            hit_point,
        ))
    }

    else {

        None
    }
}