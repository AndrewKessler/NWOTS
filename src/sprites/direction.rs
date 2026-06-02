#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SpriteDirection {
    F,
    RF,
    R,
    RB,
    B,
    LB,
    L,
    LF,
}

impl SpriteDirection {

    pub fn from_str(
        value: &str,
    ) -> Option<Self> {

        match value {

            "F"  => Some(Self::F),
            "RF" => Some(Self::RF),
            "R"  => Some(Self::R),
            "RB" => Some(Self::RB),
            "B"  => Some(Self::B),
            "LB" => Some(Self::LB),
            "L"  => Some(Self::L),
            "LF" => Some(Self::LF),

            _ => None,
        }
    }
}