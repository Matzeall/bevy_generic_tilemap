use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderSettings2D {
    pub tile_pixel_size: f32, // e.g., 32.0
    pub y_squish_factor: f32, // e.g., 1.0 for top-down, 0.5 for isometric projection
}
