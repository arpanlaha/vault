#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use std::cell::RefCell;
use wasm_bindgen::prelude::*;

const ALPHA_DECAY: f64 = 0.977_237_220_955_810_7;
const ALPHA_MIN: f64 = 0.001;
const OPTIMAL_LENGTH: f64 = 30.0;
const VELOCITY_DECAY: f64 = 0.6;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Vertex {
    index: usize,
    position: [f64; 3],
    velocity: [f64; 3],
    edge_count: usize,
}

impl Vertex {
    pub const fn new(index: usize, edge_count: usize) -> Self {
        Self {
            index,
            position: [0_f64, 0_f64, 0_f64],
            velocity: [0_f64, 0_f64, 0_f64],
            edge_count,
        }
    }
}

struct Edge {
    bias: f64,
    from: usize,
    strength: f64,
    to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize, edge_counts: &[usize]) -> Self {
        let source_edge_count = edge_counts[from] as f64;
        let dest_edge_count = edge_counts[to] as f64;

        Self {
            bias: source_edge_count / (source_edge_count + dest_edge_count),
            from,
            strength: 1_f64 / source_edge_count.min(dest_edge_count),
            to,
        }
    }

    // pub const fn from(&self) -> usize {
    //     self.from
    // }

    // pub const fn strength(&self) -> f64 {
    //     self.strength
    // }

    // pub const fn to(&self) -> usize {
    //     self.to
    // }

    // pub fn set_strength(&mut self, strength: f64) {
    //     self.strength = strength;
    // }
}

#[wasm_bindgen]
pub struct ForceLayout {
    num_vertices: usize,
    vertices: RefCell<Vec<Vertex>>,
    edges: Vec<Edge>,
    alpha: f64,
}

#[wasm_bindgen]
impl ForceLayout {
    #[must_use]
    pub fn new(num_vertices: u32, edge_tuples: &[u32]) -> Self {
        let num_vertices = num_vertices as usize;

        assert_eq!(
            edge_tuples.len() % 2,
            0,
            "Edge tuple list must be even in length."
        );

        let mut edges: Vec<Edge> = Vec::with_capacity(edge_tuples.len() / 2);

        let mut edge_counts = vec![0_usize; num_vertices];

        for edge_tuple in edge_tuples.chunks_exact(2) {
            edge_counts[edge_tuple.get(0).unwrap().to_owned() as usize] += 1;
            edge_counts[edge_tuple.get(1).unwrap().to_owned() as usize] += 1;
        }

        for edge_tuple in edge_tuples.chunks_exact(2) {
            edges.push(Edge::new(
                edge_tuple.get(0).unwrap().to_owned() as usize,
                edge_tuple.get(1).unwrap().to_owned() as usize,
                &edge_counts,
            ));
        }

        Self {
            num_vertices,
            vertices: RefCell::new(
                (0..num_vertices)
                    .map(|index| Vertex::new(index, edge_counts[index]))
                    .collect(),
            ),
            edges,
            alpha: 1_f64,
        }
    }

    pub fn tick(&mut self, iterations: u32) -> bool {
        for _ in 0..iterations {
            self.tick_single();
        }

        self.alpha <= ALPHA_MIN
    }
}

impl ForceLayout {
    fn tick_single(&mut self) {
        self.alpha *= ALPHA_DECAY;

        for edge in &self.edges {
            self.apply_edge_force(edge);
            // let length = get_length(&source.position, &dest.position);
        }

        let mut position_sums = vec![0_f64; 3];

        for Vertex { position, .. } in self.vertices.borrow().iter() {
            for dimension in 0..3 {
                position_sums[dimension] += position[dimension];
            }
        }

        let position_means: Vec<f64> = position_sums
            .iter()
            .map(|position_sum| position_sum / self.num_vertices as f64)
            .collect();

        for Vertex {
            position, velocity, ..
        } in self.vertices.borrow_mut().iter_mut()
        {
            for dimension in 0..3 {
                velocity[dimension] *= VELOCITY_DECAY;
                position[dimension] += velocity[dimension] - position_means[dimension];
            }
        }
    }

    fn apply_edge_force(&self, edge: &Edge) {
        let &Edge {
            bias,
            from,
            strength,
            to,
        } = edge;

        let (x, y, z) = {
            let vertices = self.vertices.borrow();

            let source = &vertices[from];
            let dest = &vertices[to];

            (
                (source.position[0] + source.velocity[0]) - (dest.position[0] + dest.velocity[0]),
                (source.position[1] + source.velocity[1]) - (dest.position[1] + dest.velocity[1]),
                (source.position[2] + source.velocity[2]) - (dest.position[2] + dest.velocity[2]),
            )
        };

        let length = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

        let length_factor = ((length - OPTIMAL_LENGTH) / length) * self.alpha * strength;

        let x_factor = x * length_factor;
        let y_factor = y * length_factor;
        let z_factor = z * length_factor;

        let mut vertices = self.vertices.borrow_mut();

        let dest = &mut vertices[to];
        dest.velocity[0] += x_factor * bias;
        dest.velocity[1] += y_factor * bias;
        dest.velocity[2] += z_factor * bias;

        let source = &mut vertices[from];
        let source_bias = 1_f64 - bias;
        source.velocity[0] += x_factor * source_bias;
        source.velocity[1] += y_factor * source_bias;
        source.velocity[2] += z_factor * source_bias;
    }
}

fn get_length(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}
