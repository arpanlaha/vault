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
    let crate_names = GRAPH.crate_names();
    let crates = GRAPH.crates();
    b.iter(|| crate_names.search("a", crates));
}

#[bench]
fn bench_search_crates_act(b: &mut Bencher) {
    let crate_names = GRAPH.crate_names();
    let crates = GRAPH.crates();
    b.iter(|| crate_names.search("act", crates));
}

#[bench]
fn bench_search_crates_actix(b: &mut Bencher) {
    let crate_names = GRAPH.crate_names();
    let crates = GRAPH.crates();
    b.iter(|| crate_names.search("actix", crates));
}

#[bench]
fn bench_search_crates_random(b: &mut Bencher) {
    let crate_names = GRAPH.crate_names();
    let crates = GRAPH.crates();
    b.iter(|| crate_names.search("aweiufawoe", crates));
}

#[bench]
fn bench_search_categories_a(b: &mut Bencher) {
    let category_names = GRAPH.category_names();
    let categories = GRAPH.categories();
    b.iter(|| category_names.search("A", categories));
}

#[bench]
fn bench_search_categories_web(b: &mut Bencher) {
    let category_names = GRAPH.category_names();
    let categories = GRAPH.categories();
    b.iter(|| category_names.search("Web", categories));
}

#[bench]
fn bench_search_categories_async(b: &mut Bencher) {
    let category_names = GRAPH.category_names();
    let categories = GRAPH.categories();
    b.iter(|| category_names.search("Async", categories));
}

#[bench]
fn bench_search_categories_random(b: &mut Bencher) {
    let category_names = GRAPH.category_names();
    let categories = GRAPH.categories();
    b.iter(|| category_names.search("aweiufawoe", categories));
}

#[bench]
fn bench_search_keywords_a(b: &mut Bencher) {
    let keyword_names = GRAPH.keyword_names();
    let keywords = GRAPH.keywords();
    b.iter(|| keyword_names.search("a", keywords));
}

#[bench]
fn bench_search_keywords_web(b: &mut Bencher) {
    let keyword_names = GRAPH.keyword_names();
    let keywords = GRAPH.keywords();
    b.iter(|| keyword_names.search("web", keywords));
}

#[bench]
fn bench_search_keywords_async(b: &mut Bencher) {
    let keyword_names = GRAPH.keyword_names();
    let keywords = GRAPH.keywords();
    b.iter(|| keyword_names.search("async", keywords));
}

#[bench]
fn bench_search_keywords_random(b: &mut Bencher) {
    let keyword_names = GRAPH.keyword_names();
    let keywords = GRAPH.keywords();
    b.iter(|| keyword_names.search("aweiufawoe", keywords));
}
