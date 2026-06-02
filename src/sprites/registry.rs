use std::{
    collections::HashMap,
    fs,
    path::Path,
};

use crate::assets::Texture;

use crate::sprites::{
    SpriteDefinition,
    SpriteDirection,
    SpriteFrame,
};

pub struct SpriteRegistry {

    pub sprites:
        HashMap<
            String,
            SpriteDefinition,
        >,
}

impl SpriteRegistry {

    pub fn new() -> Self {

        Self {

            sprites:
                HashMap::new(),
        }
    }

    pub fn load_test_assets(
        &mut self,
    ) {

        self.load_sprite_definition(
            "colt",
            "assets/items/weapons/colt/colt.txt",
        );
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&SpriteDefinition> {

        self.sprites.get(name)
    }

    fn load_sprite_definition(
        &mut self,
        id: &str,
        path: &str,
    ) {

        println!(
            "Loading sprite definition: {}",
            id
        );

        let content =
            fs::read_to_string(path)
                .unwrap();

        let base_path =
            Path::new(path)
                .parent()
                .unwrap();

        let mut name =
            String::new();

        let mut radius =
            8.0;

        let mut height =
            16.0;

        let mut scale_x =
            1.0;

        let mut scale_y =
            1.0;

        let mut frames =
            HashMap::new();

        let mut current_direction:
            Option<SpriteDirection> =
                None;

        let mut current_image:
            Option<String> =
                None;

        let mut offset_x =
            0;

        let mut offset_y =
            0;

        for line in content.lines() {

            let line =
                line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("name") {

                name =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .to_string();
            }

            else if line.starts_with("radius") {

                radius =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("height") {

                height =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("scale_x") {

                scale_x =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("scale_y") {

                scale_y =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("frame") {

                if let (
                    Some(direction),
                    Some(image_file),
                ) = (
                    current_direction,
                    current_image.take(),
                ) {

                    let texture =
                        Texture::load(
                            base_path
                                .join(
                                    image_file
                                )
                                .to_str()
                                .unwrap()
                        );

                    frames.insert(

                        direction,

                        SpriteFrame {

                            image:
                                texture,

                            offset_x,

                            offset_y,
                        },
                    );
                }

                let direction =
                    line
                        .split_whitespace()
                        .nth(1)
                        .unwrap();

                current_direction =
                    SpriteDirection::from_str(
                        direction
                    );

                offset_x = 0;
                offset_y = 0;
            }

            else if line.starts_with("image") {

                current_image =
                    Some(
                        line
                            .split('=')
                            .nth(1)
                            .unwrap()
                            .trim()
                            .to_string()
                    );
            }

            else if line.starts_with("offset_x") {

                offset_x =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("offset_y") {

                offset_y =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }
        }

        if let (
            Some(direction),
            Some(image_file),
        ) = (
            current_direction,
            current_image,
        ) {

            let texture =
                Texture::load(
                    base_path
                        .join(image_file)
                        .to_str()
                        .unwrap()
                );

            frames.insert(

                direction,

                SpriteFrame {

                    image:
                        texture,

                    offset_x,

                    offset_y,
                },
            );
        }

        self.sprites.insert(

            id.to_string(),

            SpriteDefinition {

                name,

                radius,

                height,

                scale_x,

                scale_y,

                frames,
            },
        );
    }
}