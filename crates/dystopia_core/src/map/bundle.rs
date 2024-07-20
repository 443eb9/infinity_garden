use bevy::{
    prelude::Bundle,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    transform::components::{GlobalTransform, Transform},
};

use crate::map::tilemap::{
    TileAtlasIndex, TileBindedTilemap, TileIndex, TileRenderSize, TileTint, TilemapStorage,
    TilemapTilesets, TilemapTint,
};

#[derive(Bundle, Default)]
pub struct TilemapBundle {
    pub tile_render_size: TileRenderSize,
    pub storgae: TilemapStorage,
    pub tilesets: TilemapTilesets,
    pub tint: TilemapTint,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub inherited_visibility: InheritedVisibility,
}

#[derive(Bundle, Default)]
pub struct TileBundle {
    pub binded_tilemap: TileBindedTilemap,
    pub index: TileIndex,
    pub atlas_index: TileAtlasIndex,
    pub tint: TileTint,
    pub visibility: Visibility,
}