mod console;
mod utils;

use console::cartridge;
use std::env;

fn main() {
//    let filename: Option<String> = env::args().skip(1).next();
    let rom_path: String = "/Users/ddrmanxbxfr/prog/emulator/rustynes/../fullset/Legend of Zelda, The (GC).nes".to_string();
    let filename: Option<String> = Some(rom_path);
    match filename {
        Some(value) => {
            let cartridge = cartridge::load_content(&value);
        },
        None => print_usage()
    }
}

fn print_usage() {
    println!("Usage: rustynes ROM_FILE [options]");
    println!("Please use roms that you legally own!");
}
