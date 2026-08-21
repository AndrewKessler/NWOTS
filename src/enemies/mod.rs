pub mod instance;

pub use instance::EnemyInstance;

use glam::Vec2;

use crate::world::Sector;
use crate::util::raycast_wall;
use crate::world::WallType;

pub fn update_enemy(
    enemy: &mut EnemyInstance,
    delta_time: f32,
    player_position: Vec2,
    sectors: &[Sector],
    radius: f32,
    speed: f32,
    run_frame_duration: f32,
    shot_frame_duration: f32,
    dying_frame_duration: f32,
) {

    if enemy.animation == "dying" {

        enemy.animation_timer +=
            delta_time;

        while enemy.animation_timer
            >= dying_frame_duration
        {

            enemy.animation_timer -=
                dying_frame_duration;

            enemy.animation_frame +=
                1;

            if enemy.animation_frame >= 7 {

                enemy.animation =
                    "corpse".to_string();

                enemy.animation_frame =
                    0;

                enemy.animation_timer =
                    0.0;

                break;
            }
        }

        return;
    }

    if enemy.animation == "corpse" {

        return;
    }

    if speed <= 0.0 {

        enemy.animation =
            "idle".to_string();

        enemy.animation_frame =
            0;

        enemy.animation_timer =
            0.0;

        return;
    }

    let to_player =
        player_position - enemy.position;

    let distance =
        to_player.length();

    if distance > radius {

        enemy.angle =
            to_player.y.atan2(
                to_player.x
            );
    }

    let direction =
        Vec2::new(
            enemy.angle.cos(),
            enemy.angle.sin(),
        );

    if enemy.animation == "shot" {

    let frame_duration =
        shot_frame_duration;

    enemy.animation_timer +=
        delta_time;

    while enemy.animation_timer
        >= frame_duration
    {

        enemy.animation_timer -=
            frame_duration;

        enemy.animation_frame +=
            1;

        if enemy.animation_frame >= 2 {

            enemy.animation =
                "run".to_string();

            enemy.animation_frame =
                0;

            enemy.animation_timer =
                0.0;

            break;
        }
    }

    return;
}

    let movement =
        direction
            * speed
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

    enemy.animation_timer +=
        delta_time;

    while enemy.animation_timer
        >= run_frame_duration
    {

        enemy.animation_timer -=
            run_frame_duration;

        enemy.animation_frame =
            (enemy.animation_frame + 1)
                % 4;
    }
}

pub fn damage_enemy(
    enemy: &mut EnemyInstance,
    damage: f32,
) -> bool {

    if enemy.health <= 0.0 {
        return false;
    }

    enemy.health -= damage;

    if enemy.health <= 0.0 {

        enemy.health = 0.0;

        enemy.animation =
            "dying".to_string();

        enemy.animation_frame =
            0;

        enemy.animation_timer =
            0.0;

        return true;
    }

    enemy.animation =
        "shot".to_string();

    enemy.animation_frame =
        0;

    enemy.animation_timer =
        0.0;

    false
}

pub fn hitscan_enemy(
    origin: Vec2,
    direction: Vec2,
    enemy_position: Vec2,
    radius: f32,
) -> Option<f32> {

    let to_enemy =
        enemy_position - origin;

    let projection =
        to_enemy.dot(direction);

    if projection <= 0.0 {
        return None;
    }

    let closest_point =
        origin + direction * projection;

    let distance_to_ray =
        enemy_position
            .distance(closest_point);

    if distance_to_ray > radius {
        return None;
    }

    Some(projection)
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