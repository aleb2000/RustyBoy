use std::fs;

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

    pub fn from_file(path: &str) -> Option<Cartridge> {
        let data = match fs::read(path) {
            Ok(v) => v,
            Err(_) => return None
        };
        
        Some(Cartridge::new(data))
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
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum CGBMode {
    Disabled   = 0x00,
    GBCSupport = 0x80,  // Works on GBC as well as original Game Boy
    GBCOnly    = 0xC0   // Only works on GBC
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum SGBMode {
    Disabled = 0x00,
    Enabled  = 0x03
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