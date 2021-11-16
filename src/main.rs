use clap::Parser;

// By default the version in the cargo.toml is not picked up.
//
// Running `cargo run -- --help` will print `clap_repro` instead of `clap_repro 0.1.0`
//
// Running `cargo run -- --version` will error with:
//
// ```
// Running `target/debug/clap_repro --version`
// error: Found argument '--version' which wasn't expected, or isn't valid in this context
//
// 	If you tried to supply `--version` as a value rather than a flag, use `-- --version`
//
// USAGE:
//     clap_repro
//
// For more information try --help
// ```
//
// If you uncomment the version line below things start to work.

fn main() {
    Opts::parse();
}

#[derive(Parser)]
// #[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {}
