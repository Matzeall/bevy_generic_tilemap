use crate::{
    core::{
        TileChangedMessage,
        resources::world_map::{GridType, WorldMap},
    },
    render_2d::resources::{RenderSettings2D, TilemapAsset},
};
use bevy::{platform::collections::HashMap, prelude::*};

pub mod resources;
mod systems;

pub struct Render2dTilemapPlugin;

impl Plugin for Render2dTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileEntities>()
            .add_systems(Update, refresh_dirty_tiles);
    }
}

#[derive(Resource, Default)]
struct TileEntities(HashMap<UVec2, Entity>);

fn refresh_dirty_tiles(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    settings: Res<RenderSettings2D>,
    tilemap_asset: Res<TilemapAsset>,
    mut tile_entities: ResMut<TileEntities>,
    mut messages: MessageReader<TileChangedMessage>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for message in messages.read() {
        let coord = message.coord; // the dirty coord

        let Some(idx) = world_map.coord_to_index(coord) else {
            continue;
        };
        let logical_tile = world_map.tiles[idx];

        // convert grid coordinates to 2D pixel space based on grid type
        let world_pos = match world_map.grid_type {
            GridType::Square => Vec2::new(
                coord.x as f32 * settings.tile_pixel_size,
                coord.y as f32 * settings.tile_pixel_size,
            ),
            GridType::Hexagonal(orientation) => {
                // skew layout coordinates for hex calculations
                let x = settings.tile_pixel_size
                    * (3.0f32.sqrt() * coord.x as f32 + 3.0f32.sqrt() / 2.0 * coord.y as f32);
                let y = settings.tile_pixel_size * (3.0 / 2.0 * coord.y as f32);
                Vec2::new(x, y)
            }
        };

        // render the tile
        if let Some(&entity) = tile_entities.0.get(&coord) {
            if let Ok(mut sprite) = sprite_query.get_mut(entity) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = logical_tile.tile_type as usize;
                }
            }
        } else {
            let entity = commands
                .spawn((
                    Sprite {
                        image: tilemap_asset.image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: tilemap_asset.layout.clone(),
                            index: logical_tile.tile_type as usize,
                        }),
                        ..default()
                    },
                    Transform::from_translation(world_pos.extend(0.0)),
                ))
                .id();

            tile_entities.0.insert(coord, entity);
        }
    }
}
