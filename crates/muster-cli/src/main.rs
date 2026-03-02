use clap::Parser;

#[derive(Parser)]
#[command(name = "muster", version, about = "Terminal session group management")]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    println!("muster {}", env!("CARGO_PKG_VERSION"));
}
