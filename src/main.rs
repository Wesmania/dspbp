use clap::StructOpt;

mod args;

fn main() {
    let cli = args::Args::parse();
    println!("{:?}", cli);
}
