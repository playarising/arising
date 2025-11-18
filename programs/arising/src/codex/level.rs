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

pub const LEVEL_CAP: u8 = 150;

pub const LEVEL_CURVE: [LevelCurveEntry; 149] = [
    LevelCurveEntry::new(1, 50, 50),
    LevelCurveEntry::new(2, 70, 120),
    LevelCurveEntry::new(3, 90, 210),
    LevelCurveEntry::new(4, 110, 320),
    LevelCurveEntry::new(5, 130, 450),
    LevelCurveEntry::new(6, 150, 600),
    LevelCurveEntry::new(7, 170, 770),
    LevelCurveEntry::new(8, 190, 960),
    LevelCurveEntry::new(9, 210, 1170),
    LevelCurveEntry::new(10, 230, 1400),
    LevelCurveEntry::new(11, 250, 1650),
    LevelCurveEntry::new(12, 270, 1920),
    LevelCurveEntry::new(13, 290, 2210),
    LevelCurveEntry::new(14, 310, 2520),
    LevelCurveEntry::new(15, 330, 2850),
    LevelCurveEntry::new(16, 350, 3200),
    LevelCurveEntry::new(17, 370, 3570),
    LevelCurveEntry::new(18, 390, 3960),
    LevelCurveEntry::new(19, 410, 4370),
    LevelCurveEntry::new(20, 430, 4800),
    LevelCurveEntry::new(21, 450, 5250),
    LevelCurveEntry::new(22, 470, 5720),
    LevelCurveEntry::new(23, 490, 6210),
    LevelCurveEntry::new(24, 510, 6720),
    LevelCurveEntry::new(25, 530, 7250),
    LevelCurveEntry::new(26, 550, 7800),
    LevelCurveEntry::new(27, 570, 8370),
    LevelCurveEntry::new(28, 590, 8960),
    LevelCurveEntry::new(29, 610, 9570),
    LevelCurveEntry::new(30, 630, 10200),
    LevelCurveEntry::new(31, 650, 10850),
    LevelCurveEntry::new(32, 670, 11520),
    LevelCurveEntry::new(33, 690, 12210),
    LevelCurveEntry::new(34, 710, 12920),
    LevelCurveEntry::new(35, 730, 13650),
    LevelCurveEntry::new(36, 750, 14400),
    LevelCurveEntry::new(37, 770, 15170),
    LevelCurveEntry::new(38, 790, 15960),
    LevelCurveEntry::new(39, 810, 16770),
    LevelCurveEntry::new(40, 830, 17600),
    LevelCurveEntry::new(41, 850, 18450),
    LevelCurveEntry::new(42, 870, 19320),
    LevelCurveEntry::new(43, 890, 20210),
    LevelCurveEntry::new(44, 910, 21120),
    LevelCurveEntry::new(45, 930, 22050),
    LevelCurveEntry::new(46, 950, 23000),
    LevelCurveEntry::new(47, 970, 23970),
    LevelCurveEntry::new(48, 990, 24960),
    LevelCurveEntry::new(49, 1010, 25970),
    LevelCurveEntry::new(50, 1030, 27000),
    LevelCurveEntry::new(51, 1050, 28050),
    LevelCurveEntry::new(52, 1070, 29120),
    LevelCurveEntry::new(53, 1090, 30210),
    LevelCurveEntry::new(54, 1110, 31320),
    LevelCurveEntry::new(55, 1130, 32450),
    LevelCurveEntry::new(56, 1150, 33600),
    LevelCurveEntry::new(57, 1170, 34770),
    LevelCurveEntry::new(58, 1190, 35960),
    LevelCurveEntry::new(59, 1210, 37170),
    LevelCurveEntry::new(60, 1230, 38400),
    LevelCurveEntry::new(61, 1250, 39650),
    LevelCurveEntry::new(62, 1270, 40920),
    LevelCurveEntry::new(63, 1290, 42210),
    LevelCurveEntry::new(64, 1310, 43520),
    LevelCurveEntry::new(65, 1330, 44850),
    LevelCurveEntry::new(66, 1350, 46200),
    LevelCurveEntry::new(67, 1370, 47570),
    LevelCurveEntry::new(68, 1390, 48960),
    LevelCurveEntry::new(69, 1410, 50370),
    LevelCurveEntry::new(70, 1430, 51800),
    LevelCurveEntry::new(71, 1450, 53250),
    LevelCurveEntry::new(72, 1470, 54720),
    LevelCurveEntry::new(73, 1490, 56210),
    LevelCurveEntry::new(74, 1510, 57720),
    LevelCurveEntry::new(75, 1530, 59250),
    LevelCurveEntry::new(76, 1550, 60800),
    LevelCurveEntry::new(77, 1570, 62370),
    LevelCurveEntry::new(78, 1590, 63960),
    LevelCurveEntry::new(79, 1610, 65570),
    LevelCurveEntry::new(80, 1630, 67200),
    LevelCurveEntry::new(81, 1650, 68850),
    LevelCurveEntry::new(82, 1670, 70520),
    LevelCurveEntry::new(83, 1690, 72210),
    LevelCurveEntry::new(84, 1710, 73920),
    LevelCurveEntry::new(85, 1730, 75650),
    LevelCurveEntry::new(86, 1750, 77400),
    LevelCurveEntry::new(87, 1770, 79170),
    LevelCurveEntry::new(88, 1790, 80960),
    LevelCurveEntry::new(89, 1810, 82770),
    LevelCurveEntry::new(90, 1830, 84600),
    LevelCurveEntry::new(91, 1850, 86450),
    LevelCurveEntry::new(92, 1870, 88320),
    LevelCurveEntry::new(93, 1890, 90210),
    LevelCurveEntry::new(94, 1910, 92120),
    LevelCurveEntry::new(95, 1930, 94050),
    LevelCurveEntry::new(96, 1950, 96000),
    LevelCurveEntry::new(97, 1970, 97970),
    LevelCurveEntry::new(98, 1990, 99960),
    LevelCurveEntry::new(99, 2010, 101970),
    LevelCurveEntry::new(100, 2030, 104000),
    LevelCurveEntry::new(101, 2050, 106050),
    LevelCurveEntry::new(102, 2070, 108120),
    LevelCurveEntry::new(103, 2090, 110210),
    LevelCurveEntry::new(104, 2110, 112320),
    LevelCurveEntry::new(105, 2130, 114450),
    LevelCurveEntry::new(106, 2150, 116600),
    LevelCurveEntry::new(107, 2170, 118770),
    LevelCurveEntry::new(108, 2190, 120960),
    LevelCurveEntry::new(109, 2210, 123170),
    LevelCurveEntry::new(110, 2230, 125400),
    LevelCurveEntry::new(111, 2250, 127650),
    LevelCurveEntry::new(112, 2270, 129920),
    LevelCurveEntry::new(113, 2290, 132210),
    LevelCurveEntry::new(114, 2310, 134520),
    LevelCurveEntry::new(115, 2330, 136850),
    LevelCurveEntry::new(116, 2350, 139200),
    LevelCurveEntry::new(117, 2370, 141570),
    LevelCurveEntry::new(118, 2390, 143960),
    LevelCurveEntry::new(119, 2410, 146370),
    LevelCurveEntry::new(120, 2430, 148800),
    LevelCurveEntry::new(121, 2450, 151250),
    LevelCurveEntry::new(122, 2470, 153720),
    LevelCurveEntry::new(123, 2490, 156210),
    LevelCurveEntry::new(124, 2510, 158720),
    LevelCurveEntry::new(125, 2530, 161250),
    LevelCurveEntry::new(126, 2550, 163800),
    LevelCurveEntry::new(127, 2570, 166370),
    LevelCurveEntry::new(128, 2590, 168960),
    LevelCurveEntry::new(129, 2610, 171570),
    LevelCurveEntry::new(130, 2630, 174200),
    LevelCurveEntry::new(131, 2650, 176850),
    LevelCurveEntry::new(132, 2670, 179520),
    LevelCurveEntry::new(133, 2690, 182210),
    LevelCurveEntry::new(134, 2710, 184920),
    LevelCurveEntry::new(135, 2730, 187650),
    LevelCurveEntry::new(136, 2750, 190400),
    LevelCurveEntry::new(137, 2770, 193170),
    LevelCurveEntry::new(138, 2790, 195960),
    LevelCurveEntry::new(139, 2810, 198770),
    LevelCurveEntry::new(140, 2830, 201600),
    LevelCurveEntry::new(141, 2850, 204450),
    LevelCurveEntry::new(142, 2870, 207320),
    LevelCurveEntry::new(143, 2890, 210210),
    LevelCurveEntry::new(144, 2910, 213120),
    LevelCurveEntry::new(145, 2930, 216050),
    LevelCurveEntry::new(146, 2950, 219000),
    LevelCurveEntry::new(147, 2970, 221970),
    LevelCurveEntry::new(148, 2990, 224960),
    LevelCurveEntry::new(149, 3010, 227970),
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
