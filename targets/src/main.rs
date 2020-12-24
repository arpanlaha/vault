use cargo_platform::Cfg;
use std::{env, fs::File, io::Write};

fn main() {
    let targets = vault_targets::get_targets();

    let mut file = File::create(env::args().nth(1).expect("Output file not specified.")).unwrap();

    writeln!(&mut file, "triple;cfgs").unwrap();

    for (triple, cfgs) in &targets {
        writeln!(
            &mut file,
            "{};{:?}",
            triple,
            cfgs.iter()
                .map(|cfg| match cfg {
                    Cfg::KeyPair(first, second) => vec![first, second],
                    Cfg::Name(first) => vec![first],
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
