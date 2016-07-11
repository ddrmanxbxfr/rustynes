mod console;
mod utils;

use console::cartridge;
use std::env;

fn main() {
    let filename: Option<String> = env::args().skip(1).next();
    match filename {
        Some(value) => cartridge::load_content(&value),
        None => print_usage()
    }
}

fn print_usage() {
    println!("Usage: rustynes ROM_FILE [options]");
    println!("Please use roms that you legally own!");
}
