use cargo_platform::Cfg;
use rustc_ap_rustc_target::spec::{self, Target, TargetTriple};
use std::collections::HashMap;

pub fn get_targets() -> HashMap<String, Vec<Cfg>> {
    spec::get_targets()
        .map(|target_triple| {
            let target =
                Target::search(&TargetTriple::from_triple(target_triple.as_str())).unwrap();
            (
                target_triple,
                vec![
                    Cfg::KeyPair(String::from("target_arch"), target.arch),
                    Cfg::KeyPair(String::from("target_endian"), target.target_endian),
                    Cfg::KeyPair(String::from("target_env"), target.target_env),
                    Cfg::KeyPair(String::from("target_os"), target.target_os),
                    Cfg::KeyPair(
                        String::from("target_pointer_width"),
                        target.target_pointer_width,
                    ),
                    Cfg::KeyPair(String::from("target_vendor"), target.target_vendor),
                ],
            )
        })
        .collect()
}
