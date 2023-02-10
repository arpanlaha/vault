#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc)]

use cargo_platform::Cfg;
use rustc_ap_rustc_target::spec::{Target, TargetTriple, TARGETS};
use std::collections::BTreeMap;
use std::path::Path;

/// Returns a mapping of supported targets to a list of cfg attributes.
#[must_use]
pub fn get_targets() -> BTreeMap<String, Vec<Cfg>> {
    TARGETS
        .iter()
        .map(|&target_triple| {
            let target = Target::search(
                &TargetTriple::from_triple(target_triple),
                &Path::new("/").to_path_buf(),
            )
            .unwrap();
            (
                String::from(target_triple),
                vec![
                    Cfg::KeyPair(String::from("target_arch"), target.arch),
                    Cfg::KeyPair(
                        String::from("target_endian"),
                        target.options.endian.as_str().into(),
                    ),
                    Cfg::KeyPair(String::from("target_env"), target.options.env),
                    Cfg::KeyPair(String::from("target_os"), target.options.os),
                    Cfg::KeyPair(
                        String::from("target_pointer_width"),
                        target.pointer_width.to_string(),
                    ),
                    Cfg::KeyPair(String::from("target_vendor"), target.options.vendor),
                ],
            )
        })
        .collect()
}
