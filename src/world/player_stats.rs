#[derive(Debug, Clone)]
pub struct PlayerStats {

    pub health: i32,
    pub max_health: i32,

    pub armour: i32,
    pub max_armour: i32,

    pub stamina: i32,
    pub max_stamina: i32,

    pub luck: i32,
    pub max_luck: i32,

    pub power: i32,
    pub max_power: i32,

    pub ammo: i32,
}

impl Default for PlayerStats {

    fn default() -> Self {

        Self {

            health: 100,
            max_health: 100,

            armour: 0,
            max_armour: 100,

            stamina: 100,
            max_stamina: 100,

            luck: 0,
            max_luck: 100,

            power: 0,
            max_power: 100,

            ammo: 0,
        }
    }
}