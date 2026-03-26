#[derive(Clone, Debug)]
pub struct Tier {
    pub level: i32,
    pub uses: i32,
    pub speed: f32,
    pub damage: i32,
}

impl Tier {
    pub fn new(level: i32, uses: i32, speed: f32, damage: i32) -> Self {
        Self {
            level,
            uses,
            speed,
            damage,
        }
    }
}

// Static tiers
lazy_static::lazy_static! {
    pub static ref WOOD: Tier = Tier::new(0, 59, 2.0, 0);
    pub static ref STONE: Tier = Tier::new(1, 131, 4.0, 1);
    pub static ref IRON: Tier = Tier::new(2, 250, 6.0, 2);
    pub static ref EMERALD: Tier = Tier::new(3, 1561, 8.0, 3);
    pub static ref GOLD: Tier = Tier::new(0, 32, 12.0, 0);
}

pub const MAX_STACK_SIZE: i32 = 64;
pub const MAX_ITEMS: usize = 512;

#[derive(Clone, Debug)]
pub struct Item {
    pub id: i32,
    pub max_stack_size: i32,
    pub icon: i32,
    pub name: String,
}

impl Item {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            max_stack_size: MAX_STACK_SIZE,
            icon: 0,
            name: name.to_string(),
        }
    }

    // Add methods
}

// Static items array
lazy_static::lazy_static! {
    pub static ref ITEMS: [Option<Item>; MAX_ITEMS] = {
        let mut arr: [Option<Item>; MAX_ITEMS] = [const { None }; MAX_ITEMS];
        // Initialize some items
        arr[256] = Some(Item::new(256, "iron_shovel"));
        arr[257] = Some(Item::new(257, "iron_pickaxe"));
        // Add more
        arr
    };

    pub static ref SHOVEL_IRON: Item = Item::new(256, "iron_shovel");
    pub static ref PICKAXE_IRON: Item = Item::new(257, "iron_pickaxe");
    // Add more
}

pub fn init_items() {
    // Initialize items
}