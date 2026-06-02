use std::fs;

use glam::Vec2;

use crate::sprites::SpriteInstance;
use crate::world::{
    Map,
    Sector,
    Wall,
    WallType,
};

pub fn load_map(
    path: &str,
) -> Map {

    let content =
        fs::read_to_string(path)
            .unwrap();

    let mut sectors =
        Vec::new();

    let mut current_sector:
        Option<Sector> = None;

    let mut spawn =
        Vec2::ZERO;

    let mut spawn_angle =
        0.0;

    let mut items =
        Vec::<SpriteInstance>::new();

    for line in content.lines() {

        let line =
            line.trim();

        if line.is_empty()
            || line.starts_with('#')
        {
            continue;
        }

        let parts: Vec<&str> =
            line.split_whitespace()
                .collect();

        match parts[0] {

            "sector" => {

                if let Some(sec)
                    = current_sector.take()
                {
                    sectors.push(sec);
                }

                current_sector =
                    Some(
                        Sector {

                            name:
                                parts[1].to_string(),

                            floor_texture:
                                "textureD".to_string(),

                            ceiling_texture:
                                "textureU".to_string(),

                            walls:
                                Vec::new(),
                        }
                    );
            }

            "floor" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {
                    sec.floor_texture =
                        parts[1].to_string();
                }
            }

            "ceiling" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {
                    sec.ceiling_texture =
                        parts[1].to_string();
                }
            }

            "wall" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {
                    sec.walls.push(

                        Wall {

                            start:
                                Vec2::new(
                                    parts[1].parse().unwrap(),
                                    parts[2].parse().unwrap(),
                                ),

                            end:
                                Vec2::new(
                                    parts[3].parse().unwrap(),
                                    parts[4].parse().unwrap(),
                                ),

                            texture:
                                parts[5].to_string(),

                            wall_type:
                                WallType::Solid,
                        }
                    );
                }
            }

            "portal" => {

                if let Some(sec)
                    = current_sector.as_mut()
                {
                    sec.walls.push(

                        Wall {

                            start:
                                Vec2::new(
                                    parts[1].parse().unwrap(),
                                    parts[2].parse().unwrap(),
                                ),

                            end:
                                Vec2::new(
                                    parts[3].parse().unwrap(),
                                    parts[4].parse().unwrap(),
                                ),

                            texture:
                                String::new(),

                            wall_type:
                                WallType::Portal(
                                    parts[5].to_string()
                                ),
                        }
                    );
                }
            }

            "spawn" => {

                spawn =
                    Vec2::new(
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                    );

                spawn_angle =
                    parts[3].parse().unwrap();
            }

            "item" => {

                        let rotation =
                            if parts.len() >= 5 {

                                parts[4]
                                    .parse()
                                    .unwrap_or(0.0)

                            } else {

                                0.0
                            };

                        items.push(

                            SpriteInstance {

                                sprite_id:
                                    parts[1].to_string(),

                                position:
                                    Vec2::new(
                                        parts[2]
                                            .parse()
                                            .unwrap(),

                                        parts[3]
                                            .parse()
                                            .unwrap(),
                                    ),

                                rotation,
                            }
                        );
                    }

            _ => {}
        }
    }

    if let Some(sec)
        = current_sector.take()
    {
        sectors.push(sec);
    }

    Map {
    sectors,
    spawn,
    spawn_angle,
    items,
    }
}