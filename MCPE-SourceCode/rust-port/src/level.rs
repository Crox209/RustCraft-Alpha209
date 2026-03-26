use crate::entity::Entity;
use crate::level_data::LevelData;
use crate::level_storage_source::LevelStorageSource;
use crate::chunk_pos::ChunkPos;
use crate::level_chunk::LevelChunk;
use std::collections::HashMap;

pub struct Level {
    level_data: LevelData,
    level_storage: Box<dyn LevelStorageSource>,
    chunks: HashMap<ChunkPos, LevelChunk>,
    entities: Vec<Entity>,
    // Placeholder for other fields
}

impl Level {
    pub fn new(level_storage: Box<dyn LevelStorageSource>, level_name: &str, settings: (), generator_version: i32) -> Self {
        let mut level = Self {
            level_data: LevelData::new(level_name.to_string()),
            level_storage,
            chunks: HashMap::new(),
            entities: Vec::new(),
        };
        level.generate_flat_world(1);
        level
    }

    fn get_chunk_mut(&mut self, chunk_x: i32, chunk_z: i32) -> &mut LevelChunk {
        self.chunks
            .entry(ChunkPos::new(chunk_x, chunk_z))
            .or_insert_with(|| LevelChunk::new(chunk_x, chunk_z))
    }

    pub fn generate_flat_world(&mut self, radius: i32) {
        for cx in -radius..=radius {
            for cz in -radius..=radius {
                let chunk = self.get_chunk_mut(cx, cz);
                for x in 0..16 {
                    for z in 0..16 {
                        for y in 0..128 {
                            let tile_id = if y == 64 {
                                crate::tile::GRASS.id as u8
                            } else if y < 64 {
                                crate::tile::STONE.id as u8
                            } else {
                                crate::tile::AIR.id as u8
                            };
                            chunk.set_tile_raw(x, y, z, tile_id);
                        }
                    }
                }
            }
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn tick_entities(&mut self) {
        for entity in &mut self.entities {
            entity.tick();
        }
    }

    pub fn get_tile(&self, x: i32, y: i32, z: i32) -> i32 {
        if y < 0 || y >= 128 {
            return crate::tile::AIR.id;
        }

        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let block_x = x.rem_euclid(16) as usize;
        let block_z = z.rem_euclid(16) as usize;
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            chunk.get_tile(block_x, y as usize, block_z) as i32
        } else {
            crate::tile::AIR.id
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, z: i32, tile: i32) -> bool {
        if y < 0 || y >= 128 {
            return false;
        }

        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let block_x = x.rem_euclid(16) as usize;
        let block_z = z.rem_euclid(16) as usize;

        let chunk = self.get_chunk_mut(chunk_x, chunk_z);
        chunk.set_tile_and_data(block_x, y as usize, block_z, tile as u8, 0)
    }

    pub fn has_chunk_at(&self, x: i32, _y: i32, z: i32) -> bool {
        let chunk_pos = ChunkPos::new(x >> 4, z >> 4);
        self.chunks.contains_key(&chunk_pos)
    }

    // Add more methods as needed
}