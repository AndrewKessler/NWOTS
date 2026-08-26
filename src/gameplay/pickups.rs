use crate::world::{
    Map,
    Player,
};

use crate::weapons::WeaponRegistry;

use crate::audio::AudioManager;

pub fn pickup_items(
    player: &mut Player,
    map: &mut Map,
    weapons: &WeaponRegistry,
    audio: &mut AudioManager,
) {

    map.items.retain(
        |item| {

            let distance =
                player.position
                    .distance(
                        item.position
                    );

            if let Some(
                weapon
            ) =
                weapons.get(
                    &item.sprite_id
                )
            {

                if distance < 16.0
                {

                    if !player
                        .inventory
                        .has_item(
                            &weapon.id
                        )
                    {

                        println!(
                            "Picked up {}",
                            weapon.display_name
                        );

                        player
                            .inventory
                            .add_item(
                                &weapon.id,
                                1,
                            );

                        player
                            .inventory
                            .equipped_weapon =
                                Some(
                                    weapon.id.clone()
                                );

                        player
                            .stats
                            .ammo +=
                                weapon
                                    .pickup_ammo
                                    as i32;
                    }

                    false

                } else {

                    true
                }

            } else if item.sprite_id
                == "ammo_colt"
            {

                if distance < 16.0 {

                    player.stats.ammo +=
                        30;

                    println!(
                        "Picked up Colt Ammo: +30"
                    );

                    audio.play_sound(
                        "assets/items/ammo/colt/pickup.mp3"
                    );

                    false

                } else {

                    true
                }

            } else {

                true
            }
        }
    );
}