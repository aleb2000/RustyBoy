use std::{fs, fmt::Display};
use anyhow::Error;
use num_enum::TryFromPrimitive;

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

pub struct Cartridge {
    // Header
    // TODO: Should these be slices instead of arrays??
    entry: [u8; 4], 
    logo: [u8; 48],
    title: [u8; 16],
    manufacturer: u32,
    cgb_flag: CGBMode,
    new_licensee_code: [u8; 2],
    sgb_flag: SGBMode,
    ty: CartridgeType,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    licensee_code: u8,
    version: u8,
    header_checksum: u8,
    global_checksum: u16,

    data: Vec<u8>
}

impl Cartridge {

    fn new(data: Vec<u8>) -> Cartridge {
        Cartridge {
            entry:  data[0x100..0x104].try_into().unwrap(),
            logo:   data[0x104..0x134].try_into().unwrap(),
            title:  data[0x134..0x144].try_into().unwrap(),

            // Manufacturer and CGB flag are part of the title data
            // Only older cartridges have titles up to 16 bytes
            manufacturer:   ((data[0x13F] as u32) << 24) +
                            ((data[0x140] as u32) << 16) +
                            ((data[0x141] as u32) << 8) +
                            ((data[0x142] as u32) << 0),
            cgb_flag:       CGBMode::try_from(data[0x143]).unwrap(),

            new_licensee_code: data[0x144..0x146].try_into().unwrap(),
            sgb_flag:       SGBMode::try_from(data[0x146]).unwrap(),
            ty:             CartridgeType::try_from(data[0x147]).unwrap(),
            rom_size:       data[0x148],
            ram_size:       data[0x149],
            dest_code:      data[0x14A],
            licensee_code:  data[0x14B],
            version:        data[0x14C],

            header_checksum:    data[0x14D],
            global_checksum:    ((data[0x14E] as u16) << 8) +
                                ((data[0x14F] as u16) << 0),

            data
        }   
    }

    pub fn from_file(path: &str) -> Result<Cartridge, Error> {
        let data = fs::read(path)?;
        
        Ok(Cartridge::new(data))
    }

    pub fn new_licensee_code_name(&self) -> &str {
        let idx = ((self.new_licensee_code[0] - b'0') * 10 + (self.new_licensee_code[1] - b'0')) as usize;
        if idx < 100 {
            NEW_LICENSEE_CODE_NAMES[idx]
        } else if self.new_licensee_code[0] == b'A' && self.new_licensee_code[1] == b'4' {
            "Konami (Yu-Gi-Oh!)"
        } else {
            "Unknown"
        }
    }
    
    pub fn licensee_code_name(&self) -> &str {
        LICENSEE_CODE_NAMES[self.licensee_code as usize]
    }

    pub fn destination_name(&self) -> &str {
        if self.dest_code == 0 {
            "Japanese"
        } else if self.dest_code == 1 {
            "Non-Japanese"
        } else {
            "Unknown"
        }
    }

    pub fn rom_size_bytes(&self) -> Option<usize> {
        if self.rom_size <= 8 {
            Some(((32 * bytesize::KIB) << self.rom_size) as usize)
        } else {
            match self.rom_size {
                52 => Some((1.1 * (bytesize::MIB as f64)) as usize),
                53 => Some((1.2 * (bytesize::MIB as f64)) as usize),
                54 => Some((1.5 * (bytesize::MIB as f64)) as usize),
                _  => None
            }
        }
    }

    pub fn ram_size_bytes(&self) -> Option<usize> {
        match self.ram_size {
            0 => Some(0),
            1 => None,
            2 => Some(8   * bytesize::KIB as usize),
            3 => Some(32  * bytesize::KIB as usize),
            4 => Some(128 * bytesize::KIB as usize),
            5 => Some(64  * bytesize::KIB as usize),
            _ => None
        }
    }
}

