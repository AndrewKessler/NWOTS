#[derive(Clone)]
pub struct WeaponDefinition {

    pub id:
        String,

    pub display_name:
        String,

    pub pickup_ammo:
        u32,

    pub icon:
        String,

    pub viewmodel:
        String,

    pub damage:
        i32,

    pub fire_rate:
        f32,

    pub ammo_type:
        String,
}