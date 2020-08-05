#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use wasm_bindgen::prelude::*;

const ALPHA_DECAY: f64 = 0.98;
const VELOCITY_DECAY: f64 = 0.6;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Vertex {
    id: u32,
    position: [f64; 3],
    velocity: [f64; 3],
}

impl Vertex {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            position: [0_f64, 0_f64, 0_f64],
            velocity: [0_f64, 0_f64, 0_f64],
        }
    }
}

struct Edge {
    from: u32,
    to: u32,
}

impl Edge {
    pub const fn new(from: u32, to: u32) -> Self {
        Self { from, to }
    }
}

#[wasm_bindgen]
pub struct ForceGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    alpha: f64,
}

#[wasm_bindgen]
impl ForceGraph {
    #[must_use]
    pub fn new(vertex_ids: &[u32], edge_tuples: &[u32]) -> Self {
        assert!(
            edge_tuples.len() == vertex_ids.len() * 2,
            "Edge tuple list must be twice the length of vertex id list."
        );

        let mut edges: Vec<Edge> = Vec::with_capacity(vertex_ids.len());

        for edge_tuple in edge_tuples.chunks_exact(2) {
            edges.push(Edge::new(
                edge_tuple.get(0).unwrap().to_owned(),
                edge_tuple.get(1).unwrap().to_owned(),
            ));
        }

        Self {
            vertices: vertex_ids
                .iter()
                .map(|&vertex_id| Vertex::new(vertex_id))
                .collect(),
            edges,
            alpha: 1_f64,
        }
    }

    pub fn tick(&mut self, iterations: u32) {
        for _ in 0..iterations {
            self.tick_single();
        }
    }
}

impl ForceGraph {
    fn tick_single(&mut self) {
        self.alpha *= ALPHA_DECAY;

        // apply forces

        for vertex in &mut self.vertices {
            let Vertex {
                position, velocity, ..
            } = vertex;

            for dimension in 0..3 {
                velocity[dimension] *= VELOCITY_DECAY;
                position[dimension] += velocity[dimension];
            }
        }
    }
}