impl std::fmt::Display for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let new = self.licensee_code == LICENSEE_USE_NEW;
        let (licensee_name, licensee_code) = if new {
            (self.new_licensee_code_name(), String::from_utf8_lossy(&self.new_licensee_code).into_owned())
        } else {
            (self.licensee_code_name(), self.licensee_code.to_string())
        };

        write!(f,
            "title: {}\n\
             manufacturer: {}\n\
             cgb_flag: {}\n\
             sgb_flag: {}\n\
             type: {}\n\
             rom_size: {} ({})\n\
             ram_size: {} ({})\n\
             destination: {} ({})\n\
             licensee ({}): {} ({})\n\
             version: {}\n\
            ",
            String::from_utf8_lossy(&self.title),
            self.manufacturer,
            self.cgb_flag,
            self.sgb_flag,
            self.ty,
            bytesize::to_string(self.rom_size_bytes().unwrap() as u64, true), self.rom_size,
            bytesize::to_string(self.ram_size_bytes().unwrap() as u64, true), self.ram_size,
            self.destination_name(), self.dest_code,
            if new {"new"} else {"old"}, licensee_name, licensee_code,
            self.version
        )
    }
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum CGBMode {
    Disabled   = 0x00,
    CGBSupport = 0x80,  // Works on GBC as well as original Game Boy
    CGBOnly    = 0xC0   // Only works on GBC
}

impl Display for CGBMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CGBMode::Disabled   => write!(f, "Disabled"),
            CGBMode::CGBSupport => write!(f, "Color Game Boy Support"),
            CGBMode::CGBOnly    => write!(f, "Color Game Boy Only")
        }
    }
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum SGBMode {
    Disabled = 0x00,
    Enabled  = 0x03
}

impl Display for SGBMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SGBMode::Disabled => write!(f, "Disabled"),
            SGBMode::Enabled  => write!(f, "Enabled")
        }
    }
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum CartridgeType {
    RomOnly = 0x00,
    MBC1 = 0x01,
    MBC1Ram = 0x02,
    MBC1RamBattery = 0x03,
    MBC2 = 0x05,
    MBC2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    MMM01 = 0x0B,
    MMM01Ram = 0x0C,
    MMM01RamBattery = 0x0D,
    MBC3TimerBattery = 0x0F,
    MBC3TimerRamBattery = 0x10,
    MBC3 = 0x11,
    MBC3Ram = 0x12,
    MBC3RamBattery = 0x13,
    MBC5 = 0x19,
    MBC5Ram = 0x1A,
    MBC5RamBattery = 0x1B,
    MBC5Rumble = 0x1C,
    MBC5RumbleRam = 0x1D,
    MBC5RumbleRamBattery = 0x1E,
    MBC6 = 0x20,
    MBC7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF
}

impl Display for CartridgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CartridgeType::RomOnly                    => write!(f, "ROM ONLY"),
            CartridgeType::MBC1                       => write!(f, "MBC1"),
            CartridgeType::MBC1Ram                    => write!(f, "MBC1 + RAM"),
            CartridgeType::MBC1RamBattery             => write!(f, "MBC1 + RAM + BATTERY"),
            CartridgeType::MBC2                       => write!(f, "MBC2"),
            CartridgeType::MBC2Battery                => write!(f, "MBC2 + BATTERY"),
            CartridgeType::RomRam                     => write!(f, "ROM + RAM"),
            CartridgeType::RomRamBattery              => write!(f, "ROM + RAM + BATTERY"),
            CartridgeType::MMM01                      => write!(f, "MMM01"),
            CartridgeType::MMM01Ram                   => write!(f, "MMM01 + RAM"),
            CartridgeType::MMM01RamBattery            => write!(f, "MMM01 + RAM + BATTERY"),
            CartridgeType::MBC3TimerBattery           => write!(f, "MBC3 + TIMER + BATTERY"),
            CartridgeType::MBC3TimerRamBattery        => write!(f, "MBC3 + TIMER + RAM + BATTERY"),
            CartridgeType::MBC3                       => write!(f, "MBC3"),
            CartridgeType::MBC3Ram                    => write!(f, "MBC3 + RAM"),
            CartridgeType::MBC3RamBattery             => write!(f, "MBC3 + RAM + BATTERY"),
            CartridgeType::MBC5                       => write!(f, "MBC5"),
            CartridgeType::MBC5Ram                    => write!(f, "MBC5 + RAM"),
            CartridgeType::MBC5RamBattery             => write!(f, "MBC5 + RAM + BATTERY"),
            CartridgeType::MBC5Rumble                 => write!(f, "MBC5 + RUMBLE"),
            CartridgeType::MBC5RumbleRam              => write!(f, "MBC5 + RUMBLE + RAM"),
            CartridgeType::MBC5RumbleRamBattery       => write!(f, "MBC5 + RUMBLE + RAM + BATTERY"),
            CartridgeType::MBC6                       => write!(f, "MBC6"),
            CartridgeType::MBC7SensorRumbleRamBattery => write!(f, "MBC7 + SENSOR + RUMBLE + RAM + BATTERY"),
            CartridgeType::PocketCamera               => write!(f, "POCKET CAMERA"),
            CartridgeType::BandaiTama5                => write!(f, "BANDAI TAMA5"),
            CartridgeType::HuC3                       => write!(f, "HuC3"),
            CartridgeType::HuC1RamBattery             => write!(f, "HuC1 + RAM + BATTERY")
        }
    }
}

