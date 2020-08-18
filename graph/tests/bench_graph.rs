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
    b.iter(|| GRAPH.get_dependency_graph("actix-web", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_rocket(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("rocket", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_warp(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("warp", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_hyper(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("hyper", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_serde(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("serde", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_tokio(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("tokio", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_futures(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("futures", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_wasm_bindgen(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("wasm-bindgen", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_ripgrep(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("ripgrep", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_clippy(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("clippy", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_rustfmt(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("rustfmt", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_cargo(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("cargo", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_crossbeam(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("crossbeam", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_parking_lot(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("parking_lot", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_socket2(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("socket2", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_rayon(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("rayon", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_diesel(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("diesel", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_sqlx(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("sqlx", vec![], "x86_64-unknown-linux-gnu"));
}

#[bench]
fn bench_graph_tokei(b: &mut Bencher) {
    b.iter(|| GRAPH.get_dependency_graph("tokei", vec![], "x86_64-unknown-linux-gnu"));
}
