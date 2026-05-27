// ============================================================================
// MINERVA BASE ENGINE
// SECTOR RENDERER
//
// FEATURES
// - Menu restored
// - ESC pause menu
// - Font rendering restored
// - Portal rendering exclusion
// - Sector-aware floors/ceilings
// - Sector polygons
// - Perspective correct floor motion
// - Proper wall tiling
// - Proper portal openings
// ============================================================================

use fontdue::Font;
use glam::Vec2;
use pixels::{Pixels, SurfaceTexture};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
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

const TEXTURE_SIZE: usize = 64;

const WALK_SPEED: f32 = 3.0;
const RUN_MULTIPLIER: f32 = 2.0;

const MOUSE_SENSITIVITY_X: f32 = 0.003;
const MOUSE_SENSITIVITY_Y: f32 = 0.5;

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
// GAME STATE
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    MainMenu,
    Playing,
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
// WALL TYPES
// ============================================================================

#[derive(Clone)]
enum WallType {
    Solid,
    Portal(String),
}

// ============================================================================
// WALL
// ============================================================================

#[derive(Clone)]
struct Wall {
    start: Vec2,
    end: Vec2,
    texture: String,
    wall_type: WallType,
}

// ============================================================================
// SECTOR
// ============================================================================

struct Sector {
    name: String,
    floor_texture: String,
    ceiling_texture: String,
    walls: Vec<Wall>,
}

// ============================================================================
// MAP
// ============================================================================

struct Map {
    sectors: Vec<Sector>,
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

        let img =
            image::open(path)
                .unwrap()
                .to_rgba8();

        let (w, h) =
            img.dimensions();

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

        let tx =
            x % self.width;

        let ty =
            y % self.height;

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

fn load_texture_registry(
    path: &str,
) -> HashMap<String, Texture> {

    let mut textures =
        HashMap::new();

    let content =
        fs::read_to_string(path)
            .unwrap();

    let mut texture_count =
        0usize;

    for line in content.lines() {

        let line =
            line.trim();

        if line.is_empty()
            || line.starts_with("#")
        {
            continue;
        }

        let parts: Vec<&str> =
            line.split('=')
                .collect();

        if parts.len() != 2 {
            continue;
        }

        let name =
            parts[0]
                .trim()
                .to_string();

        let path =
            parts[1]
                .trim();

        textures.insert(
            name,
            Texture::load(path),
        );

        texture_count += 1;

        if texture_count >= 256 {

            panic!(
                "Texture limit exceeded (256)"
            );
        }
    }

    textures
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

// ============================================================================
// MAP LOADER
// ============================================================================

fn load_map(path: &str) -> Map {

    let content =
        fs::read_to_string(path)
            .unwrap();

    let mut sectors =
        Vec::new();

    let mut current_sector:
        Option<Sector> = None;

    let mut spawn =
        Vec2::ZERO;

    let mut spawn_angle =
        0.0;

    for line in content.lines() {

        let line =
            line.trim();

        if line.is_empty()
            || line.starts_with("#")
        {
            continue;
        }

        let parts: Vec<&str> =
            line.split_whitespace()
                .collect();

        match parts[0] {

            "sector" => {

                if let Some(sec)
                    = current_sector.take()
                {
                    sectors.push(sec);
                }

                current_sector =
                    Some(
                        Sector {

                            name:
                                parts[1].to_string(),

                            floor_texture:
                                "textureD".to_string(),

                            ceiling_texture:
                                "textureU".to_string(),

                            walls:
                                Vec::new(),
                        }
                    );
            }

            "floor" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {

                    sec.floor_texture =
                        parts[1].to_string();
                }
            }

            "ceiling" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {

                    sec.ceiling_texture =
                        parts[1].to_string();
                }
            }

            "wall" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {

                    sec.walls.push(

                        Wall {

                            start:
                                Vec2::new(
                                    parts[1].parse().unwrap(),
                                    parts[2].parse().unwrap(),
                                ),

                            end:
                                Vec2::new(
                                    parts[3].parse().unwrap(),
                                    parts[4].parse().unwrap(),
                                ),

                            texture:
                                parts[5].to_string(),

                            wall_type:
                                WallType::Solid,
                        }
                    );
                }
            }

            "portal" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {

                    sec.walls.push(

                        Wall {

                            start:
                                Vec2::new(
                                    parts[1].parse().unwrap(),
                                    parts[2].parse().unwrap(),
                                ),

                            end:
                                Vec2::new(
                                    parts[3].parse().unwrap(),
                                    parts[4].parse().unwrap(),
                                ),

                            texture:
                                String::new(),

                            wall_type:
                                WallType::Portal(
                                    parts[5].to_string()
                                ),
                        }
                    );
                }
            }

