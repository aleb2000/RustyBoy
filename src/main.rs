mod opcode;
mod cartridge;

use clap::Parser;
use cartridge::Cartridge;

use crate::cartridge::{licensee_code_name, CartridgeType};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: String
}

fn main() {
    let args = Args::parse();

    println!("File read correctly!");
}
