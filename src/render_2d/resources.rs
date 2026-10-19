use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderSettings2D {
    pub tile_pixel_size: f32,
    pub y_squish_factor: f32, // e.g., 1.0 for top-down, 0.5 for isometric projection
}

#[derive(Resource)]
pub struct TilemapAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
