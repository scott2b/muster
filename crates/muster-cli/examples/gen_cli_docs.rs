//! Generate CLI reference documentation from clap type definitions.
//!
//! Usage: `cargo run --example gen_cli_docs -p muster-cli > docs/src/cli-reference.md`

fn main() {
    let markdown = clap_markdown::help_markdown::<muster_cli::Cli>();
    print!("{markdown}");
}
