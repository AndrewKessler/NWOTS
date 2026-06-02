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

    pub fn from_angle(
        angle: f32,
    ) -> Self {

        let eighth =
            std::f32::consts::TAU
                / 8.0;

        let mut a =
            angle;

        while a < 0.0 {
            a +=
                std::f32::consts::TAU;
        }

        while a >=
            std::f32::consts::TAU
        {
            a -=
                std::f32::consts::TAU;
        }

        let sector =
            ((a + eighth / 2.0)
                / eighth)
                as usize
                % 8;

        match sector {

            0 => Self::F,
            1 => Self::RF,
            2 => Self::R,
            3 => Self::RB,
            4 => Self::B,
            5 => Self::LB,
            6 => Self::L,
            7 => Self::LF,

            _ => Self::F,
        }
    }
}