use crate::player::Player;
use crate::level::Level;

pub struct LocalPlayer {
    player: Player,
    // Client-specific fields
}

impl LocalPlayer {
    pub fn new(level: Option<Box<Level>>, is_creative: bool) -> Self {
        Self {
            player: Player::new(level, is_creative),
        }
    }

    pub fn tick(&mut self) {
        self.player.tick();
        // Client tick
    }

    // Add more methods
}