/**
 * The new licensee codes are 2 ascii characters,
 * hence this array is indexed in such a way that the two characters
 * converted to decimal digits give the correct licensee name.
 * This only works for licensee codes whose characters represent decimal digits.
 * Only the last licensee does not respect the criteria, which is included anyway for completeness.
 * The special value "Unknown" is used to fill the gaps of unknown or unused licensee codes.
 */
const NEW_LICENSEE_CODE_NAMES: &'static [&'static str] = &[
//  0             1                2               3                  4               5                        6                           7                   8              9
    "None",       "Nintendo R&D1", "Unknown",      "Unknown",         "Unknown",      "Unknown",               "Unknown",                  "Unknown",          "Capcom",      "Unknown",
    "Unknown",    "Unknown",       "Unknown",      "Electronic Arts", "Unknown",      "Unknown",               "Unknown",                  "Unknown",          "Hudson Soft", "b-ai",
    "kss",        "Unknown",       "pow",          "Unknown",         "PCM Complete", "san-x",                 "Unknown",                  "Unknown",          "Kemco Japan", "seta",
    "Viacom",     "Nintendo",      "Bandai",       "Ocean/Acclaim",   "Konami",       "Hector",                "Unknown",                  "Taito",            "Hudson",      "Banpresto",
    "Unknown",    "Ubi Soft",      "Atlus",        "Unknown",         "Malibu",       "Unknown",               "angel",                    "Bullet-Proof",     "Unknown",     "irem", 
    "Absolute",   "Acclaim",       "Activision",   "American sammy",  "Konami",       "Hi tech entertainment", "LJN",                      "Matchbox",         "Mattel",      "Milton Bradley",
    "Titus",      "Virgin",        "Unknown",      "Unknown",         "LucasArts",    "Unknown",               "Unknown",                  "Ocean",            "Unknown",     "Electronic Arts",
    "Infogrames", "Interplay",     "Broderbund",   "sculptured",      "Unknown",      "sci",                   "Unknown",                  "Unknown",          "THQ",         "Accolade",
    "misawa",     "Unknown",       "Unknown",      "lozc",            "Unknown",      "Unknown",               "Tokuma Shoten Intermedia", "Tsukuda Original", "Unknown",     "Unknown",
    "Unknown",    "Chunsoft",      "Video system", "Ocean/Acclaim",   "Unknown",      "Varie",                 "Yonezawa/s’pal",           "Kaneko",           "Unknown",     "Pack in soft",
    
    // Listed here for completeness but the index does not match the ascii code
    "Konami (Yu-Gi-Oh!)"
];

/**
 * The licensee code is a single byte.
 * Here the code directly indexes the correct name.
 * Names are taken from https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt
 * Some names are incomplete, they are recognizable as they start with an asterisk '*'.
 * The special value "Unknown" is used to fill the gaps of unknown or unused licensee codes.
 * The code 0x33 is a special value indicating that the new licensee code should be used instead.
 */
