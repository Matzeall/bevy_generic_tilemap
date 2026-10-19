use bevy::{prelude::*, window::PrimaryWindow};
use bevy_generic_tilemap::{
    CoreTilemapPlugin, Render2dTilemapPlugin,
    core::{
        TileChangedMessage,
        resources::world_map::{GridType, LogicalTile, WorldMap},
    },
    render_2d::resources::{RenderSettings2D, TilemapAsset},
};
use rand::random_range;

fn main() {
    let map = WorldMap::new(10, 10, GridType::Square);
    let resource = RenderSettings2D {
        tile_pixel_size: 128.0,
        y_squish_factor: 1.0,
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_generic_tilemap".to_string(),
                        resolution: [1200, 800].into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(map)
        .insert_resource(resource)
        .add_plugins(CoreTilemapPlugin)
        .add_plugins(Render2dTilemapPlugin)
        .add_systems(Startup, setup_tilemap_assets)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_randome_tile)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 0.0)));
}

fn update_randome_tile(mut map: ResMut<WorldMap>, mut messages: MessageWriter<TileChangedMessage>) {
    let x = random_range(0..(map.width - 1));
    let y = random_range(0..(map.height - 1));

    if let Some(tile_num) = map.coord_to_index((x, y).into()) {
        let mut new_tile = map.tiles[tile_num];
        new_tile.tile_type = random_range(0..20);

        map.set_tile((x, y).into(), new_tile, &mut messages);
    }
}

fn setup_tilemap_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image_handle = asset_server.load("tilesheet_block.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(128), // dimensions of an individual cell
        20,                // total number of horizontal columns in the sheet
        5,                 // total number of vertical rows in the sheet
        None,              // optional spacing/padding between cells (Option<UVec2>)
        None,              // optional coordinate offset from the top-left corner (Option<UVec2>)
    );

    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(TilemapAsset {
        image: image_handle,
        layout: layout_handle,
    });
}
