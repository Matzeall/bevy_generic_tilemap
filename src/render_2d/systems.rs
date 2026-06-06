use crate::{
    core::resources::{chunk_streaming::ChunkStreamingFocus, world_map::WorldMap},
    render_2d::resources::RenderSettings2D,
};
use bevy::prelude::*;

pub fn update_streaming_focus(
    camera_query: Query<&Transform, With<Camera2d>>,
    mut focus: ResMut<ChunkStreamingFocus>,
    settings: Res<RenderSettings2D>,
    world_map: Res<WorldMap>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let cam_pos = camera_transform.translation.truncate();

    let logical_x = (cam_pos.x / settings.tile_pixel_size).round() as i32;
    let logical_y =
        (cam_pos.y / (settings.tile_pixel_size * settings.y_squish_factor)).round() as i32;

    let new_center = IVec2::new(logical_x, logical_y);

    if focus.center_coord != new_center {
        focus.center_coord = new_center;
    }
}
