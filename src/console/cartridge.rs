use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;
use console::rom_data;

/*
 * HEADERS of the NES Cadridge
 *
 * 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
 * 4: Size of PRG ROM in 16 KB units
 * 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
 * 6: Flags 6
 * 7: Flags 7
 * 8: Size of PRG RAM in 8 KB units (Value 0 infers 8 KB for compatibility; see PRG RAM circuit)
 * 9: Flags 9
 * 10: Flags 10 (unofficial)
 * 11-15: ZEROS
*/

fn read_rom(filename: &String) -> Result<Vec<u8>, io::Error> {
    println!("Reading NES ROM File {}", filename);
    let mut rom_file = try!(File::open(filename));
    let mut buffer: Vec<u8> = Vec::new();

    // read the whole file
    try!(rom_file.read_to_end(&mut buffer));
    return Ok(buffer);
}

pub fn load_content(filename: &String) {
    match read_rom(&filename) {
        Ok(data) => rom_data::prepare(&data),
        Err(e) => panic!("Failed to read ROM data :: ERROR = {}", e)
    }
}
