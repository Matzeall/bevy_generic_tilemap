// base modules and also plugins
pub mod core;
pub mod render_2d;
pub mod render_3d;

// public access to the plugins -> user combines Core+2d or Core+3d depending on their needs
pub use core::CoreTilemapPlugin;
pub use render_2d::Render2dTilemapPlugin;
pub use render_3d::Render3dTilemapPlugin;
