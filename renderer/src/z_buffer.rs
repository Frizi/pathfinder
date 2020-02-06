// pathfinder/renderer/src/z_buffer.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Software occlusion culling.

use crate::gpu_data::SolidTileBatchPrimitive;
use crate::paint::PaintMetadata;
use crate::scene::PathObject;
use crate::tile_map::DenseTileMap;
use crate::tiles;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::transform2d::Transform2I;
use pathfinder_geometry::vector::Vector2I;
use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

pub struct ZBuffer {
    buffer: DenseTileMap<AtomicUsize>,
}

impl ZBuffer {
    pub fn new(view_box: RectF) -> ZBuffer {
        let tile_rect = tiles::round_rect_out_to_tile_bounds(view_box);
        ZBuffer {
            buffer: DenseTileMap::from_builder(|_| AtomicUsize::new(0), tile_rect),
        }
    }

    pub fn test(&self, coords: Vector2I, object_index: u32) -> bool {
        let tile_index = self.buffer.coords_to_index_unchecked(coords);
        let existing_depth = self.buffer.data[tile_index as usize].load(AtomicOrdering::SeqCst);
        existing_depth < object_index as usize + 1
    }

    pub fn update(&self, coords: Vector2I, object_index: u16) {
        let tile_index = self.buffer.coords_to_index_unchecked(coords);
        let mut old_depth = self.buffer.data[tile_index].load(AtomicOrdering::SeqCst);
        let new_depth = (object_index + 1) as usize;
        while old_depth < new_depth {
            let prev_depth = self.buffer.data[tile_index].compare_and_swap(
                old_depth,
                new_depth,
                AtomicOrdering::SeqCst,
            );
            if prev_depth == old_depth {
                // Successfully written.
                return;
            }
            old_depth = prev_depth;
        }
    }

    pub fn build_solid_tiles(&self,
                             paths: &[PathObject],
                             paint_metadata: &[PaintMetadata],
                             object_range: Range<u32>)
                             -> Vec<SolidTileBatchPrimitive> {
        let mut solid_tiles = vec![];
        for tile_index in 0..self.buffer.data.len() {
            let depth = self.buffer.data[tile_index].load(AtomicOrdering::Relaxed);
            if depth == 0 {
                continue;
            }

            let tile_coords = self.buffer.index_to_coords(tile_index);
            let object_index = (depth - 1) as u32;
            if object_index < object_range.start || object_index >= object_range.end {
                continue;
            }

            let paint_id = paths[object_index as usize].paint();
            let tex_transform = paint_metadata[paint_id.0 as usize].tex_transform;

            solid_tiles.push(SolidTileBatchPrimitive::new(tile_coords + self.buffer.rect.origin(),
                                                          object_index as u16,
                                                          tex_transform));
        }

        solid_tiles
    }
}

impl SolidTileBatchPrimitive {
    fn new(tile_coords: Vector2I, object_index: u16, tex_transform: Transform2I)
           -> SolidTileBatchPrimitive {
        SolidTileBatchPrimitive {
            tile_x: tile_coords.x() as i16,
            tile_y: tile_coords.y() as i16,
            object_index: object_index,
            texture_m00: tex_transform.matrix.m11() as u16,
            texture_m10: tex_transform.matrix.m21() as u16,
            texture_m01: tex_transform.matrix.m12() as u16,
            texture_m11: tex_transform.matrix.m22() as u16,
            texture_m02: tex_transform.vector.x() as u16,
            texture_m12: tex_transform.vector.y() as u16,
            pad: 0,
        }
    }
}
