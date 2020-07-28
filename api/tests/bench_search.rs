#![feature(test)]
extern crate test;

mod common;

use futures::executor::block_on;
use test::Bencher;
use vault_api::utils::{common::Search, state::Graph};

#[bench]
fn bench_search_crates_a(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.crates().search("a"));
}

#[bench]
fn bench_search_crates_act(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.crates().search("act"));
}

#[bench]
fn bench_search_crates_actix_web(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.crates().search("actix"));
}

#[bench]
fn bench_search_crates_random(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.crates().search("aweiufawoe"));
}

#[bench]
fn bench_search_categories_a(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.categories().search("a"));
}

#[bench]
fn bench_search_categories_web(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.categories().search("web"));
}

#[bench]
fn bench_search_categories_async(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.categories().search("async"));
}

#[bench]
fn bench_search_categories_random(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.categories().search("aweiufawoe"));
}

#[bench]
fn bench_search_keywords_a(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.keywords().search("a"));
}

#[bench]
fn bench_search_keywords_web(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.keywords().search("web"));
}

#[bench]
fn bench_search_keywords_async(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.keywords().search("async"));
}

#[bench]
fn bench_search_keywords_random(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.keywords().search("aweiufawoe"));
}
