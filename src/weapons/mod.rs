pub mod definition;
pub mod registry;
pub mod viewmodel;
pub mod state;

pub use definition::WeaponDefinition;
pub use registry::WeaponRegistry;
pub use viewmodel::render_viewmodel;
pub use state::WeaponState;