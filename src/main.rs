use clap::Parser;

fn main() {
    Opts::parse();
}

#[derive(Parser)]
#[clap(version)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /**
    First line docs for subcommand foo, always shown.

    Extra docs for subcommand foo, only shown in long help.

    `clap_repro foo --help` -> long help (as expected)
    `clap_repro foo -h` -> short help (as expected)
    `clap_repro help foo` -> short help (unexpected)

    Examples:

    ❯ clap_repro foo # Does nothing.
    */
    Foo,
}
