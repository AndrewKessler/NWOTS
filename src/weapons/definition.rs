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

    pub idle_frame:
        String,

    pub fire_frames:
        usize,

    pub damage:
        i32,

    pub fire_rate:
        f32,

    pub ammo_type:
        String,
}