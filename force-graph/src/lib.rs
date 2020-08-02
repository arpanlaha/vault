#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions
)]

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Node {
    id: u32,
    coords: [u32; 3],
}

impl Node {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            coords: [0, 0, 0],
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
    vertices: Vec<Node>,
    edges: Vec<Edge>,
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
                .map(|&vertex_id| Node::new(vertex_id))
                .collect(),
            edges,
        }
    }

    pub fn tick(&mut self, iterations: u32) {
        for _ in 0..iterations {}
    }
}
