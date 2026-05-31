use std::collections::HashSet;

use glam::Vec2;

use winit::keyboard::KeyCode;

use crate::world::{
    Map,
    Player,
};

use crate::physics::collision::collision_check;

use crate::util::constants::{
    WALK_SPEED,
    RUN_MULTIPLIER,
};

pub fn update_player(
    player: &mut Player,
    keys: &HashSet<KeyCode>,
    map: &Map,
) {

    let mut speed =
        WALK_SPEED;

    if keys.contains(
        &KeyCode::ShiftLeft
    ) {

        speed *=
            RUN_MULTIPLIER;
    }

    let forward =
        Vec2::new(
            player.angle.cos(),
            player.angle.sin(),
        );

    let right =
        Vec2::new(
            -player.angle.sin(),
            player.angle.cos(),
        );

    let mut new_position =
        player.position;

    if keys.contains(
        &KeyCode::KeyW
    ) {

        new_position +=
            forward * speed;
    }

    if keys.contains(
        &KeyCode::KeyS
    ) {

        new_position -=
            forward * speed;
    }

    if keys.contains(
        &KeyCode::KeyA
    ) {

        new_position -=
            right * speed;
    }

    if keys.contains(
        &KeyCode::KeyD
    ) {

        new_position +=
            right * speed;
    }

    if collision_check(
        new_position,
        map,
    ) {

        player.position =
            new_position;
    }
}