use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;
use console::rom_header;
use console::flag_6;


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
 *
 * --------------------------------------------------------------------------------
 *
 * INES ROM Format
 *
 * Header (16 bytes)
 * Trainer, if present (0 or 512 bytes)
 * PRG ROM data (16384 * x bytes)
 * CHR ROM data, if present (8192 * y bytes)
 *
*/

fn read_rom(filename: &String) -> Result<Vec<u8>, io::Error> {
    println!("Reading NES ROM File {}", filename);
    let mut rom_file = try!(File::open(filename));
    let mut buffer: Vec<u8> = Vec::new();

    // read the whole file
    try!(rom_file.read_to_end(&mut buffer));
    return Ok(buffer);
}

pub fn load_content(filename: &String) -> CartridgeData {
    match read_rom(filename) {
        Ok(data) => {
            let raw_data = data.clone();
            let rom_data = prepare(&raw_data);
            return rom_data;
        },
        Err(e) => {
            panic!("Failed to read ROM data :: ERROR = {}", e);
        }
    }
}

pub struct CartridgeData {
    header: rom_header::RomHeader,
    prg_bank: Vec<u8>,
    chr_bank: Vec<u8>
}

pub fn prepare<'a>(rom_data: &Vec<u8>) -> CartridgeData {
    let headers: rom_header::RomHeader = rom_header::read_rom_header(rom_data);

    let prg_start_pos = 16 as usize;
    let prg_end_pos = (16 + headers.prg_rom_size) as usize;
    let chr_start_pos = (prg_end_pos) as usize;
    let chr_end_pos = (16 + headers.prg_rom_size + headers.chr_rom_size) as usize;
    println!("start :: {} end :: {}", prg_start_pos, prg_end_pos);

    let prg_data: &[u8] = &rom_data[prg_start_pos..prg_end_pos];
    let chr_data: &[u8] = &rom_data[chr_start_pos..chr_end_pos];

    
    // Make sure the length fit
    assert_eq!(prg_data.len(), headers.prg_rom_size as usize);
    assert_eq!(chr_data.len(), headers.chr_rom_size as usize);
    println!("{:?}", headers);

    return CartridgeData { header: headers, prg_bank: Vec::from(prg_data), chr_bank: Vec::from(chr_data) };
   // println!("{:?}", prg_data);
}
