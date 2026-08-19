pub mod instance;

pub use instance::EnemyInstance;

use glam::Vec2;

pub fn update_enemy(
    enemy: &mut EnemyInstance,
    delta_time: f32,
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

    // Move in the direction
    // the enemy is facing.

    let direction =
        Vec2::new(
            enemy.angle.cos(),
            enemy.angle.sin(),
        );

    enemy.position +=
        direction
            * enemy.speed
            * delta_time;

    enemy.animation =
        "run".to_string();

    // Four-frame animation.

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