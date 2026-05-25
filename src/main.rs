use glam::Vec2;
use pixels::{Pixels, SurfaceTexture};
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

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const ROOM_SIZE: f32 = 512.0;
const TEXTURE_SIZE: usize = 64;

const FOV: f32 = PI / 3.0;

const WALK_SPEED: f32 = 3.0;
const RUN_MULTIPLIER: f32 = 2.0;

const MOUSE_SENSITIVITY: f32 = 0.003;

const PLAYER_RADIUS: f32 = 12.0;

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

    fn sample(&self, x: usize, y: usize) -> [u8; 4] {
        let tx = x % self.width;
        let ty = y % self.height;

        let idx = (ty * self.width + tx) * 4;

        [
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        ]
    }
}

struct Player {
    position: Vec2,
    angle: f32,
    pitch: f32,
}

struct KeyBindings {
    forward: KeyCode,
    backward: KeyCode,
    left: KeyCode,
    right: KeyCode,
    run: KeyCode,
}

impl KeyBindings {
    fn load(path: &str) -> Self {
        let content =
            fs::read_to_string(path)
                .expect("failed to read keyboard config");

        let mut forward = KeyCode::KeyW;
        let mut backward = KeyCode::KeyS;
        let mut left = KeyCode::KeyA;
        let mut right = KeyCode::KeyD;
        let mut run = KeyCode::ShiftLeft;

        for line in content.lines() {

            let parts: Vec<&str> =
                line.split('=').collect();

            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            let parsed = match value {
                "W" => KeyCode::KeyW,
                "S" => KeyCode::KeyS,
                "A" => KeyCode::KeyA,
                "D" => KeyCode::KeyD,
                "SHIFT" => KeyCode::ShiftLeft,
                _ => continue,
            };

            match key {
                "forward" => forward = parsed,
                "backward" => backward = parsed,
                "left" => left = parsed,
                "right" => right = parsed,
                "run" => run = parsed,
                _ => {}
            }
        }

        Self {
            forward,
            backward,
            left,
            right,
            run,
        }
    }
}

