/*
 * 7       0
 * ---------
 * NNNN FTBM
 * 
 * N: Lower 4 bits of the mapper number
 * F: Four screen mode. 0 = no, 1 = yes. (When set, the M bit has no effect)
 * T: Trainer.  0 = no trainer present, 1 = 512 byte trainer at 7000-71FFh
 * B: SRAM at 6000-7FFFh battery backed.  0= no, 1 = yes
 * M: Mirroring.  0 = horizontal, 1 = vertical.
 *
 */
use utils::byte;
#[derive(Debug)]
pub struct Flag6 {
    nes_mapper: NesMapper,
    four_screen_mode: bool,
    trainer_mode: bool,
    sram_battery_backed: bool,
    mirroring: ScreenMode
}

enum Flag6BitPosition {
    FourScreenMode,
    TrainerMode,
    SramBatteryBacked,
    Mirroring
}

impl Flag6BitPosition {
    fn value(&self) -> u8 {
        match *self {
            Flag6BitPosition::FourScreenMode => 5,
            Flag6BitPosition::TrainerMode => 6,
            Flag6BitPosition::SramBatteryBacked => 7,
            Flag6BitPosition::Mirroring => 8
        }
    }
}

#[derive(Debug)]
enum ScreenMode {
    HORIZONTAL,
    VERTICAL
}

#[derive(Debug)]
enum NesMapper {
    NROM,
    MMC1,
    UNROM,
    CNROM,
    MMC3,
    MMC5,
    AOROM
}

impl NesMapper {
    fn value(&self) -> usize {
        match *self {
            NesMapper::NROM => 0,
            NesMapper::MMC1 => 1,
            NesMapper::UNROM => 2,
            NesMapper::CNROM => 3,
            NesMapper::MMC3 => 4,
            NesMapper::MMC5 => 5,
            NesMapper::AOROM => 7
        }
    }
}

fn nes_mapper_value_from_flag_6(flag_value: u8) -> Option<NesMapper> {
    let mapper_value = flag_value >> 4;
    match mapper_value {
        0 => Some(NesMapper::NROM),
        1 => Some(NesMapper::MMC1),
        2 => Some(NesMapper::UNROM),
        3 => Some(NesMapper::CNROM),
        4 => Some(NesMapper::MMC3),
        5 => Some(NesMapper::MMC5),
        7 => Some(NesMapper::AOROM),
        _ => None
    }
}

pub fn parse_flag_6(flag_value: &u8) -> Flag6 {
    let nes_mapper = nes_mapper_value_from_flag_6(flag_value.clone());
    match nes_mapper {
        Some(mapper) => {
            return Flag6 {
                nes_mapper: mapper,
                four_screen_mode: byte::extract_bit(flag_value, Flag6BitPosition::FourScreenMode.value()) == 1,
                trainer_mode: byte::extract_bit(flag_value, Flag6BitPosition::TrainerMode.value()) == 1,
                sram_battery_backed: byte::extract_bit(flag_value, Flag6BitPosition::SramBatteryBacked.value()) == 1,
                mirroring: match byte::extract_bit(flag_value, Flag6BitPosition::Mirroring.value()) {
                    0 => ScreenMode::HORIZONTAL,
                    1 => ScreenMode::VERTICAL,
                    _ => panic!("Unknown flag 6 screen mode")
                }
            }        
        },
        None => panic!("NES Mapper not found!  == {} ", flag_value >> 4)
    }
}
