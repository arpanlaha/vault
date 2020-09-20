#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use test::Bencher;
use vault_graph::{Graph, Random};

lazy_static! {
    static ref GRAPH: Graph = Graph::test();
}

#[bench]
fn bench_random_crates(b: &mut Bencher) {
    b.iter(|| GRAPH.crates().random());
}

#[bench]
fn bench_random_categories(b: &mut Bencher) {
    b.iter(|| GRAPH.categories().random());
}

#[bench]
fn bench_random_keywords(b: &mut Bencher) {
    b.iter(|| GRAPH.keywords().random());
}
