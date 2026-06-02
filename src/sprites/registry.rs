use std::collections::HashMap;

use crate::assets::Texture;

pub struct SpriteRegistry {
    pub images: HashMap<String, Texture>,
}

impl SpriteRegistry {

    pub fn new() -> Self {

        Self {
            images: HashMap::new(),
        }
    }

    pub fn load_test_assets(
        &mut self,
    ) {

        println!(
            "Loading sprite: colt"
        );

        self.images.insert(
            "colt".to_string(),
            Texture::load(
                "assets/items/weapons/colt/coltF.png"
            ),
        );
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&Texture> {

        self.images.get(name)
    }
}