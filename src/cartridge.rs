use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;

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

const EXPECTED_NES_HEADER: [u8; 3] = [78,69,83]; // NES String begining of cartridge

enum HeaderPosition {
    PrgRomSize,
    ChrRomSize,
    Flag6,
    Flag7,
    PrgRamSize,
    Flag9,
    Flag10
}

impl HeaderPosition {
    fn position(&self) -> usize {
        match *self {
            HeaderPosition::PrgRomSize => 4,
            HeaderPosition::ChrRomSize => 5,
            HeaderPosition::Flag6 => 6,
            HeaderPosition::Flag7 => 7,
            HeaderPosition::PrgRamSize => 8,
            HeaderPosition::Flag9 => 9,
            HeaderPosition::Flag10 => 10
        }
    }
}

#[derive(Debug)]
struct RomHeader<'a> {
    authenticity_header: &'a [u8],
    prg_rom_size: u8,
    chr_rom_size: u8,
    flag_6: u8,
    flag_7: u8,
    prg_ram_size: u8,
    flag_9: u8,
    flag_10: u8
}

pub fn prepare_rom_data(filename: &String) {
    match read_rom(&filename) {
        Ok(rom_data) => {
            let headers: RomHeader = read_rom_header(&rom_data);
            println!("{:?}", headers);
        },
        Err(e) => panic!("Failed to read ROM data :: ERROR = {}", e)
    }
}

fn read_rom_header(rom_data: &Vec<u8>) -> RomHeader {
    let base_headers: &[u8] = &rom_data[0..15];
    let nes_header: &[u8] = validate_nes_header(&base_headers);
    let prg_rom_size: u8 = rom_data[HeaderPosition::PrgRomSize.position()];
    return RomHeader {
        authenticity_header: nes_header,
        prg_rom_size: rom_data[HeaderPosition::PrgRomSize.position()],
        chr_rom_size: rom_data[HeaderPosition::ChrRomSize.position()],
        flag_6: rom_data[HeaderPosition::Flag6.position()],
        flag_7: rom_data[HeaderPosition::Flag7.position()],
        prg_ram_size: rom_data[HeaderPosition::PrgRamSize.position()],
        flag_9: rom_data[HeaderPosition::Flag9.position()],
        flag_10: rom_data[HeaderPosition::Flag10.position()]
    };
}

fn validate_nes_header(rom_data: &[u8]) -> &[u8] {
    // Make sure the header is valid...
    let nes_header: &[u8] = &rom_data[0..3];
    assert_eq!(nes_header, EXPECTED_NES_HEADER);
    match str::from_utf8(&nes_header) {
        Ok(v) => println!("ROM NES HEADER = {}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
    return &nes_header[0..3];
}

pub fn read_rom(filename: &String) -> Result<Vec<u8>, io::Error> {
    println!("Reading NES ROM File {}", filename);
    let mut rom_file = try!(File::open(filename));
    let mut buffer: Vec<u8> = Vec::new();

    // read the whole file
    try!(rom_file.read_to_end(&mut buffer));
    return Ok(buffer);
}
