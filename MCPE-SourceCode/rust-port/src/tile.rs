use crate::material::Material;
use crate::aabb::Aabb;

#[derive(Clone, Debug)]
pub struct SoundType {
    pub volume: f32,
    pub pitch: f32,
    pub break_sound: String,
    pub step_sound: String,
}

impl SoundType {
    pub fn new(name: &str, volume: f32, pitch: f32) -> Self {
        Self {
            volume,
            pitch,
            break_sound: format!("step.{}", name),
            step_sound: format!("step.{}", name),
        }
    }

    pub fn with_break_sound(name: &str, break_sound: &str, volume: f32, pitch: f32) -> Self {
        Self {
            volume,
            pitch,
            step_sound: format!("step.{}", name),
            break_sound: break_sound.to_string(),
        }
    }
}

// Static sound types
lazy_static::lazy_static! {
    pub static ref SOUND_NORMAL: SoundType = SoundType::new("stone", 1.0, 1.0);
    pub static ref SOUND_WOOD: SoundType = SoundType::new("wood", 1.0, 1.0);
    pub static ref SOUND_GRAVEL: SoundType = SoundType::new("gravel", 1.0, 1.0);
    pub static ref SOUND_GRASS: SoundType = SoundType::new("grass", 1.0, 1.0);
    pub static ref SOUND_STONE: SoundType = SoundType::new("stone", 1.0, 1.0);
    pub static ref SOUND_METAL: SoundType = SoundType::new("metal", 1.0, 1.0);
    pub static ref SOUND_GLASS: SoundType = SoundType::new("glass", 1.0, 1.0);
    pub static ref SOUND_CLOTH: SoundType = SoundType::new("cloth", 1.0, 1.0);
    pub static ref SOUND_SAND: SoundType = SoundType::new("sand", 1.0, 1.0);
    pub static ref SOUND_SILENT: SoundType = SoundType::new("silent", 0.0, 1.0);
}

pub const SHAPE_INVISIBLE: i32 = -1;
pub const SHAPE_BLOCK: i32 = 0;
// Add more shapes as needed

#[derive(Clone, Debug)]
pub struct Tile {
    pub id: i32,
    pub material: Material,
    pub sound_type: SoundType,
    pub shape: i32,
    pub texture: i32,
    pub light_emission: i32,
    pub light_block: i32,
    pub hardness: f32,
    pub blast_resistance: f32,
    pub name: String,
}

impl Tile {
    pub fn new(id: i32, material: Material, sound_type: SoundType, name: &str) -> Self {
        Self {
            id,
            material,
            sound_type,
            shape: SHAPE_BLOCK,
            texture: 0,
            light_emission: 0,
            light_block: 15,
            hardness: 1.0,
            blast_resistance: 1.0,
            name: name.to_string(),
        }
    }

    pub fn get_aabb(&self, _x: i32, _y: i32, _z: i32) -> Aabb {
        Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    }

    // Add more methods
}

// Static tiles
lazy_static::lazy_static! {
    pub static ref AIR: Tile = Tile::new(0, *crate::material::AIR, SOUND_NORMAL.clone(), "air");
    pub static ref STONE: Tile = Tile::new(1, *crate::material::STONE, SOUND_STONE.clone(), "stone");
    pub static ref GRASS: Tile = Tile::new(2, *crate::material::DIRT, SOUND_GRASS.clone(), "grass");
    // Add more tiles
}

pub fn init_tiles() {
    // Initialize tiles
}