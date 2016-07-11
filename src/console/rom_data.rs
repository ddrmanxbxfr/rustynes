use console::cartridge;
use console::flag_6;

const EXPECTED_NES_HEADER: [u8; 3] = [78,69,83]; // NES String begining of cartridge
const BASE_PRG_BANK_SIZE: u32 = 16384;
const BASE_CHR_BANK_SIZE: u32 = 8192;

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
pub struct RomHeader<'a> {
    authenticity_header: &'a [u8],
    prg_rom_size: u32,
    chr_rom_size: u32,
    flag_6: u8,
    flag_7: u8,
    prg_ram_size: u8,
    flag_9: u8,
    flag_10: u8
}

pub fn prepare(rom_data: &Vec<u8>) {
    let headers: RomHeader = read_rom_header(rom_data);
    println!("{:?}", headers);
    let flag_6_infos = flag_6::parse_flag_6(&headers.flag_6);
    println!("{:?}", flag_6_infos);
}
pub fn read_rom_header(rom_data: &Vec<u8>) -> RomHeader {
    let base_headers: &[u8] = &rom_data[0..15];
    let nes_header: &[u8] = validate_nes_header(&base_headers);
    let prg_rom_size: u8 = rom_data[HeaderPosition::PrgRomSize.position()];
    return RomHeader {
        authenticity_header: nes_header,
        prg_rom_size: u32::from(rom_data[HeaderPosition::PrgRomSize.position()]) * BASE_PRG_BANK_SIZE,
        chr_rom_size: u32::from(rom_data[HeaderPosition::ChrRomSize.position()]) * BASE_CHR_BANK_SIZE,
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
    return &nes_header[0..3];
}