const LICENSEE_CODE_NAMES: &'static [&'static str] = &[
//  0x0                     0x1                0x2             0x3                      0x4                 0x5                0x6                 0x7                0x8                 0x9                  0xA                      0xB                      0xC                  0xD               0xE                0xF
    "none",                 "nintendo",        "Unknown",      "Unknown",               "Unknown",          "Unknown",         "Unknown",          "Unknown",         "capcom",           "hot-b",             "jaleco",                "coconuts",              "elite systems",     "Unknown",        "Unknown",         "Unknown",
    "Unknown",              "Unknown",         "Unknown",      "electronic arts",       "Unknown",          "Unknown",         "Unknown",          "Unknown",         "hudsonsoft",       "itc entertainment", "yanoman",               "Unknown",               "Unknown",           "clary",          "Unknown",         "virgin",
    "Unknown",              "Unknown",         "Unknown",      "Unknown",               "pcm complete",     "san-x",           "Unknown",          "Unknown",         "kotobuki systems", "seta",              "Unknown",               "Unknown",               "Unknown",           "Unknown",        "Unknown",         "Unknown",
    "infogrames",           "nintendo",        "bandai",       "NEW_LICENSEE",          "konami",           "hector",          "Unknown",          "Unknown",         "capcom",           "banpresto",         "Unknown",               "Unknown",               "*entertainment i",  "Unknown",        "gremlin",         "Unknown",
    "Unknown",              "ubi soft",        "atlus",        "Unknown",               "malibu",           "Unknown",         "angel",            "spectrum holoby", "Unknown",          "irem",              "virgin",                "Unknown",               "Unknown",           "malibu",         "Unknown",         "u.s. gold",
    "absolute",             "acclaim",         "activision",   "american sammy",        "gametek",          "park place",      "ljn",              "matchbox",        "Unknown",          "milton bradley",    "mindscape",             "romstar",               "naxat soft",        "tradewest",      "Unknown",         "Unknown",
    "titus",                "virgin",          "Unknown",      "Unknown",               "Unknown",          "Unknown",         "Unknown",          "ocean",           "Unknown",          "electronic arts",   "Unknown",               "Unknown",               "Unknown",           "Unknown",        "elite systems",   "electro brain",
    "infogrames",           "interplay",       "broderbund",   "sculptered soft",       "Unknown",          "the sales curve", "Unknown",          "Unknown",         "t*hq",             "accolade",          "triffix entertainment", "Unknown",               "microprose",        "Unknown",        "Unknown",         "kemco",
    "misawa entertainment", "Unknown",         "Unknown",      "lozc",                  "Unknown",          "Unknown",         "*tokuma shoten i", "Unknown",         "Unknown",          "Unknown",           "Unknown",               "bullet-proof software", "vic tokai",         "Unknown",        "ape",             "i'max",
    "Unknown",              "chun soft",       "video system", "tsuburava",             "Unknown",          "varie",           "yonezawa/s'pal",   "kaneko",          "Unknown",          "arc",               "nihon bussan",          "tecmo",                 "imagineer",         "banpresto",      "Unknown",         "nova",
    "Unknown",              "hori electric",   "bandai",       "Unknown",               "konami",           "Unknown",         "kawada",           "takara",          "Unknown",          "technos japan",     "broderbund",            "Unknown",               "toei animation",    "toho",           "Unknown",         "namco",
    "acclaim",              "ascii or nexoft", "bandai",       "Unknown",               "enix",             "Unknown",         "hal",              "snk",             "Unknown",          "pony canyon",       "*culture brain o",      "sunsoft",               "Unknown",           "sony imagesoft", "Unknown",         "sammy",
    "taito",                "Unknown",         "kemco",        "squaresoft",            "*tokuma shoten i", "data east",       "tonkin house",     "Unknown",         "koei",             "ufl",               "ultra",                 "vap",                   "use",               "meldac",         "*pony canyon or", "angel",
    "taito",                "sofel",           "quest",        "sigma enterprises",     "ask kodansha",     "Unknown",         "naxat soft",       "copya systems",   "Unknown",          "banpresto",         "tomy",                  "ljn",                   "Unknown",           "ncs",            "human",           "altron",
    "jaleco",               "towachiki",       "uutaka",       "varie",                 "Unknown",          "epoch",           "Unknown",          "athena",          "asmik",            "natsume",           "king records",          "atlus",                 "epic/sony records", "Unknown",        "igs",             "Unknown",
    "a wave",               "Unknown",         "Unknown",      "extreme entertainment", "Unknown",          "Unknown",         "Unknown",          "Unknown",         "Unknown",          "Unknown",           "Unknown",               "Unknown",               "Unknown",           "Unknown",        "Unknown",         "ljn"
];

const LICENSEE_USE_NEW: u8 = 0x33;