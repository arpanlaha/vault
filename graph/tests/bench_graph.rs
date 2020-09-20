#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use futures::executor;
use test::Bencher;
use vault_graph::Graph;

lazy_static! {
    static ref GRAPH: Graph = Graph::test();
}

#[bench]
fn bench_graph_actix_web(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "actix-web",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_rocket(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "rocket",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_warp(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "warp",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_hyper(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "hyper",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_serde(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "serde",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_tokio(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "tokio",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_futures(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "futures",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_wasm_bindgen(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "wasm-bindgen",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_ripgrep(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "ripgrep",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_clippy(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "clippy",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_rustfmt(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "rustfmt",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_cargo(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "cargo",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_crossbeam(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "crossbeam",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_parking_lot(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "parking_lot",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_socket2(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "socket2",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_rayon(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "rayon",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_diesel(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "diesel",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_sqlx(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "sqlx",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}

#[bench]
fn bench_graph_tokei(b: &mut Bencher) {
    b.iter(|| {
        GRAPH.get_dependency_graph(
            "tokei",
            vec![],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &Some(String::from("unix")),
        )
    });
}
