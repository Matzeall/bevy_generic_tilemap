use bevy::prelude::*;

use crate::core::TileChangedMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HexOrientation {
    #[default]
    PointyTopped,
    FlatTopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridType {
    #[default]
    Square,
    Hexagonal(HexOrientation),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LogicalTile {
    pub tile_type: u16,
}

#[derive(Resource, Debug)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub grid_type: GridType,
    pub tiles: Vec<LogicalTile>,
}

impl WorldMap {
    pub fn new(width: u32, height: u32, grid_type: GridType) -> Self {
        Self {
            width,
            height,
            grid_type,
            tiles: vec![LogicalTile::default(); (width * height) as usize],
        }
    }

    pub fn coord_to_index(&self, coord: UVec2) -> Option<usize> {
        if coord.x < self.width && coord.y < self.height {
            Some((coord.x + coord.y * self.width) as usize)
        } else {
            None
        }
    }

    pub fn set_tile(
        &mut self,
        coord: UVec2,
        tile: LogicalTile,
        writer: &mut MessageWriter<TileChangedMessage>,
    ) {
        if let Some(idx) = self.coord_to_index(coord) {
            self.tiles[idx] = tile;
            // notify the rendering plugins that a tile needs updating
            writer.write(TileChangedMessage { coord });
        }
    }
}
