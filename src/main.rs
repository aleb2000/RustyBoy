mod opcode;
mod cartridge;

use clap::Parser;
use cartridge::Cartridge;

use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: String
}

fn main() -> Result<()> {
    let args = Args::parse();

    let cart = Cartridge::from_file(&args.file)
        .context("Cannot load cartridge, make sure the file exists and it is a valid Game Boy ROM")?;

    println!("Loaded cartridge!");

    println!("{}", cart);

    Ok(())
}
