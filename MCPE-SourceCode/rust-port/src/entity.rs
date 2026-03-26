use crate::entity_core::EntityCore;
use crate::level::Level;
use crate::vec3::Vec3;
use crate::aabb::Aabb;

pub struct Entity {
    core: EntityCore,
    level: Option<Box<Level>>, // Placeholder, use Rc or something
    // Add more fields as needed
}

impl Entity {
    pub fn new(level: Option<Box<Level>>) -> Self {
        Self {
            core: EntityCore::default(),
            level,
        }
    }

    pub fn tick(&mut self) {
        // Update position
        self.core.xo = self.core.x;
        self.core.yo = self.core.y;
        self.core.zo = self.core.z;
        self.core.x += self.core.xd;
        self.core.y += self.core.yd;
        self.core.z += self.core.zd;
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.core.x = x;
        self.core.y = y;
        self.core.z = z;
    }

    pub fn move_to(&mut self, x: f32, y: f32, z: f32, y_rot: f32, x_rot: f32) {
        self.set_pos(x, y, z);
        self.core.y_rot = y_rot;
        self.core.x_rot = x_rot;
    }

    pub fn is_alive(&self) -> bool {
        true // Placeholder
    }

    pub fn is_player(&self) -> bool {
        false
    }

    // Add more methods
}