use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub spin: f32,

    pub cutscene:
        Option<CutsceneConfig>,

    pub menu:
        MenuConfig,

    pub episode:
        Vec<EpisodeConfig>,

    pub blockchain_auth:
        Option<BlockchainAuthConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainAuthConfig {
    pub enabled: bool,

    pub transaction_id: String,

    pub manifest: String,
}

#[derive(Debug, Deserialize)]
pub struct CutsceneConfig {
    pub path: String,
    pub fps: u32,
    pub music: String,
    pub skippable: bool,
}

#[derive(Debug, Deserialize)]
pub struct MenuConfig {
    pub font: String,
    pub background: String,
    pub music: String,
    pub move_sound: String,
    pub start_message: String,
    pub save_message: String,
    pub load_message: String,
    pub exit_message: String,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeConfig {
    pub title: String,
    pub maps: Vec<MapConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MapConfig {
    pub title: String,
    pub file: String,
    pub music: String,
    pub enter: String,
}