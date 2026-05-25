use glam::Vec2;
use pixels::{Pixels, SurfaceTexture};
use std::f32::consts::PI;
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const ROOM_SIZE: f32 = 512.0;
const TEXTURE_SIZE: usize = 64;
const FOV: f32 = PI / 3.0;

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
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Minerva Base")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
            .build(&event_loop)
            .unwrap(),
    );

    let surface_texture =
        SurfaceTexture::new(WIDTH, HEIGHT, window.clone());

    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)
        .expect("failed to create pixel surface");

    let texture_n = Texture::load("assets/textures/textureN.png");
    let texture_s = Texture::load("assets/textures/textureS.png");
    let texture_e = Texture::load("assets/textures/textureE.png");
    let texture_w = Texture::load("assets/textures/textureW.png");
    let texture_u = Texture::load("assets/textures/textureU.png");
    let texture_d = Texture::load("assets/textures/textureD.png");

    let mut player = Player {
        position: Vec2::new(ROOM_SIZE / 2.0, ROOM_SIZE / 2.0),
        angle: 0.0,
    };

    event_loop
        .run(move |event, target| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => target.exit(),

                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key,
                                    state,
                                    ..
                                },
                            ..
                        } => {
                            if state == ElementState::Pressed {
                                match physical_key {
                                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                                        player.angle -= 0.08;
                                    }

                                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                                        player.angle += 0.08;
                                    }

                                    _ => {}
                                }
                            }
                        }

                        WindowEvent::RedrawRequested => {
                            let frame = pixels.frame_mut();

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
                    window.as_ref().request_redraw();
                }

                _ => {}
            }
        })
        .unwrap();
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
    for x in 0..WIDTH as usize {
        let ray_angle = player.angle
            - FOV / 2.0
            + (x as f32 / WIDTH as f32) * FOV;

        let ray_dir = Vec2::new(ray_angle.cos(), ray_angle.sin());

        let mut distance = 0.0;
        let step = 1.0;

        let mut hit_x = 0.0;
        let mut hit_y = 0.0;

        let mut wall_texture = texture_n;
        let mut horizontal_hit = false;

        while distance < 2000.0 {
            let pos = player.position + ray_dir * distance;

            if pos.x <= 0.0 {
                wall_texture = texture_w;
                hit_x = pos.x;
                hit_y = pos.y;
                horizontal_hit = false;
                break;
            }

            if pos.x >= ROOM_SIZE {
                wall_texture = texture_e;
                hit_x = pos.x;
                hit_y = pos.y;
                horizontal_hit = false;
                break;
            }

            if pos.y <= 0.0 {
                wall_texture = texture_n;
                hit_x = pos.x;
                hit_y = pos.y;
                horizontal_hit = true;
                break;
            }

            if pos.y >= ROOM_SIZE {
                wall_texture = texture_s;
                hit_x = pos.x;
                hit_y = pos.y;
                horizontal_hit = true;
                break;
            }

            distance += step;
        }

        let corrected_distance =
            distance * (player.angle - ray_angle).cos();

        let wall_height =
            (HEIGHT as f32 * 64.0) / corrected_distance.max(1.0);

        let wall_top = ((HEIGHT as f32 / 2.0)
            - wall_height / 2.0)
            .max(0.0) as usize;

        let wall_bottom = ((HEIGHT as f32 / 2.0)
            + wall_height / 2.0)
            .min(HEIGHT as f32 - 1.0) as usize;

        for y in 0..HEIGHT as usize {
            let idx = (y * WIDTH as usize + x) * 4;

            if y < wall_top {
                let ceiling_dist = (HEIGHT as f32 / 2.0)
                    / ((HEIGHT as f32 / 2.0) - y as f32);

                let world_x = player.position.x
                    + ray_dir.x * ceiling_dist * 64.0;

                let world_y = player.position.y
                    + ray_dir.y * ceiling_dist * 64.0;

                let tx = world_x as usize;
                let ty = world_y as usize;

                let color = texture_u.sample(tx, ty);

                frame[idx] = color[0];
                frame[idx + 1] = color[1];
                frame[idx + 2] = color[2];
                frame[idx + 3] = 255;
            } else if y > wall_bottom {
                let floor_dist = (HEIGHT as f32 / 2.0)
                    / (y as f32 - HEIGHT as f32 / 2.0);

                let world_x = player.position.x
                    + ray_dir.x * floor_dist * 64.0;

                let world_y = player.position.y
                    + ray_dir.y * floor_dist * 64.0;

                let tx = world_x as usize;
                let ty = world_y as usize;

                let color = texture_d.sample(tx, ty);

                frame[idx] = color[0];
                frame[idx + 1] = color[1];
                frame[idx + 2] = color[2];
                frame[idx + 3] = 255;
            } else {
                let tex_coord = if horizontal_hit {
                    hit_x
                } else {
                    hit_y
                };

                let tx = tex_coord as usize % TEXTURE_SIZE;

                let wall_y = y as f32 - wall_top as f32;

                let ty = ((wall_y / wall_height)
                    * TEXTURE_SIZE as f32) as usize;

                let color = wall_texture.sample(tx, ty);

                let shade =
                    1.0 / (1.0 + corrected_distance * 0.01);

                frame[idx] = (color[0] as f32 * shade) as u8;
                frame[idx + 1] =
                    (color[1] as f32 * shade) as u8;
                frame[idx + 2] =
                    (color[2] as f32 * shade) as u8;
                frame[idx + 3] = 255;
            }
        }
    }
}
