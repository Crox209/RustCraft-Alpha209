use std::cell::RefCell;
use std::sync::OnceLock;

use crate::random::Random;

pub const PI: f32 = std::f32::consts::PI;
pub const TWO_PI: f32 = 2.0 * PI;
pub const DEG_RAD: f32 = PI / 180.0;
pub const RAD_DEG: f32 = 180.0 / PI;

const SIN_TABLE_SIZE: usize = 65536;
const SIN_MASK: usize = SIN_TABLE_SIZE - 1;
const SIN_SCALE: f32 = SIN_TABLE_SIZE as f32 / TWO_PI;

static SIN_TABLE: OnceLock<Vec<f32>> = OnceLock::new();
thread_local! {
    static MTH_RANDOM: RefCell<Random> = RefCell::new(Random::new(0x1a2b_3c4d));
}

fn sin_table() -> &'static [f32] {
    SIN_TABLE.get_or_init(|| {
        (0..SIN_TABLE_SIZE)
            .map(|i| ((i as f32) / SIN_SCALE).sin())
            .collect()
    })
}

pub fn init_mth() {
    let _ = sin_table();
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn inv_sqrt(x: f32) -> f32 {
    // Preserve the original approximation behavior.
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 0x5f37_59df - (i >> 1);
    let mut y = f32::from_bits(i);
    y = y * (1.5 - xhalf * y * y);
    y
}

pub fn floor(v: f32) -> i32 {
    let i = v as i32;
    if v < i as f32 { i - 1 } else { i }
}

pub fn sin(x: f32) -> f32 {
    let idx = ((x * SIN_SCALE) as i32 as usize) & SIN_MASK;
    sin_table()[idx]
}

pub fn cos(x: f32) -> f32 {
    let idx = ((x * SIN_SCALE + (SIN_TABLE_SIZE / 4) as f32) as i32 as usize) & SIN_MASK;
    sin_table()[idx]
}

pub fn atan(x: f32) -> f32 {
    x.atan()
}

pub fn atan2(dy: f32, dx: f32) -> f32 {
    dy.atan2(dx)
}

pub fn random() -> f32 {
    // Keep behavior aligned with original Mth.cpp, which delegates to Random::nextFloat().
    MTH_RANDOM.with(|r| r.borrow_mut().next_float())
}

pub fn random_n(n: i32) -> i32 {
    MTH_RANDOM.with(|r| r.borrow_mut().next_int_n(n))
}

pub fn abs_f(a: f32) -> f32 {
    a.abs()
}

pub fn min_f(a: f32, b: f32) -> f32 {
    a.min(b)
}

pub fn max_f(a: f32, b: f32) -> f32 {
    a.max(b)
}

pub fn abs_i(a: i32) -> i32 {
    a.abs()
}

pub fn min_i(a: i32, b: i32) -> i32 {
    a.min(b)
}

pub fn max_i(a: i32, b: i32) -> i32 {
    a.max(b)
}

pub fn clamp_i(v: i32, low: i32, high: i32) -> i32 {
    v.clamp(low, high)
}

pub fn clamp_f(v: f32, low: f32, high: f32) -> f32 {
    v.clamp(low, high)
}

pub fn lerp_f(src: f32, dst: f32, alpha: f32) -> f32 {
    src + (dst - src) * alpha
}

pub fn lerp_i(src: i32, dst: i32, alpha: f32) -> i32 {
    src + ((dst - src) as f32 * alpha) as i32
}

pub fn abs_decrease(value: f32, with: f32, min: f32) -> f32 {
    if value > 0.0 {
        max_f(min, value - with)
    } else {
        min_f(value + with, -min)
    }
}

pub fn abs_max(a: f32, b: f32) -> f32 {
    a.abs().max(b.abs())
}

pub fn abs_max_signed(a: f32, b: f32) -> f32 {
    if a.abs() > b.abs() { a } else { b }
}

pub fn int_floor_div(a: i32, b: i32) -> i32 {
    if a < 0 {
        -((-a - 1) / b) - 1
    } else {
        a / b
    }
}