            "spawn" => {

                spawn =
                    Vec2::new(
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                    );

                spawn_angle =
                    parts[3].parse().unwrap();
            }

            _ => {}
        }
    }

    if let Some(sec)
        = current_sector.take()
    {
        sectors.push(sec);
    }

    Map {
        sectors,
        spawn,
        spawn_angle,
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

    let map =
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

  let textures =
    load_texture_registry(
        "config/textures.txt"
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

    let mut game_state =
        GameState::MainMenu;

    let mut menu_index =
        0usize;

    let mut right_mouse =
        false;

    let mut pressed_keys =
        HashSet::new();

    let mut player =
        Player {

            position:
                map.spawn,

            angle:
                map.spawn_angle,

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

                    if right_mouse
                        &&
                        game_state
                            ==
                            GameState::Playing
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
                                ==
                                MouseButton::Right
                            {

                                right_mouse =
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

                                    // ESC

                                    if code
                                        ==
                                        KeyCode::Escape
                                    {

                                        game_state =
                                            match game_state {

                                                GameState::MainMenu =>
                                                    GameState::Playing,

                                                GameState::Playing =>
                                                    GameState::MainMenu,
                                            };
                                    }

                                    // MENU

                                    if game_state
                                        ==
                                        GameState::MainMenu
                                    {

                                        if code
                                            ==
                                            KeyCode::ArrowUp
                                        {

                                            if menu_index > 0 {
                                                menu_index -= 1;
                                            }
                                        }

                                        if code
                                            ==
                                            KeyCode::ArrowDown
                                        {

                                            if menu_index < 1 {
                                                menu_index += 1;
                                            }
                                        }

                                        if code
                                            ==
                                            KeyCode::Enter
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
                                }
                            }
                        }

                        WindowEvent::RedrawRequested => {

                            let frame =
                                pixels.frame_mut();

                            if game_state
                                ==
                                GameState::MainMenu
                            {

                                render_menu(
                                    frame,
                                    &menu_background,
                                    &menu_font,
                                    &config,
                                    menu_index,
                                );
                            }

                            else {

                                update_player(
                                    &mut player,
                                    &pressed_keys,
                                    &map,
                                );

                                render_world(
                                frame,
                                &player,
                                &map,
                                &textures,
                                );
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
// MENU RENDERING
// ============================================================================

fn render_menu(
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

// ============================================================================
// PLAYER UPDATE
// ============================================================================

fn update_player(
    player: &mut Player,
    keys: &HashSet<KeyCode>,
    map: &Map,
) {

    let mut speed =
        WALK_SPEED;

    if keys.contains(
        &KeyCode::ShiftLeft
    ) {

        speed *=
            RUN_MULTIPLIER;
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

    if keys.contains(
        &KeyCode::KeyW
    ) {

        new_position +=
            forward * speed;
    }

    if keys.contains(
        &KeyCode::KeyS
    ) {

        new_position -=
            forward * speed;
    }

    if keys.contains(
        &KeyCode::KeyA
    ) {

        new_position -=
            right * speed;
    }

    if keys.contains(
        &KeyCode::KeyD
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

    for sector in &map.sectors {

        for wall in &sector.walls {

            if let WallType::Solid =
                wall.wall_type
            {

                let wall_dir =
                    wall.end - wall.start;

                let wall_length =
                    wall_dir.length();

                let wall_normal =
                    wall_dir.normalize();

                let to_player =
                    position - wall.start;

                let projection =
                    to_player.dot(
                        wall_normal
                    );

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
        }
    }

    true
}

fn point_in_sector(
    point: Vec2,
    sector: &Sector,
) -> bool {

    let mut inside =
        false;

    let walls =
        &sector.walls;

    let count =
        walls.len();

    for i in 0..count {

        let a =
            walls[i].start;

        let b =
            walls[i].end;

        let intersect =
            ((a.y > point.y)
                !=
             (b.y > point.y))
            &&
            (
                point.x
                    <
                (b.x - a.x)
                    * (point.y - a.y)
                    / ((b.y - a.y) + 0.0001)
                    + a.x
            );

        if intersect {
            inside = !inside;
        }
    }

    inside
}

// ============================================================================
// WORLD RENDER
// ============================================================================

fn render_world(
    frame: &mut [u8],
    player: &Player,
    map: &Map,
    textures: &HashMap<String, Texture>,
) {

    // ========================================================================
    // CLEAR FRAME
    // ========================================================================

    for pixel in frame.chunks_exact_mut(4) {

        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
        pixel[3] = 255;
    }

    // ========================================================================
    // COLUMN RENDERER
    // ========================================================================

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
            f32::MAX;

        let mut hit_wall: Option<&Wall> =
            None;

        let mut hit_sector: Option<&Sector> =
            None;

        let mut hit_point =
            Vec2::ZERO;

        // ====================================================================
        // FIND CLOSEST WALL
        // ====================================================================

        for sector in &map.sectors {

            for wall in &sector.walls {

                if let Some((distance, point))
                    =
                    raycast_wall(
                        player.position,
                        ray_dir,
                        wall,
                    )
                {

                    match &wall.wall_type {

    WallType::Solid => {

        if distance < closest_distance {

            closest_distance =
                distance;

            hit_wall =
                Some(wall);

            hit_sector =
                Some(sector);

            hit_point =
                point;
        }
    }

 WallType::Portal(_) => {

    // IMPORTANT:
    // Portal walls are invisible and non-blocking.
    // We do NOT terminate the ray here.
    // Instead the ray continues naturally
    // until it hits a real solid wall.

    continue;
}

}
                }
            }
        }

        // ====================================================================
        // NO HIT
        // ====================================================================

        if hit_wall.is_none() {
            continue;
        }

        let wall =
            hit_wall.unwrap();

        let sector =
            hit_sector.unwrap();

        // ====================================================================
        // TEXTURES
        // ====================================================================

    let wall_texture =
    textures
        .get(&wall.texture)
        .or_else(|| textures.get("textureN"))
        .unwrap();
        
        // ====================================================================
        // WALL PROJECTION
        // ====================================================================

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

        // ====================================================================
        // WALL TILING
        // ====================================================================

        let dx =
            (wall.end.x - wall.start.x).abs();

        let dy =
            (wall.end.y - wall.start.y).abs();

        let hit_u =
            if dx > dy {
                hit_point.x
            } else {
                hit_point.y
            };

        let texture_x =
            hit_u.abs() as usize
                % TEXTURE_SIZE;

        // ====================================================================
        // CEILING
        // ====================================================================

        for y in 0..wall_top {

            if y < 0
                || y >= HEIGHT as i32
            {
                continue;
            }

            let p =
                y as f32
                    - HEIGHT as f32 / 2.0
                    - player.pitch;

            if p.abs() < 0.1 {
                continue;
            }

            let row_distance =
                (HEIGHT as f32 / 2.0)
                    / p.abs();

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
                world_x.abs() as usize
                    % TEXTURE_SIZE;

            let tex_y =
                world_y.abs() as usize
                    % TEXTURE_SIZE;

            let mut active_sector =
    sector;

for test_sector in &map.sectors {

    if point_in_sector(
        Vec2::new(
            world_x,
            world_y,
        ),
        test_sector,
    ) {

        active_sector =
            test_sector;

        break;
    }
}

let ceiling_texture =
    textures
        .get(&active_sector.ceiling_texture)
        .or_else(|| textures.get("textureU"))
        .unwrap();

            let color =
                ceiling_texture.sample(
                    tex_x,
                    tex_y,
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

        // ====================================================================
        // WALL
        // ====================================================================

        match wall.wall_type {

            WallType::Solid => {

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
                        ((y as usize * WIDTH as usize)
                            + x)
                            * 4;

                    frame[idx] = color[0];
                    frame[idx + 1] = color[1];
                    frame[idx + 2] = color[2];
                    frame[idx + 3] = 255;
                }
            }

            // ================================================================
            // PORTALS DO NOT RENDER WALLS
            // ================================================================

            WallType::Portal(_) => {}
        }

        // ====================================================================
        // FLOOR
        // ====================================================================

        for y in wall_bottom..HEIGHT as i32 {

            if y < 0
                || y >= HEIGHT as i32
            {
                continue;
            }

            let p =
                y as f32
                    - HEIGHT as f32 / 2.0
                    - player.pitch;

            if p.abs() < 0.1 {
                continue;
            }

            let row_distance =
                (HEIGHT as f32 / 2.0)
                    / p.abs();

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
                world_x.abs() as usize
                    % TEXTURE_SIZE;

            let tex_y =
                world_y.abs() as usize
                    % TEXTURE_SIZE;
            
            let mut active_sector =
    sector;

for test_sector in &map.sectors {

    if point_in_sector(
        Vec2::new(
            world_x,
            world_y,
        ),
        test_sector,
    ) {

        active_sector =
            test_sector;

        break;
    }
}

let floor_texture =
    textures
        .get(&active_sector.floor_texture)
        .or_else(|| textures.get("textureD"))
        .unwrap();

            let color =
                floor_texture.sample(
                    tex_x,
                    tex_y,
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