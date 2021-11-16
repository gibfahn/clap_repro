use clap::Parser;
use structopt::StructOpt;

// By default in clap the version in the Cargo.toml is not picked up.
// In structopt it is.
//
// # In Clap
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
// If you uncomment the #[clap(version)] line below then the version is picked up.

fn main() {
    // Uses clap, doesn't set the version.
    Opts::parse();

    // Uses StructOpt, does set the version.
    // Opts::from_args();
}

#[derive(StructOpt, Parser)]
// If you set this manually, then clap will have the same behaviour as structopt.
// #[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {}
