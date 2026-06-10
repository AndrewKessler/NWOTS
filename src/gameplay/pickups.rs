use crate::world::{
    Map,
    Player,
};

pub fn pickup_items(
    player: &mut Player,
    map: &mut Map,
) {

    map.items.retain(
        |item| {

            let distance =
                player.position
                    .distance(
                        item.position
                    );

            if item.sprite_id
                == "colt"
                &&
                distance < 16.0
            {

                if !player
                    .inventory
                    .has_item(
                        "colt"
                    )
                {

                    println!(
                        "Picked up Colt"
                    );

                    player
                        .inventory
                        .add_item(
                            "colt",
                            1,
                        );

                    player
                        .inventory
                        .equipped_weapon =
                            Some(
                                "colt"
                                    .to_string()
                            );

                    player
                        .stats
                        .ammo += 10;
                }

                false
            }
            else {

                true
            }
        }
    );
}