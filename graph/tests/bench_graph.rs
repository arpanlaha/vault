#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use futures::executor;
use test::Bencher;
use vault_graph::Graph;

lazy_static! {
    static ref GRAPH: Graph = executor::block_on(Graph::test());
}

#[bench]
fn bench_graph_actix_web(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("actix-web", vec![]));
}

#[bench]
fn bench_graph_serde(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("serde", vec![]));
}

#[bench]
fn bench_graph_tokio(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("tokio", vec![]));
}
