#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
pub enum WeaponState {

    Idle,

    Firing,

    Cooldown,
}