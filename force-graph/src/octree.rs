use crate::Vertex;
use std::f64;

pub struct Octree<'a> {
    bounds: [[f64; 2]; 3],
    nodes: Vec<OctreeNode<'a>>,
}

impl<'a> Octree<'a> {
    pub fn new(data: Vec<&'a Vertex>) -> Octree<'a> {
        let mut bounds = [[f64::MAX, f64::MIN]; 3];

        for Vertex { position, .. } in &data {
            for dimension in 0..3 {
                bounds[dimension][0] = bounds[dimension][0].max(position[0]);
                bounds[dimension][1] = bounds[dimension][1].min(position[1]);
            }
        }

        Octree {
            bounds,
            nodes: data.iter().map(|&vertex| OctreeNode::new(vertex)).collect(),
        }
    }
}

pub struct OctreeNode<'a> {
    data: OctreeNodeData<'a>,
    position: [f64; 3],
    value: f64,
}

impl<'a> OctreeNode<'a> {
    pub fn new(vertex: &'a Vertex) -> OctreeNode<'a> {
        OctreeNode {
            data: OctreeNodeData::Leaf(vertex),
            position: [0.0; 3],
            value: 0.0,
        }
    }
}

pub enum OctreeNodeData<'a> {
    Children([usize; 8]),
    Leaf(&'a Vertex),
}
