use crate::hit_result::HitResult;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            x1: 1.0,
            y1: 1.0,
            z1: 1.0,
        }
    }
}

impl Aabb {
    pub fn new(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        Self { x0, y0, z0, x1, y1, z1 }
    }

    pub fn intersects(self, c: Self) -> bool {
        !(c.x1 <= self.x0
            || c.x0 >= self.x1
            || c.y1 <= self.y0
            || c.y0 >= self.y1
            || c.z1 <= self.z0
            || c.z0 >= self.z1)
    }

    pub fn clip_x_collide(self, c: Self, mut xa: f32) -> f32 {
        if c.y1 <= self.y0 || c.y0 >= self.y1 || c.z1 <= self.z0 || c.z0 >= self.z1 {
            return xa;
        }
        if xa > 0.0 && c.x1 <= self.x0 {
            let max = self.x0 - c.x1;
            if max < xa {
                xa = max;
            }
        }
        if xa < 0.0 && c.x0 >= self.x1 {
            let max = self.x1 - c.x0;
            if max > xa {
                xa = max;
            }
        }
        xa
    }

    pub fn clip(self, a: Vec3, b: Vec3) -> HitResult {
        let mut candidates: Vec<(Vec3, i32)> = Vec::new();

        if let Some(v) = a.clip_x(b, self.x0).filter(|v| self.contains_x(*v)) {
            candidates.push((v, 4));
        }
        if let Some(v) = a.clip_x(b, self.x1).filter(|v| self.contains_x(*v)) {
            candidates.push((v, 5));
        }
        if let Some(v) = a.clip_y(b, self.y0).filter(|v| self.contains_y(*v)) {
            candidates.push((v, 0));
        }
        if let Some(v) = a.clip_y(b, self.y1).filter(|v| self.contains_y(*v)) {
            candidates.push((v, 1));
        }
        if let Some(v) = a.clip_z(b, self.z0).filter(|v| self.contains_z(*v)) {
            candidates.push((v, 2));
        }
        if let Some(v) = a.clip_z(b, self.z1).filter(|v| self.contains_z(*v)) {
            candidates.push((v, 3));
        }

        if candidates.is_empty() {
            return HitResult::default();
        }

        let (pos, face) = candidates
            .into_iter()
            .min_by(|(p1, _), (p2, _)| {
                a.distance_to_sqr(*p1)
                    .partial_cmp(&a.distance_to_sqr(*p2))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("non-empty");

        HitResult::tile(0, 0, 0, face, pos)
    }

    fn contains_x(self, v: Vec3) -> bool {
        v.y >= self.y0 && v.y <= self.y1 && v.z >= self.z0 && v.z <= self.z1
    }
    fn contains_y(self, v: Vec3) -> bool {
        v.x >= self.x0 && v.x <= self.x1 && v.z >= self.z0 && v.z <= self.z1
    }
    fn contains_z(self, v: Vec3) -> bool {
        v.x >= self.x0 && v.x <= self.x1 && v.y >= self.y0 && v.y <= self.y1
    }
}

