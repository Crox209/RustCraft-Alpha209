use crate::mob::Mob;
use crate::level::Level;

pub struct Player {
    mob: Mob,
    is_creative: bool,
    // Placeholder for inventory, etc.
}

impl Player {
    pub fn new(level: Option<Box<Level>>, is_creative: bool) -> Self {
        Self {
            mob: Mob::new(level),
            is_creative,
        }
    }

    pub fn tick(&mut self) {
        self.mob.tick();
        // Player specific tick
    }

    pub fn is_player(&self) -> bool {
        true
    }

    pub fn is_creative_mode_allowed(&self) -> bool {
        true
    }

    // Add more methods
}