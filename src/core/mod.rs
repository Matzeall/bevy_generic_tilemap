use bevy::prelude::*;

pub mod resources;
pub mod systems;

pub struct CoreTilemapPlugin;

impl Plugin for CoreTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TileChangedMessage>();
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct TileChangedMessage {
    pub coord: UVec2,
}
