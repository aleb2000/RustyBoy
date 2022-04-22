mod opcode;
mod cartridge;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: String
}

fn main() {
    let _args = Args::parse();

    println!("This is not yet an emulator.");
}
