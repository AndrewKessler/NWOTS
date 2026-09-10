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
mod hud;
mod cutscene;
mod weapons;
mod gameplay;
mod audio;
mod enemies;
mod authentication;
mod crypto_namespace;
mod chain_react;
mod wallet_auth;

use engine::App;

fn main() {
    App::run();
}