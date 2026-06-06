use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ChunkStreamingFocus {
    pub center_coord: IVec2,
    pub chunk_radius: i32,
}
