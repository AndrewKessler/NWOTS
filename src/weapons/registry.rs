use std::{
    collections::HashMap,
    fs,
    path::Path,
};

use crate::weapons::WeaponDefinition;

pub struct WeaponRegistry {

    weapons:
        HashMap<
            String,
            WeaponDefinition
        >,
}

impl WeaponRegistry {

    pub fn new() -> Self {

        Self {

            weapons:
                HashMap::new(),
        }
    }

    pub fn get(
        &self,
        id: &str,
    ) -> Option<&WeaponDefinition> {

        self.weapons.get(id)
    }

    pub fn load_registry(
        &mut self,
        path: &str,
    ) {

        println!(
            "Loading weapon registry: {}",
            path
        );

        let content =
            fs::read_to_string(path)
                .unwrap();

        for line in content.lines() {

            let line =
                line.trim();

            if line.is_empty()
                || line.starts_with('#')
            {
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

            let file =
                parts[1]
                    .trim();

            self.load_weapon(
                id,
                file,
            );
        }
    }

    fn load_weapon(
        &mut self,
        id: &str,
        path: &str,
    ) {

        println!(
            "Loading weapon: {}",
            id
        );

        let content =
            fs::read_to_string(path)
                .unwrap();

        let base_path =
            Path::new(path)
                .parent()
                .unwrap();

        let mut display_name =
            String::new();

        let mut pickup_ammo =
            0;

        let mut icon =
            String::new();

        let mut viewmodel =
            String::new();

        for line in content.lines() {

            let line =
                line.trim();

            if line.is_empty() {

                continue;
            }

            if line.starts_with("name") {

                display_name =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .to_string();
            }

            else if line.starts_with(
                "pickup_ammo"
            ) {

                pickup_ammo =
                    line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
            }

            else if line.starts_with(
                "icon"
            ) {

                icon =
                    base_path
                        .join(
                            line
                                .split('=')
                                .nth(1)
                                .unwrap()
                                .trim()
                        )
                        .to_string_lossy()
                        .to_string();
            }

            else if line.starts_with(
                "viewmodel"
            ) {

                viewmodel =
                    base_path
                        .join(
                            line
                                .split('=')
                                .nth(1)
                                .unwrap()
                                .trim()
                        )
                        .to_string_lossy()
                        .to_string();
            }
        }

        self.weapons.insert(

            id.to_string(),

            WeaponDefinition {

                id:
                    id.to_string(),

                display_name,

                pickup_ammo,

                icon,

                viewmodel,
            }
        );
    }
}