fn main() {

    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Minerva Base")
            .with_inner_size(
                LogicalSize::new(WIDTH, HEIGHT),
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
        .expect("failed to create pixel surface");

    let texture_n =
        Texture::load("assets/textures/textureN.png");

    let texture_s =
        Texture::load("assets/textures/textureS.png");

    let texture_e =
        Texture::load("assets/textures/textureE.png");

    let texture_w =
        Texture::load("assets/textures/textureW.png");

    let texture_u =
        Texture::load("assets/textures/textureU.png");

    let texture_d =
        Texture::load("assets/textures/textureD.png");

    let bindings =
        KeyBindings::load("config/keyboard.txt");

    let mut pressed_keys = HashSet::new();

    let mut right_mouse_held = false;

    let mut player = Player {
        position: Vec2::new(
            ROOM_SIZE / 2.0,
            ROOM_SIZE / 2.0,
        ),
        angle: 0.0,
        pitch: 0.0,
    };

    event_loop
        .run(move |event, target| {

            match event {

                Event::DeviceEvent {
                    event:
                        DeviceEvent::MouseMotion {
                            delta,
                        },
                    ..
                } => {

                    if right_mouse_held {

                        player.angle +=
                            delta.0 as f32
                                * MOUSE_SENSITIVITY;

                        player.pitch +=
                            delta.1 as f32
                                * 0.2;

                        player.pitch =
                            player.pitch.clamp(
                                -120.0,
                                120.0,
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

                            if button == MouseButton::Right {

                                right_mouse_held =
                                    state
                                        == ElementState::Pressed;
                            }
                        }

                        WindowEvent::KeyboardInput {
                            event,
                            ..
                        } => {

                            if let PhysicalKey::Code(code) =
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
                            }
                        }

                        WindowEvent::RedrawRequested => {

                            update_player(
                                &mut player,
                                &bindings,
                                &pressed_keys,
                            );

                            let frame =
                                pixels.frame_mut();

                            render(
                                frame,
                                &player,
                                &texture_n,
                                &texture_s,
                                &texture_e,
                                &texture_w,
                                &texture_u,
                                &texture_d,
                            );

                            pixels.render().unwrap();
                        }

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    window
                        .as_ref()
                        .request_redraw();
                }

                _ => {}
            }
        })
        .unwrap();
}

fn update_player(
    player: &mut Player,
    bindings: &KeyBindings,
    pressed_keys: &HashSet<KeyCode>,
) {

    let mut speed = WALK_SPEED;

    if pressed_keys.contains(&bindings.run) {
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

    if pressed_keys.contains(&bindings.forward) {
        new_position +=
            forward * speed;
    }

    if pressed_keys.contains(&bindings.backward) {
        new_position -=
            forward * speed;
    }

    if pressed_keys.contains(&bindings.left) {
        new_position -=
            right * speed;
    }

    if pressed_keys.contains(&bindings.right) {
        new_position +=
            right * speed;
    }

    if new_position.x > PLAYER_RADIUS
        && new_position.x < ROOM_SIZE - PLAYER_RADIUS
        && new_position.y > PLAYER_RADIUS
        && new_position.y < ROOM_SIZE - PLAYER_RADIUS
    {
        player.position =
            new_position;
    }
}

fn render(
    frame: &mut [u8],
    player: &Player,
    texture_n: &Texture,
    texture_s: &Texture,
    texture_e: &Texture,
    texture_w: &Texture,
    texture_u: &Texture,
    texture_d: &Texture,
) {

    let pitch_offset =
        player.pitch as i32;

    for x in 0..WIDTH as usize {

        let ray_angle =
            player.angle
                - FOV / 2.0
                + (x as f32
                    / WIDTH as f32)
                    * FOV;

        let ray_dir =
            Vec2::new(
                ray_angle.cos(),
                ray_angle.sin(),
            );

        let mut distance = 0.0;

        let step = 1.0;

        let mut hit_x = 0.0;
        let mut hit_y = 0.0;

        let mut wall_texture =
            texture_n;

        let mut horizontal_hit =
            false;

        while distance < 2000.0 {

            let pos =
                player.position
                    + ray_dir * distance;

            if pos.x <= 0.0 {

                wall_texture =
                    texture_w;

                hit_x = pos.x;
                hit_y = pos.y;

                horizontal_hit = false;

                break;
            }

            if pos.x >= ROOM_SIZE {

                wall_texture =
                    texture_e;

                hit_x = pos.x;
                hit_y = pos.y;

                horizontal_hit = false;

                break;
            }

            if pos.y <= 0.0 {

                wall_texture =
                    texture_n;

                hit_x = pos.x;
                hit_y = pos.y;

                horizontal_hit = true;

                break;
            }

            if pos.y >= ROOM_SIZE {

                wall_texture =
                    texture_s;

                hit_x = pos.x;
                hit_y = pos.y;

                horizontal_hit = true;

                break;
            }

            distance += step;
        }

        let corrected_distance =
            distance
                * (player.angle
                    - ray_angle)
                    .cos();

        let wall_height =
            (HEIGHT as f32 * 64.0)
                / corrected_distance.max(1.0);

        let wall_top =
            ((HEIGHT as f32 / 2.0)
                - wall_height / 2.0)
                .max(0.0)
                as i32
                + pitch_offset;

        let wall_bottom =
            ((HEIGHT as f32 / 2.0)
                + wall_height / 2.0)
                .min(
                    HEIGHT as f32 - 1.0,
                )
                as i32
                + pitch_offset;

        for y in 0..HEIGHT as usize {

            let idx =
                (y * WIDTH as usize + x)
                    * 4;

            let yi = y as i32;

            if yi < wall_top {

                let ceiling_dist =
                    (HEIGHT as f32 / 2.0)
                        / ((HEIGHT as f32 / 2.0)
                            - yi as f32
                            + player.pitch);

                let world_x =
                    player.position.x
                        + ray_dir.x
                            * ceiling_dist
                            * 64.0;

                let world_y =
                    player.position.y
                        + ray_dir.y
                            * ceiling_dist
                            * 64.0;

                let color =
                    texture_u.sample(
                        world_x as usize,
                        world_y as usize,
                    );

                frame[idx] = color[0];
                frame[idx + 1] = color[1];
                frame[idx + 2] = color[2];
                frame[idx + 3] = 255;

            } else if yi > wall_bottom {

                let floor_dist =
                    (HEIGHT as f32 / 2.0)
                        / (yi as f32
                            - HEIGHT as f32 / 2.0
                            - player.pitch);

                let world_x =
                    player.position.x
                        + ray_dir.x
                            * floor_dist
                            * 64.0;

                let world_y =
                    player.position.y
                        + ray_dir.y
                            * floor_dist
                            * 64.0;

                let color =
                    texture_d.sample(
                        world_x as usize,
                        world_y as usize,
                    );

                frame[idx] = color[0];
                frame[idx + 1] = color[1];
                frame[idx + 2] = color[2];
                frame[idx + 3] = 255;

            } else {

                let tex_coord =
                    if horizontal_hit {
                        hit_x
                    } else {
                        hit_y
                    };

                let tx =
                    tex_coord as usize
                        % TEXTURE_SIZE;

                let wall_y =
                    yi as f32
                        - wall_top as f32;

                let ty =
                    ((wall_y
                        / wall_height)
                        * TEXTURE_SIZE as f32)
                        as usize;

                let color =
                    wall_texture.sample(
                        tx,
                        ty,
                    );

                let shade =
                    1.0
                        / (1.0
                            + corrected_distance
                                * 0.01);

                frame[idx] =
                    (color[0] as f32
                        * shade)
                        as u8;

                frame[idx + 1] =
                    (color[1] as f32
                        * shade)
                        as u8;

                frame[idx + 2] =
                    (color[2] as f32
                        * shade)
                        as u8;

                frame[idx + 3] = 255;
            }
        }
    }
}
