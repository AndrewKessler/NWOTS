pub mod instance;

pub use instance::EnemyInstance;

use glam::Vec2;

use crate::world::Sector;
use crate::util::raycast_wall;
use crate::world::WallType;

pub fn update_enemy(
    enemy: &mut EnemyInstance,
    delta_time: f32,
    sectors: &[Sector],
    radius: f32,
) {

    if enemy.speed <= 0.0 {

        enemy.animation =
            "idle".to_string();

        enemy.animation_frame =
            0;

        enemy.animation_timer =
            0.0;

        return;
    }

    let direction =
        Vec2::new(
            enemy.angle.cos(),
            enemy.angle.sin(),
        );

    let movement =
        direction
            * enemy.speed
            * delta_time;

    let new_position =
        enemy.position
            + movement;

    if can_move_to(
        enemy.position,
        new_position,
        radius,
        sectors,
    ) {

        enemy.position =
            new_position;
    }

    enemy.animation =
        "run".to_string();

    let frame_duration =
        0.12;

    enemy.animation_timer +=
        delta_time;

    while enemy.animation_timer
        >= frame_duration
    {

        enemy.animation_timer -=
            frame_duration;

        enemy.animation_frame =
            (enemy.animation_frame + 1)
                % 4;
    }
}

fn can_move_to(
    current: Vec2,
    target: Vec2,
    radius: f32,
    sectors: &[Sector],
) -> bool {

    let movement =
        target - current;

    let distance =
        movement.length();

    if distance <= 0.0 {
        return true;
    }

    let direction =
        movement / distance;

    for sector in sectors {

        for wall in &sector.walls {

            if !matches!(
                wall.wall_type,
                WallType::Solid
            ) {
                continue;
            }

            if let Some((wall_distance, _)) =
                raycast_wall(
                    current,
                    direction,
                    wall,
                )
            {

                if wall_distance
                    <= distance + radius
                {
                    return false;
                }
            }
        }
    }

    true
}