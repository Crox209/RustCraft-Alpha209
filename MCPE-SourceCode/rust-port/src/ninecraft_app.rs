use crate::app::{App, AppContext};
use crate::app_platform::AppPlatform;
use crate::minecraft::Minecraft;

pub struct NinecraftApp {
    minecraft: Minecraft,
    verbose: bool,
    frames: i32,
    last_tick_ms: i64,
}

impl NinecraftApp {
    pub fn new() -> Self {
        Self {
            minecraft: Minecraft::new(),
            verbose: true,
            frames: 0,
            last_tick_ms: 0,
        }
    }

    pub fn minecraft_mut(&mut self) -> &mut Minecraft {
        &mut self.minecraft
    }

    fn init_gl_states(&mut self) {
        // TODO: OpenGL initialization
    }

    fn update_stats(&mut self) {
        // TODO: Update stats
    }

    fn restart_server(&mut self) {
        // TODO: Restart server
    }

    fn test_creation_and_destruction(&mut self) {
        // TODO: Tests
    }

    fn test_joining_and_destruction(&mut self) {
        // TODO: Tests
    }
}

impl App for NinecraftApp {
    fn init(&mut self) {
        // Call Minecraft init
        self.minecraft.init();
        // Additional init
        crate::material::init_materials();
        crate::tile::init_tiles();
        crate::item::init_items();
        self.init_gl_states();
        // TODO: Load language, etc.
    }

    fn is_inited(&self) -> bool { self.minecraft.is_inited() }

    fn platform(&self) -> Option<&dyn AppPlatform> {
        self.minecraft.platform()
    }

    fn on_graphics_reset(&mut self) {
        self.minecraft.on_graphics_reset();
        self.init_gl_states();
    }

    fn update(&mut self) {
        self.minecraft.update();
        self.update_stats();
    }

    fn quit(&mut self) { self.minecraft.quit(); }

    fn want_to_quit(&self) -> bool { self.minecraft.want_to_quit() }

    fn handle_back(&mut self, is_down: bool) -> bool {
        self.minecraft.handle_back(is_down)
    }
}