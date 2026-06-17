pub struct LevelTransition {

    pub active: bool,

    pub timer: f32,

    pub next_map: String,

    pub title: String,
}

impl LevelTransition {

    pub fn new() -> Self {

        Self {

            active: false,

            timer: 0.0,

            next_map: String::new(),

            title: String::new(),
        }
    }
}