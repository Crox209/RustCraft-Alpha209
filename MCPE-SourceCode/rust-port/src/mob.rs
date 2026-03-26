use crate::entity::Entity;
use crate::level::Level;

pub struct Mob {
    entity: Entity,
    health: i32,
    max_health: i32,
    // Placeholder for AI, etc.
}

impl Mob {
    pub fn new(level: Option<Box<Level>>) -> Self {
        Self {
            entity: Entity::new(level),
            health: 10,
            max_health: 10,
        }
    }

    pub fn tick(&mut self) {
        self.entity.tick();
        // AI step
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn hurt(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    // Add more methods
}