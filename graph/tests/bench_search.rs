#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use futures::executor;
use test::Bencher;
use vault_graph::{Graph, Search};

lazy_static! {
    static ref GRAPH: Graph = executor::block_on(Graph::test());
}

#[bench]
fn bench_search_crates_a(b: &mut Bencher) {
    b.iter(|| GRAPH.crates().search("a"));
}

#[bench]
fn bench_search_crates_act(b: &mut Bencher) {
    b.iter(|| GRAPH.crates().search("act"));
}

#[bench]
fn bench_search_crates_actix_web(b: &mut Bencher) {
    b.iter(|| GRAPH.crates().search("actix"));
}

#[bench]
fn bench_search_crates_random(b: &mut Bencher) {
    b.iter(|| GRAPH.crates().search("aweiufawoe"));
}

#[bench]
fn bench_search_categories_a(b: &mut Bencher) {
    b.iter(|| GRAPH.categories().search("a"));
}

#[bench]
fn bench_search_categories_web(b: &mut Bencher) {
    b.iter(|| GRAPH.categories().search("web"));
}

#[bench]
fn bench_search_categories_async(b: &mut Bencher) {
    b.iter(|| GRAPH.categories().search("async"));
}

#[bench]
fn bench_search_categories_random(b: &mut Bencher) {
    b.iter(|| GRAPH.categories().search("aweiufawoe"));
}

#[bench]
fn bench_search_keywords_a(b: &mut Bencher) {
    b.iter(|| GRAPH.keywords().search("a"));
}

#[bench]
fn bench_search_keywords_web(b: &mut Bencher) {
    b.iter(|| GRAPH.keywords().search("web"));
}

#[bench]
fn bench_search_keywords_async(b: &mut Bencher) {
    b.iter(|| GRAPH.keywords().search("async"));
}

#[bench]
fn bench_search_keywords_random(b: &mut Bencher) {
    b.iter(|| GRAPH.keywords().search("aweiufawoe"));
}
