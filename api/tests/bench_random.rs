#![feature(test)]
extern crate test;

mod common;

use futures::executor::block_on;
use test::Bencher;
use vault_api::utils::{common::Random, state::Graph};

#[bench]
fn bench_random_crates(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.crates().random());
}

#[bench]
fn bench_random_categories(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.categories().random());
}

#[bench]
fn bench_random_keywords(b: &mut Bencher) {
    let graph = block_on(Graph::test());
    b.iter(|| graph.keywords().random());
}
