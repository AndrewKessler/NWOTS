#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SpriteDirection {
    Front,
    FrontRight,
    Right,
    BackRight,
    Back,
    BackLeft,
    Left,
    FrontLeft,
}