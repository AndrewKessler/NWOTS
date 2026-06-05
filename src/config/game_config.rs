use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameConfig {

    pub cutscene:
        Option<CutsceneConfig>,

    pub menu:
        MenuConfig,

    pub episode:
        Vec<EpisodeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CutsceneConfig {

    pub file: String,

    pub skippable: bool,
}

#[derive(Debug, Deserialize)]
pub struct MenuConfig {

    pub font: String,

    pub background: String,

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
}