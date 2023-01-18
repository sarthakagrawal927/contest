use clap::Parser;
use rust::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
