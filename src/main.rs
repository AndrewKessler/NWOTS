mod config;
mod engine;
mod world;
mod map_loader;
mod assets;
mod render;
mod physics;
mod input;
mod util;
mod sprites;

use engine::App;

fn main() {
    App::run();
}