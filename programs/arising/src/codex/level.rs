#[derive(Clone, Copy)]
pub struct LevelCurveEntry {
    pub level: u8,
    pub xp_to_next: u32,
    pub cumulative_xp: u64,
}

impl LevelCurveEntry {
    pub const fn new(level: u8, xp_to_next: u32, cumulative_xp: u64) -> Self {
        Self {
            level,
            xp_to_next,
            cumulative_xp,
        }
    }
}

pub const LEVEL_CAP: u8 = 50;

pub const LEVEL_CURVE: [LevelCurveEntry; 49] = [
    LevelCurveEntry::new(1, 150, 150),
    LevelCurveEntry::new(2, 200, 350),
    LevelCurveEntry::new(3, 250, 600),
    LevelCurveEntry::new(4, 300, 900),
    LevelCurveEntry::new(5, 350, 1_250),
    LevelCurveEntry::new(6, 400, 1_650),
    LevelCurveEntry::new(7, 450, 2_100),
    LevelCurveEntry::new(8, 500, 2_600),
    LevelCurveEntry::new(9, 550, 3_150),
    LevelCurveEntry::new(10, 600, 3_750),
    LevelCurveEntry::new(11, 9_880, 13_630),
    LevelCurveEntry::new(12, 11_720, 25_350),
    LevelCurveEntry::new(13, 13_680, 39_030),
    LevelCurveEntry::new(14, 15_760, 54_790),
    LevelCurveEntry::new(15, 17_960, 72_750),
    LevelCurveEntry::new(16, 20_280, 93_030),
    LevelCurveEntry::new(17, 22_720, 115_750),
    LevelCurveEntry::new(18, 25_280, 141_030),
    LevelCurveEntry::new(19, 27_960, 168_990),
    LevelCurveEntry::new(20, 30_760, 199_750),
    LevelCurveEntry::new(21, 5_750, 205_500),
    LevelCurveEntry::new(22, 6_610, 212_110),
    LevelCurveEntry::new(23, 7_600, 219_710),
    LevelCurveEntry::new(24, 8_730, 228_440),
    LevelCurveEntry::new(25, 10_030, 238_470),
    LevelCurveEntry::new(26, 11_520, 249_990),
    LevelCurveEntry::new(27, 13_220, 263_210),
    LevelCurveEntry::new(28, 15_160, 278_370),
    LevelCurveEntry::new(29, 17_360, 295_730),
    LevelCurveEntry::new(30, 19_860, 315_590),
    LevelCurveEntry::new(31, 22_680, 338_270),
    LevelCurveEntry::new(32, 25_860, 364_130),
    LevelCurveEntry::new(33, 29_440, 393_570),
    LevelCurveEntry::new(34, 33_420, 426_990),
    LevelCurveEntry::new(35, 37_880, 464_870),
    LevelCurveEntry::new(36, 42_810, 507_680),
    LevelCurveEntry::new(37, 48_260, 555_940),
    LevelCurveEntry::new(38, 54_270, 610_210),
    LevelCurveEntry::new(39, 60_910, 671_120),
    LevelCurveEntry::new(40, 68_210, 739_330),
    LevelCurveEntry::new(41, 76_260, 815_590),
    LevelCurveEntry::new(42, 85_100, 900_690),
    LevelCurveEntry::new(43, 94_800, 995_490),
    LevelCurveEntry::new(44, 105_420, 1_100_910),
    LevelCurveEntry::new(45, 117_070, 1_217_980),
    LevelCurveEntry::new(46, 129_840, 1_347_820),
    LevelCurveEntry::new(47, 143_810, 1_491_630),
    LevelCurveEntry::new(48, 159_190, 1_650_820),
    LevelCurveEntry::new(49, 176_000, 1_826_820),
];

pub fn requirement_for(level: u8) -> Option<&'static LevelCurveEntry> {
    LEVEL_CURVE.iter().find(|entry| entry.level == level)
}

pub fn next_requirement_after(level: u8) -> Option<&'static LevelCurveEntry> {
    LEVEL_CURVE.iter().find(|entry| entry.level == level + 1)
}

pub fn level_from_experience(total_xp: u64) -> u8 {
    let mut level = 1u8;
    for entry in LEVEL_CURVE.iter() {
        if total_xp < entry.cumulative_xp {
            break;
        }
        level = (entry.level + 1).min(LEVEL_CAP);
    }
    level
}

pub fn total_core_points_for_level(level: u8) -> u16 {
    level.saturating_sub(1) as u16
}

pub fn total_attribute_points_for_level(level: u8) -> u16 {
    (level.saturating_sub(1) / 5) as u16
}
