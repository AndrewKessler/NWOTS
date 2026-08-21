use std::{
    collections::HashMap,
    fs,
    path::Path,
};

use crate::assets::Texture;

use crate::sprites::{
    SpriteAnimation,
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

    pub fn load_registry(
        &mut self,
        path: &str,
    ) {

        println!(
            "Loading item registry: {}",
            path
        );

        let content =
            fs::read_to_string(path)
                .unwrap();

        for line in content.lines() {

            let line =
                line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with('#') {
                continue;
            }

            let parts:
                Vec<&str> =
                line
                    .split('=')
                    .collect();

            if parts.len() != 2 {
                continue;
            }

            let id =
                parts[0]
                    .trim();

            let definition_path =
                parts[1]
                    .trim();

            self.load_sprite_definition(
                id,
                definition_path,
            );
        }
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

        let mut ground_offset =
            0.0;

        let mut scale_x =
            1.0;

        let mut scale_y =
            1.0;

        let mut health =
            100.0;

        let mut speed =
            100.0;

        let mut animations:
            HashMap<
                String,
                SpriteAnimation,
            > =
            HashMap::new();

        let mut current_animation =
            String::from("idle");

        let mut current_frame_duration =
            0.20;

        let mut current_ground_offset =
            ground_offset;

        let mut in_animation =
            false;

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

            if line.starts_with('#') {
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

            else if line.starts_with(
                "ground_offset"
            ) {

                let value =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();

                if in_animation {

                    current_ground_offset =
                        value;

                } else {

                    ground_offset =
                        value;

                    current_ground_offset =
                        value;
                }
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

            else if line.starts_with("health") {

                health =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("speed") {

                speed =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with(
                "frame_duration"
            ) {

                current_frame_duration =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with("animation") {

                in_animation =
                    true;

                current_ground_offset =
                    ground_offset;

                if let (
                    Some(direction),
                    Some(image_file),
                ) = (
                    current_direction,
                    current_image.take(),
                ) {

                    let texture =
                        Texture::load(
                            &base_path
                                .join(
                                    image_file
                                )
                                .to_str()
                                .unwrap()
                        );

                    animations
                        .entry(
                            current_animation
                                .clone()
                        )
                        .or_insert_with(|| {

                            SpriteAnimation {

                                frame_duration:
                                    current_frame_duration,

                                ground_offset:
                                    current_ground_offset,

                                frames:
                                    HashMap::new(),
                            }
                        })
                        .frames
                        .entry(direction)
                        .or_insert_with(
                            Vec::new
                        )
                        .push(
                            SpriteFrame {

                                image:
                                    texture,

                                offset_x,

                                offset_y,
                            }
                        );
                }

                current_animation =
                    line
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .to_string();
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
                            &base_path
                                .join(
                                    image_file
                                )
                                .to_str()
                                .unwrap()
                        );

                    animations
                        .entry(
                            current_animation
                                .clone()
                        )
                        .or_insert_with(|| {

                            SpriteAnimation {

                                frame_duration:
                                    current_frame_duration,

                                ground_offset:
                                    current_ground_offset,

                                frames:
                                    HashMap::new(),
                            }
                        })
                        .frames
                        .entry(direction)
                        .or_insert_with(
                            Vec::new
                        )
                        .push(
                            SpriteFrame {

                                image:
                                    texture,

                                offset_x,

                                offset_y,
                            }
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
                    &base_path
                        .join(image_file)
                        .to_str()
                        .unwrap()
                );

            animations
                .entry(
                    current_animation
                        .clone()
                )
                .or_insert_with(|| {

                    SpriteAnimation {

                        frame_duration:
                            current_frame_duration,

                        ground_offset:
                            current_ground_offset,

                        frames:
                            HashMap::new(),
                    }
                })
                .frames
                .entry(direction)
                .or_insert_with(
                    Vec::new
                )
                .push(
                    SpriteFrame {

                        image:
                            texture,

                        offset_x,

                        offset_y,
                    }
                );
        }

        self.sprites.insert(

            id.to_string(),

            SpriteDefinition {

                name,

                radius,

                height,

                ground_offset,

                scale_x,

                scale_y,

                health,

                speed,

                animations,
            }
        );
    }
}