#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

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
    // spec::get_targets()
    //     .map(|target_triple| {
    //         let target =
    //             Target::search(&TargetTriple::from_triple(target_triple.as_str())).unwrap();
    //         (
    //             target_triple,
    //             vec![
    //                 Cfg::KeyPair(String::from("target_arch"), target.arch),
    //                 Cfg::KeyPair(String::from("target_endian"), target.target_endian),
    //                 Cfg::KeyPair(String::from("target_env"), target.target_env),
    //                 Cfg::KeyPair(String::from("target_os"), target.target_os),
    //                 Cfg::KeyPair(
    //                     String::from("target_pointer_width"),
    //                     target.target_pointer_width,
    //                 ),
    //                 Cfg::KeyPair(String::from("target_vendor"), target.target_vendor),
    //             ],
    //         )
    //     })
    //     .collect()
}

// pub fn get_targets() {
//     // println!("{:?}", TARGETS);
//     for target in TARGETS {
//         println!(
//             "{:?}",
//             Target::search(
//                 &TargetTriple::from_triple(target),
//                 &Path::new("/").to_path_buf()
//             )
//             .is_err()
//         );
//     }
// }
