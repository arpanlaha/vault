#![feature(test)]
extern crate test;

use futures::executor::block_on;
use test::Bencher;
use vault_graph::Graph;

#[bench]
fn bench_graph_actix_web(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.get_dependency_graph("actix-web", vec![]));
}

#[bench]
fn bench_graph_serde(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.get_dependency_graph("serde", vec![]));
}

#[bench]
fn bench_graph_tokio(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.get_dependency_graph("tokio", vec![]));
}
