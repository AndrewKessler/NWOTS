use std::collections::HashMap;

use glam::Vec2;

use crate::assets::Texture;
use crate::world::{
    Map,
    Player,
    Sector,
    Wall,
    WallType,
};

use crate::util::{
    point_in_sector,
    raycast_wall,
};

use crate::util::constants::{
    WIDTH,
    HEIGHT,
    FOV,
    TEXTURE_SIZE,
};

pub fn render_world(
    frame: &mut [u8],
    player: &Player,
    map: &Map,
    textures: &HashMap<String, Texture>,
) {

    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
        pixel[3] = 255;
    }

    for x in 0..WIDTH as usize {

        let ray_angle =
            player.angle
                - FOV / 2.0
                + (x as f32 / WIDTH as f32) * FOV;

        let ray_dir =
            Vec2::new(
                ray_angle.cos(),
                ray_angle.sin(),
            );

        let mut closest_distance =
            f32::MAX;

        let mut hit_wall: Option<&Wall> =
            None;

        let mut hit_sector: Option<&Sector> =
            None;

        let mut hit_point =
            Vec2::ZERO;

        for sector in &map.sectors {

            for wall in &sector.walls {

                if let Some((distance, point)) =
                    raycast_wall(
                        player.position,
                        ray_dir,
                        wall,
                    )
                {
                    match &wall.wall_type {

                        WallType::Solid => {

                            if distance < closest_distance {

                                closest_distance =
                                    distance;

                                hit_wall =
                                    Some(wall);

                                hit_sector =
                                    Some(sector);

                                hit_point =
                                    point;
                            }
                        }

                        WallType::Portal(_) => {
                            continue;
                        }
                    }
                }
            }
        }

        if hit_wall.is_none() {
            continue;
        }

        let wall =
            hit_wall.unwrap();

        let sector =
            hit_sector.unwrap();

        let wall_texture =
            textures
                .get(&wall.texture)
                .or_else(|| textures.get("textureN"))
                .unwrap();

        let corrected_distance =
            closest_distance
                * (player.angle - ray_angle)
                    .cos();

        let wall_height =
            (HEIGHT as f32 * 64.0)
                / corrected_distance.max(1.0);

        let wall_top =
            ((HEIGHT as f32 / 2.0)
                - wall_height / 2.0
                + player.pitch)
                as i32;

        let wall_bottom =
            ((HEIGHT as f32 / 2.0)
                + wall_height / 2.0
                + player.pitch)
                as i32;

        let dx =
            (wall.end.x - wall.start.x).abs();

        let dy =
            (wall.end.y - wall.start.y).abs();

        let hit_u =
            if dx > dy {
                hit_point.x
            } else {
                hit_point.y
            };

        let texture_x =
            hit_u.abs() as usize
                % TEXTURE_SIZE;

        // CEILING

        for y in 0..wall_top {

            if y < 0
                || y >= HEIGHT as i32
            {
                continue;
            }

            let p =
                y as f32
                    - HEIGHT as f32 / 2.0
                    - player.pitch;

            if p.abs() < 0.1 {
                continue;
            }

            let row_distance =
                (HEIGHT as f32 / 2.0)
                    / p.abs();

            let world_x =
                player.position.x
                    + ray_dir.x
                        * row_distance
                        * 64.0;

            let world_y =
                player.position.y
                    + ray_dir.y
                        * row_distance
                        * 64.0;

            let tex_x =
                world_x.abs() as usize
                    % TEXTURE_SIZE;

            let tex_y =
                world_y.abs() as usize
                    % TEXTURE_SIZE;

            let mut active_sector =
                sector;

            for test_sector in &map.sectors {

                if point_in_sector(
                    Vec2::new(
                        world_x,
                        world_y,
                    ),
                    test_sector,
                ) {

                    active_sector =
                        test_sector;

                    break;
                }
            }

            let ceiling_texture =
                textures
                    .get(&active_sector.ceiling_texture)
                    .or_else(|| textures.get("textureU"))
                    .unwrap();

            let color =
                ceiling_texture.sample(
                    tex_x,
                    tex_y,
                );

            let idx =
                ((y as usize * WIDTH as usize)
                    + x)
                    * 4;

            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }

        // WALL

        match wall.wall_type {

            WallType::Solid => {

                for y in wall_top..wall_bottom {

                    if y < 0
                        || y >= HEIGHT as i32
                    {
                        continue;
                    }

                    let wall_y =
                        y as f32
                            - wall_top as f32;

                    let texture_y =
                        ((wall_y / wall_height)
                            * TEXTURE_SIZE as f32)
                            .clamp(0.0, 63.0)
                            as usize;

                    let color =
                        wall_texture.sample(
                            texture_x,
                            texture_y,
                        );

                    let idx =
                        ((y as usize * WIDTH as usize)
                            + x)
                            * 4;

                    frame[idx] = color[0];
                    frame[idx + 1] = color[1];
                    frame[idx + 2] = color[2];
                    frame[idx + 3] = 255;
                }
            }

            WallType::Portal(_) => {}
        }

        // FLOOR

        for y in wall_bottom..HEIGHT as i32 {

            if y < 0
                || y >= HEIGHT as i32
            {
                continue;
            }

            let p =
                y as f32
                    - HEIGHT as f32 / 2.0
                    - player.pitch;

            if p.abs() < 0.1 {
                continue;
            }

            let row_distance =
                (HEIGHT as f32 / 2.0)
                    / p.abs();

            let world_x =
                player.position.x
                    + ray_dir.x
                        * row_distance
                        * 64.0;

            let world_y =
                player.position.y
                    + ray_dir.y
                        * row_distance
                        * 64.0;

            let tex_x =
                world_x.abs() as usize
                    % TEXTURE_SIZE;

            let tex_y =
                world_y.abs() as usize
                    % TEXTURE_SIZE;

            let mut active_sector =
                sector;

            for test_sector in &map.sectors {

                if point_in_sector(
                    Vec2::new(
                        world_x,
                        world_y,
                    ),
                    test_sector,
                ) {

                    active_sector =
                        test_sector;

                    break;
                }
            }

            let floor_texture =
                textures
                    .get(&active_sector.floor_texture)
                    .or_else(|| textures.get("textureD"))
                    .unwrap();

            let color =
                floor_texture.sample(
                    tex_x,
                    tex_y,
                );

            let idx =
                ((y as usize * WIDTH as usize)
                    + x)
                    * 4;

            frame[idx] = color[0];
            frame[idx + 1] = color[1];
            frame[idx + 2] = color[2];
            frame[idx + 3] = 255;
        }
    }
}