use anyhow::Result;
use clap::Parser;
use rust::{config::Config, opts::Opts};

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);
    return Ok(());
}
