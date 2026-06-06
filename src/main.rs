use bevy::prelude::*;
use bevy_generic_tilemap::{
    CoreTilemapPlugin, Render2dTilemapPlugin,
    core::{
        TileChangedMessage,
        resources::world_map::{GridType, LogicalTile, WorldMap},
    },
};
use rand::random_range;

fn main() {
    let map = WorldMap::new(10, 10, GridType::Square);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_generic_tilemap".to_string(),
                resolution: [1200, 800].into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(map)
        .add_plugins(CoreTilemapPlugin)
        .add_plugins(Render2dTilemapPlugin) // Settle on 2D for now
        .add_systems(Update, update_randome_tile)
        .run();
}

fn update_randome_tile(mut map: ResMut<WorldMap>, mut messages: MessageWriter<TileChangedMessage>) {
    let x = random_range(0..(map.width - 1));
    let y = random_range(0..(map.height - 1));

    let mut new_tile = LogicalTile::default();
    new_tile.tile_type = if new_tile.tile_type == 1 { 0 } else { 1 };

    map.set_tile((x, y).into(), new_tile, &mut messages);
}
