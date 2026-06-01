use std::collections::HashMap;
use std::fs;

use crate::assets::Texture;

pub struct TextureRegistry {
    pub textures: HashMap<String, Texture>,
}

impl TextureRegistry {
    pub fn load(
        path: &str,
    ) -> Self {

        let mut textures =
            HashMap::new();

        let content =
            fs::read_to_string(path)
                .unwrap();

        let mut texture_count =
            0usize;

        for line in content.lines() {

            let line =
                line.trim();

            if line.is_empty()
                || line.starts_with('#')
            {
                continue;
            }

            let parts: Vec<&str> =
                line.split('=')
                    .collect();

            if parts.len() != 2 {
                continue;
            }

            let name =
                parts[0]
                    .trim()
                    .to_string();

            let texture_path =
                parts[1]
                    .trim();

            println!(
            "Loading texture: {} -> {}",
            name,
            texture_path
            );

textures.insert(
    name,
    Texture::load(texture_path),
);

            texture_count += 1;

            if texture_count >= 256 {
                panic!(
                    "Texture limit exceeded (256)"
                );
            }
        }

        Self {
            textures,
        }
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&Texture> {

        self.textures.get(name)
    }
}