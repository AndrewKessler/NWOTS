#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum GameState {

    Cutscene,

    Menu,

    Playing,

    Exit,
}