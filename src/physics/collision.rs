use glam::Vec2;

use crate::world::{
    Map,
    WallType,
};

pub fn collision_check(
    position: Vec2,
    map: &Map,
) -> bool {

    for sector in &map.sectors {

        for wall in &sector.walls {

            if let WallType::Solid =
                wall.wall_type
            {

                let wall_dir =
                    wall.end - wall.start;

                let wall_length =
                    wall_dir.length();

                let wall_normal =
                    wall_dir.normalize();

                let to_player =
                    position - wall.start;

                let projection =
                    to_player.dot(
                        wall_normal
                    );

                if projection >= 0.0
                    &&
                    projection <= wall_length
                {

                    let closest =
                        wall.start
                            + wall_normal
                                * projection;

                    let distance =
                        (position - closest)
                            .length();

                    if distance < 10.0 {
                        return false;
                    }
                }
            }
        }
    }

    true
}