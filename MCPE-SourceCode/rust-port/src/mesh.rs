use crate::level::Level;
use crate::chunk_codec::{CHUNK_WIDTH, CHUNK_DEPTH, CHUNK_HEIGHT};

// Base cube vertex positions (36 vertices) grouped by faces (6 verts per face).
const BASE_CUBE_POS: [f32; 36 * 3] = [
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,

    -0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5,

    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,

    0.5,  0.5,  0.5,
    0.5,  0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,

    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,

    -0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
];

/// Generate a merged vertex list (position.xyz, color.rgb) for visible faces in a chunk-area.
pub fn generate_chunked_mesh_vertices(level: &Level, chunk_radius: i32) -> Vec<f32> {
    let mut out: Vec<f32> = Vec::new();
    for chunk_x in -chunk_radius..=chunk_radius {
        for chunk_z in -chunk_radius..=chunk_radius {
            let base_x = chunk_x * CHUNK_WIDTH as i32;
            let base_z = chunk_z * CHUNK_DEPTH as i32;

            for bx in 0..CHUNK_WIDTH {
                for bz in 0..CHUNK_DEPTH {
                    for by in 0..CHUNK_HEIGHT {
                        let gx = base_x + bx as i32;
                        let gy = by as i32;
                        let gz = base_z + bz as i32;
                        let tile = level.get_tile(gx, gy, gz);
                        if tile == crate::tile::AIR.id {
                            continue;
                        }

                        // Simple color mapping by tile
                        let color = if tile == crate::tile::GRASS.id {
                            [0.2f32, 0.9f32, 0.2f32]
                        } else if tile == crate::tile::STONE.id {
                            [0.6f32, 0.6f32, 0.6f32]
                        } else {
                            [0.8f32, 0.5f32, 0.3f32]
                        };

                        // neighbor checks (global coords)
                        let n_px = level.get_tile(gx + 1, gy, gz);
                        let n_nx = level.get_tile(gx - 1, gy, gz);
                        let n_pz = level.get_tile(gx, gy, gz + 1);
                        let n_nz = level.get_tile(gx, gy, gz - 1);
                        let n_py = level.get_tile(gx, gy + 1, gz);
                        let n_ny = level.get_tile(gx, gy - 1, gz);

                        let cx = gx as f32 + 0.5;
                        let cy = gy as f32 + 0.5;
                        let cz = gz as f32 + 0.5;

                        // faces: 0 = -Z, 1 = +Z, 2 = -X, 3 = +X, 4 = -Y, 5 = +Y
                        let neighbors = [n_nz, n_pz, n_nx, n_px, n_ny, n_py];

                        for face in 0..6 {
                            if neighbors[face] != crate::tile::AIR.id {
                                continue;
                            }

                            let face_offset = face * 6 * 3;
                            for v in 0..6 {
                                let vx = BASE_CUBE_POS[face_offset + v * 3 + 0] + cx;
                                let vy = BASE_CUBE_POS[face_offset + v * 3 + 1] + cy;
                                let vz = BASE_CUBE_POS[face_offset + v * 3 + 2] + cz;
                                out.push(vx);
                                out.push(vy);
                                out.push(vz);
                                out.push(color[0]);
                                out.push(color[1]);
                                out.push(color[2]);
                            }
                        }
                    }
                }
            }
        }
    }

    out
}
