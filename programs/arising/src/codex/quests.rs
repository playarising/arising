use anchor_lang::prelude::*;

use super::base::CoreStats;
use super::resources::ResourceKind;

const LOW_LEVEL_PROTECTION: u8 = 10;
const LOW_LEVEL_ENERGY_STEP: f32 = 0.04;
const MINIMUM_EFFICIENCY_FLOOR: u16 = 1;
const MAX_EFFICIENCY_DISCOUNT: f32 = 0.6;
const MAX_EFFICIENCY_PENALTY: f32 = 1.4;
const MINIMUM_ENERGY_COST: u16 = 1;
const ENERGY_LEVEL_DIVISOR: u16 = 12;
const ENERGY_LEVEL_BASE: u16 = 5;
const STAT_MIGHT_DIVISOR: u16 = 6;
const STAT_SPEED_DIVISOR: u16 = 8;
const STAT_INTELLECT_DIVISOR: u16 = 9;
const STAT_SUM_LEVEL_DIVISOR: u16 = 4;
const STAT_SUM_LEVEL_BASE: u16 = 3;
const STAT_SUM_ENERGY_OFFSET: u16 = 2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum QuestType {
    Job,
    Farm,
    Raid,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QuestReward {
    Experience { amount: u64 },
    Resource { resource: ResourceKind, amount: u64 },
}

impl QuestReward {
    pub const fn experience(amount: u64) -> Self {
        Self::Experience { amount }
    }
    pub const fn resource(resource: ResourceKind, amount: u64) -> Self {
        Self::Resource { resource, amount }
    }
}

pub struct QuestCodexEntry {
    pub id: u16,
    pub quest_type: QuestType,
    pub name: &'static str,
    pub level_required: u8,
    pub cooldown_seconds: u32,
    pub base_energy_cost: u16,
    pub minimum_stats: CoreStats,
    pub rewards: &'static [QuestReward],
}

impl QuestCodexEntry {
    pub const fn new(
        id: u16,
        quest_type: QuestType,
        name: &'static str,
        level_required: u8,
        cooldown_seconds: u32,
        base_energy_cost: u16,
        minimum_stats: CoreStats,
        rewards: &'static [QuestReward],
    ) -> Self {
        Self {
            id,
            quest_type,
            name,
            level_required,
            cooldown_seconds,
            base_energy_cost,
            minimum_stats,
            rewards,
        }
    }

    fn level_based_energy_floor(&self) -> u16 {
        ENERGY_LEVEL_BASE + (self.level_required as u16 / ENERGY_LEVEL_DIVISOR.max(1))
    }

    pub fn effective_base_energy(&self) -> u16 {
        self.base_energy_cost
            .max(self.level_based_energy_floor())
            .max(MINIMUM_ENERGY_COST)
    }

    fn level_stat_floor(&self) -> CoreStats {
        let level = self.level_required as u16;
        CoreStats::new(
            1 + level / STAT_MIGHT_DIVISOR.max(1),
            1 + level / STAT_SPEED_DIVISOR.max(1),
            1 + level / STAT_INTELLECT_DIVISOR.max(1),
        )
    }

    fn stat_sum_floor(&self) -> u16 {
        let level_floor =
            STAT_SUM_LEVEL_BASE + (self.level_required as u16 / STAT_SUM_LEVEL_DIVISOR.max(1));
        let energy_floor = STAT_SUM_ENERGY_OFFSET + self.effective_base_energy() as u16;
        level_floor.max(energy_floor)
    }

    pub fn effective_minimum_stats(&self) -> CoreStats {
        let floored = self.minimum_stats.max_components(self.level_stat_floor());
        floored.enforce_total_floor(self.stat_sum_floor())
    }

    fn efficiency_requirement_floor(&self, character_level: u8) -> u16 {
        let quest_floor = MINIMUM_EFFICIENCY_FLOOR + (self.level_required as u16 / 10);
        let missing_levels = LOW_LEVEL_PROTECTION.saturating_sub(character_level) as u16;
        quest_floor + (missing_levels + 1) / 2
    }

    fn ratio(stat: u16, requirement: u16, floor: u16) -> f32 {
        let effective = requirement.max(floor).max(1);
        stat as f32 / effective as f32
    }

    fn efficiency(&self, stats: CoreStats, character_level: u8, required: CoreStats) -> f32 {
        let floor = self.efficiency_requirement_floor(character_level);
        let might = Self::ratio(stats.might, required.might, floor);
        let speed = Self::ratio(stats.speed, required.speed, floor);
        let intellect = Self::ratio(stats.intellect, required.intellect, floor);

        might.min(speed).min(intellect)
    }

    fn low_level_penalty(level: u8) -> f32 {
        if level >= LOW_LEVEL_PROTECTION {
            1.0
        } else {
            1.0 + f32::from(LOW_LEVEL_PROTECTION - level) * LOW_LEVEL_ENERGY_STEP
        }
    }

    fn cooldown_hours(&self) -> f32 {
        (self.cooldown_seconds as f32 / 3600.0).max(0.25)
    }

    fn cooldown_pressure(&self) -> f32 {
        let hours = self.cooldown_hours();
        (1.0 / hours).clamp(0.85, 1.25)
    }

    fn reward_weight(&self) -> f32 {
        self.rewards
            .iter()
            .copied()
            .map(|reward| match reward {
                QuestReward::Experience { amount } => (amount as f32 / 100.0).max(0.5),
                QuestReward::Resource { amount, .. } => (amount as f32).sqrt().max(0.5),
            })
            .sum::<f32>()
            .max(0.5)
    }

    fn reward_pressure(&self) -> f32 {
        let density = self.reward_weight() / self.cooldown_hours();
        (density / 1.5).clamp(0.85, 1.3)
    }

    pub fn energy_cost(&self, stats: CoreStats, character_level: u8) -> u16 {
        let required = self.effective_minimum_stats();
        let efficiency = self.efficiency(stats, character_level, required);
        let stat_modifier = if efficiency >= 1.0 {
            (1.0 / efficiency).clamp(MAX_EFFICIENCY_DISCOUNT, 1.0)
        } else if efficiency <= 0.0 {
            MAX_EFFICIENCY_PENALTY
        } else {
            (1.0 / efficiency).clamp(1.0, MAX_EFFICIENCY_PENALTY)
        };

        let energy = f32::from(self.effective_base_energy())
            * stat_modifier
            * Self::low_level_penalty(character_level)
            * self.cooldown_pressure()
            * self.reward_pressure()
            * self.stats_pressure(required);

        energy
            .ceil()
            .clamp(f32::from(MINIMUM_ENERGY_COST), f32::from(u16::MAX)) as u16
    }

    fn stats_pressure(&self, required: CoreStats) -> f32 {
        let total = required.sum() as f32;
        (total / 12.0).clamp(1.0, 1.35)
    }

    pub fn stats_satisfied(&self, stats: CoreStats) -> bool {
        let required = self.effective_minimum_stats();
        stats.might >= required.might
            && stats.speed >= required.speed
            && stats.intellect >= required.intellect
    }
}

pub const QUEST_CODEX: [QuestCodexEntry; 89] = [
    // === LEVEL 1-10: Basic Resources (Wood, Leather, Bronze intro) ===
    QuestCodexEntry::new(
        1,
        QuestType::Job,
        "Caravan Guard - Ard Trading Road",
        1,
        3600,
        5,
        CoreStats::new(2, 2, 1),
        &[QuestReward::resource(ResourceKind::Gold, 100)],
    ),
    QuestCodexEntry::new(
        2,
        QuestType::Farm,
        "Woodcutting - Oldholm Pass",
        1,
        3600,
        5,
        CoreStats::new(2, 2, 1),
        &[QuestReward::resource(ResourceKind::Wood, 1)],
    ),
    QuestCodexEntry::new(
        3,
        QuestType::Raid,
        "Bandit Camp - Yukrawa Plains",
        1,
        3600,
        1,
        CoreStats::new(2, 1, 2),
        &[QuestReward::experience(40)],
    ),
    QuestCodexEntry::new(
        4,
        QuestType::Raid,
        "Scouting Drills - Zhan Dunes",
        1,
        3600,
        1,
        CoreStats::new(2, 2, 1),
        &[QuestReward::experience(50)],
    ),
    QuestCodexEntry::new(
        5,
        QuestType::Farm,
        "Wood Quarry - Blackthorn Range",
        1,
        4500,
        1,
        CoreStats::new(2, 2, 1),
        &[QuestReward::resource(ResourceKind::Wood, 1)],
    ),
    QuestCodexEntry::new(
        6,
        QuestType::Farm,
        "Timber Stockpile - Rongden Delta",
        2,
        3600,
        5,
        CoreStats::new(2, 2, 1),
        &[QuestReward::resource(ResourceKind::Wood, 1)],
    ),
    QuestCodexEntry::new(
        7,
        QuestType::Farm,
        "Hunting Grounds - Shant Timberline",
        2,
        3600,
        5,
        CoreStats::new(2, 2, 1),
        &[QuestReward::resource(ResourceKind::RawLeather, 1)],
    ),
    QuestCodexEntry::new(
        8,
        QuestType::Farm,
        "Wood Vein Delve - Kamato Sea Coast",
        2,
        4500,
        1,
        CoreStats::new(2, 2, 2),
        &[QuestReward::resource(ResourceKind::Wood, 1)],
    ),
    QuestCodexEntry::new(
        9,
        QuestType::Job,
        "Artisan Support - Leesha Swamp",
        3,
        3600,
        6,
        CoreStats::new(3, 2, 2),
        &[QuestReward::resource(ResourceKind::Gold, 110)],
    ),
    QuestCodexEntry::new(
        10,
        QuestType::Farm,
        "Great Hunt - Miyakata Foothills",
        3,
        4500,
        7,
        CoreStats::new(3, 2, 2),
        &[QuestReward::resource(ResourceKind::RawLeather, 1)],
    ),
    QuestCodexEntry::new(
        11,
        QuestType::Raid,
        "Wood Smuggler Run - Elderwood Edge",
        3,
        5400,
        1,
        CoreStats::new(3, 2, 2),
        &[QuestReward::resource(ResourceKind::Wood, 1)],
    ),
    QuestCodexEntry::new(
        12,
        QuestType::Job,
        "Bridge Patrol - Oldholm Span",
        5,
        3600,
        6,
        CoreStats::new(3, 3, 2),
        &[QuestReward::resource(ResourceKind::Gold, 95)],
    ),
    QuestCodexEntry::new(
        13,
        QuestType::Raid,
        "Forest Ambush - Elderwood Edge",
        6,
        3600,
        8,
        CoreStats::new(5, 2, 2),
        &[QuestReward::experience(75)],
    ),
    QuestCodexEntry::new(
        14,
        QuestType::Job,
        "Frontier Escort - Elderwood Edge",
        8,
        3600,
        7,
        CoreStats::new(3, 6, 2),
        &[QuestReward::resource(ResourceKind::Gold, 105)],
    ),
    QuestCodexEntry::new(
        15,
        QuestType::Farm,
        "Bronze Quarry - Dalyar Ruins",
        10,
        3600,
        7,
        CoreStats::new(6, 3, 3),
        &[QuestReward::resource(ResourceKind::BronzeOre, 1)],
    ),
    // === LEVEL 11-20: Bronze Era ===
    QuestCodexEntry::new(
        16,
        QuestType::Farm,
        "Bronze Vein Delve - Raivat Crossing",
        11,
        4500,
        8,
        CoreStats::new(5, 5, 3),
        &[QuestReward::resource(ResourceKind::BronzeOre, 1)],
    ),
    QuestCodexEntry::new(
        17,
        QuestType::Raid,
        "Bronze Smuggler Run - Foyston Reaches",
        12,
        5400,
        9,
        CoreStats::new(6, 6, 2),
        &[QuestReward::resource(ResourceKind::BronzeOre, 1)],
    ),
    QuestCodexEntry::new(
        18,
        QuestType::Raid,
        "Bronze Keep Skirmish - Raivat Crossing",
        14,
        3600,
        10,
        CoreStats::new(7, 4, 4),
        &[QuestReward::experience(140)],
    ),
    QuestCodexEntry::new(
        19,
        QuestType::Farm,
        "Leesha Poachers - Elderwood Edge",
        15,
        4500,
        8,
        CoreStats::new(7, 5, 4),
        &[QuestReward::resource(ResourceKind::RawLeather, 1)],
    ),
    QuestCodexEntry::new(
        20,
        QuestType::Farm,
        "Iron Survey - Leesha Swamp",
        20,
        3600,
        8,
        CoreStats::new(9, 7, 4),
        &[QuestReward::resource(ResourceKind::IronOre, 1)],
    ),
    // === LEVEL 21-30: Iron Era ===
    QuestCodexEntry::new(
        21,
        QuestType::Farm,
        "Iron Quarry - Drakewatch Ridge",
        20,
        3600,
        8,
        CoreStats::new(9, 7, 4),
        &[QuestReward::resource(ResourceKind::IronOre, 1)],
    ),
    QuestCodexEntry::new(
        22,
        QuestType::Farm,
        "Iron Vein Delve - Chitu Marsh",
        21,
        4500,
        9,
        CoreStats::new(9, 7, 4),
        &[QuestReward::resource(ResourceKind::IronOre, 1)],
    ),
    QuestCodexEntry::new(
        23,
        QuestType::Raid,
        "Iron Smuggler Run - Mor Torneh Icefield",
        22,
        5400,
        10,
        CoreStats::new(9, 6, 6),
        &[QuestReward::resource(ResourceKind::IronOre, 1)],
    ),
    QuestCodexEntry::new(
        24,
        QuestType::Job,
        "Iron Supply Line - Chitu Marsh",
        24,
        3600,
        9,
        CoreStats::new(9, 8, 6),
        &[QuestReward::resource(ResourceKind::Gold, 140)],
    ),
    QuestCodexEntry::new(
        25,
        QuestType::Job,
        "Steel Patrol - Avos Crags",
        27,
        3600,
        10,
        CoreStats::new(11, 8, 6),
        &[QuestReward::resource(ResourceKind::Gold, 155)],
    ),
    QuestCodexEntry::new(
        26,
        QuestType::Raid,
        "Steel Vanguard Skirmish - Denvar Tower",
        31,
        3600,
        11,
        CoreStats::new(12, 8, 8),
        &[QuestReward::resource(ResourceKind::SteelOre, 1)],
    ),
    // === LEVEL 31-40: Steel Era ===
    QuestCodexEntry::new(
        27,
        QuestType::Farm,
        "Steel Quarry - Avos Crags",
        31,
        3600,
        10,
        CoreStats::new(12, 8, 8),
        &[QuestReward::resource(ResourceKind::SteelOre, 1)],
    ),
    QuestCodexEntry::new(
        28,
        QuestType::Farm,
        "Steel Vein Delve - Ard Trading Road",
        32,
        4500,
        10,
        CoreStats::new(12, 10, 7),
        &[QuestReward::resource(ResourceKind::SteelOre, 1)],
    ),
    QuestCodexEntry::new(
        29,
        QuestType::Raid,
        "Steel Smuggler Run - Denvar Tower",
        33,
        5400,
        11,
        CoreStats::new(12, 10, 7),
        &[QuestReward::resource(ResourceKind::SteelOre, 1)],
    ),
    QuestCodexEntry::new(
        30,
        QuestType::Farm,
        "Rhovunor Stalk - Kamato Sea Coast",
        35,
        4500,
        10,
        CoreStats::new(13, 10, 8),
        &[QuestReward::resource(ResourceKind::RawLeather, 1)],
    ),
    QuestCodexEntry::new(
        31,
        QuestType::Farm,
        "Silver Outcrop Sweep - Zhan Dunes",
        40,
        3600,
        10,
        CoreStats::new(15, 10, 10),
        &[QuestReward::resource(ResourceKind::SilverOre, 1)],
    ),
    QuestCodexEntry::new(
        32,
        QuestType::Farm,
        "Silver Quarry - Oldholm Pass",
        40,
        3600,
        10,
        CoreStats::new(15, 10, 10),
        &[QuestReward::resource(ResourceKind::SilverOre, 1)],
    ),
    // === LEVEL 41-49: Silver Era (still 1 resource) ===
    QuestCodexEntry::new(
        33,
        QuestType::Farm,
        "Silver Vein Delve - Zhan Dunes",
        41,
        4500,
        11,
        CoreStats::new(14, 12, 9),
        &[QuestReward::resource(ResourceKind::SilverOre, 1)],
    ),
    QuestCodexEntry::new(
        34,
        QuestType::Raid,
        "Silver Smuggler Run - Yukrawa Plains",
        42,
        5400,
        12,
        CoreStats::new(16, 11, 9),
        &[QuestReward::resource(ResourceKind::SilverOre, 1)],
    ),
    QuestCodexEntry::new(
        35,
        QuestType::Raid,
        "Silver Vanguard - Zhan Dunes",
        44,
        3600,
        12,
        CoreStats::new(16, 12, 10),
        &[QuestReward::experience(200)],
    ),
    QuestCodexEntry::new(
        36,
        QuestType::Raid,
        "Gold Prospectors' Clash - Leesha Swamp",
        50,
        4500,
        12,
        CoreStats::new(18, 13, 11),
        &[QuestReward::resource(ResourceKind::GoldOre, 1)],
    ),
    // === LEVEL 50-59: Gold Era (2 resources allowed) ===
    QuestCodexEntry::new(
        37,
        QuestType::Farm,
        "Gold Quarry - Rongden Delta",
        50,
        5400,
        12,
        CoreStats::new(18, 13, 11),
        &[QuestReward::resource(ResourceKind::GoldOre, 2)],
    ),
    QuestCodexEntry::new(
        38,
        QuestType::Farm,
        "Gold Vein Delve - Shant Timberline",
        51,
        4500,
        11,
        CoreStats::new(18, 14, 11),
        &[QuestReward::resource(ResourceKind::GoldOre, 1)],
    ),
    QuestCodexEntry::new(
        39,
        QuestType::Raid,
        "Gold Smuggler Run - Leesha Swamp",
        52,
        7200,
        14,
        CoreStats::new(18, 14, 12),
        &[
            QuestReward::resource(ResourceKind::Gold, 240),
            QuestReward::resource(ResourceKind::GoldOre, 2),
        ],
    ),
    QuestCodexEntry::new(
        40,
        QuestType::Job,
        "Guild Commission - Denvar Tower",
        55,
        5400,
        12,
        CoreStats::new(20, 13, 13),
        &[
            QuestReward::resource(ResourceKind::Gold, 210),
            QuestReward::experience(165),
        ],
    ),
    QuestCodexEntry::new(
        41,
        QuestType::Farm,
        "Platinum Survey - Miyakata Foothills",
        61,
        5400,
        13,
        CoreStats::new(22, 15, 13),
        &[QuestReward::resource(ResourceKind::PlatinumOre, 2)],
    ),
    // === LEVEL 60-69: Platinum Era (2 resources) ===
    QuestCodexEntry::new(
        42,
        QuestType::Farm,
        "Platinum Quarry - Miyakata Foothills",
        61,
        5400,
        13,
        CoreStats::new(22, 15, 13),
        &[QuestReward::resource(ResourceKind::PlatinumOre, 2)],
    ),
    QuestCodexEntry::new(
        43,
        QuestType::Farm,
        "Platinum Vein Delve - Elderwood Edge",
        62,
        4500,
        12,
        CoreStats::new(22, 16, 13),
        &[QuestReward::resource(ResourceKind::PlatinumOre, 1)],
    ),
    QuestCodexEntry::new(
        44,
        QuestType::Raid,
        "Platinum Smuggler Run - Blackthorn Range",
        63,
        7200,
        15,
        CoreStats::new(22, 15, 15),
        &[
            QuestReward::resource(ResourceKind::Gold, 280),
            QuestReward::resource(ResourceKind::PlatinumOre, 2),
        ],
    ),
    QuestCodexEntry::new(
        45,
        QuestType::Farm,
        "Kamato Pelts - Dalyar Ruins",
        65,
        5400,
        13,
        CoreStats::new(21, 17, 15),
        &[QuestReward::resource(ResourceKind::RawLeather, 2)],
    ),
    QuestCodexEntry::new(
        46,
        QuestType::Raid,
        "Platinum Warband - Avos Crags",
        67,
        5400,
        15,
        CoreStats::new(23, 17, 15),
        &[
            QuestReward::experience(250),
            QuestReward::resource(ResourceKind::PlatinumOre, 2),
        ],
    ),
    // === LEVEL 70-79: Adamantine Era (2 resources) ===
    QuestCodexEntry::new(
        47,
        QuestType::Farm,
        "Adamantine Quarry - Kamato Sea Coast",
        70,
        5400,
        13,
        CoreStats::new(24, 18, 15),
        &[QuestReward::resource(ResourceKind::AdamantineOre, 2)],
    ),
    QuestCodexEntry::new(
        48,
        QuestType::Farm,
        "Adamantine Vein Delve - Foyston Reaches",
        71,
        4500,
        13,
        CoreStats::new(24, 18, 16),
        &[QuestReward::resource(ResourceKind::AdamantineOre, 1)],
    ),
    QuestCodexEntry::new(
        49,
        QuestType::Raid,
        "Adamantine Smuggler Run - Dalyar Ruins",
        72,
        7200,
        16,
        CoreStats::new(24, 18, 17),
        &[
            QuestReward::resource(ResourceKind::Gold, 320),
            QuestReward::resource(ResourceKind::AdamantineOre, 2),
        ],
    ),
    QuestCodexEntry::new(
        50,
        QuestType::Job,
        "Adamantine Convoy - Mor Torneh Icefield",
        74,
        5400,
        14,
        CoreStats::new(24, 19, 17),
        &[
            QuestReward::resource(ResourceKind::Gold, 265),
            QuestReward::experience(220),
        ],
    ),
    QuestCodexEntry::new(
        51,
        QuestType::Raid,
        "Adamantine Bastion - Kamato Sea Coast",
        76,
        5400,
        16,
        CoreStats::new(26, 19, 17),
        &[QuestReward::resource(ResourceKind::AdamantineOre, 2)],
    ),
    QuestCodexEntry::new(
        52,
        QuestType::Job,
        "Cobalt Reconnaissance - Mor Torneh Icefield",
        80,
        5400,
        15,
        CoreStats::new(27, 21, 17),
        &[
            QuestReward::resource(ResourceKind::Gold, 275),
            QuestReward::resource(ResourceKind::CobaltOre, 1),
        ],
    ),
    // === LEVEL 80-89: Cobalt Era (3 resources allowed) ===
    QuestCodexEntry::new(
        53,
        QuestType::Farm,
        "Cobalt Quarry - Raivat Crossing",
        80,
        5400,
        14,
        CoreStats::new(27, 21, 17),
        &[QuestReward::resource(ResourceKind::CobaltOre, 2)],
    ),
    QuestCodexEntry::new(
        54,
        QuestType::Farm,
        "Cobalt Vein Delve - Mor Torneh Icefield",
        81,
        4500,
        14,
        CoreStats::new(26, 20, 19),
        &[QuestReward::resource(ResourceKind::CobaltOre, 1)],
    ),
    QuestCodexEntry::new(
        55,
        QuestType::Raid,
        "Cobalt Smuggler Run - Drakewatch Ridge",
        82,
        7200,
        17,
        CoreStats::new(26, 21, 19),
        &[
            QuestReward::resource(ResourceKind::Gold, 360),
            QuestReward::resource(ResourceKind::CobaltOre, 3),
        ],
    ),
    QuestCodexEntry::new(
        56,
        QuestType::Raid,
        "Cobalt Citadel Raid - Blackthorn Range",
        85,
        5400,
        17,
        CoreStats::new(28, 21, 19),
        &[
            QuestReward::experience(290),
            QuestReward::resource(ResourceKind::CobaltOre, 2),
        ],
    ),
    QuestCodexEntry::new(
        57,
        QuestType::Job,
        "Angelic Relief Convoy - Denvar Tower",
        91,
        5400,
        16,
        CoreStats::new(30, 22, 21),
        &[
            QuestReward::resource(ResourceKind::Gold, 300),
            QuestReward::resource(ResourceKind::AngelicEssence, 1),
        ],
    ),
    // === LEVEL 90-104: Angelic Era (3 resources) ===
    QuestCodexEntry::new(
        58,
        QuestType::Farm,
        "Angelic Quarry - Chitu Marsh",
        91,
        5400,
        15,
        CoreStats::new(30, 22, 21),
        &[QuestReward::resource(ResourceKind::AngelicEssence, 2)],
    ),
    QuestCodexEntry::new(
        59,
        QuestType::Farm,
        "Angelic Vein Delve - Denvar Tower",
        92,
        4500,
        15,
        CoreStats::new(30, 23, 21),
        &[QuestReward::resource(ResourceKind::AngelicEssence, 1)],
    ),
    QuestCodexEntry::new(
        60,
        QuestType::Raid,
        "Angelic Smuggler Run - Avos Crags",
        93,
        7200,
        18,
        CoreStats::new(30, 23, 21),
        &[
            QuestReward::resource(ResourceKind::Gold, 400),
            QuestReward::resource(ResourceKind::AngelicEssence, 3),
        ],
    ),
    QuestCodexEntry::new(
        61,
        QuestType::Farm,
        "Angelic Hunts - Foyston Reaches",
        95,
        5400,
        16,
        CoreStats::new(32, 23, 21),
        &[QuestReward::resource(ResourceKind::RawLeather, 3)],
    ),
    QuestCodexEntry::new(
        62,
        QuestType::Job,
        "Angelic Relief - Denvar Tower",
        96,
        5400,
        16,
        CoreStats::new(32, 24, 21),
        &[
            QuestReward::resource(ResourceKind::Gold, 320),
            QuestReward::experience(285),
        ],
    ),
    QuestCodexEntry::new(
        63,
        QuestType::Farm,
        "Crimson Scouts - Oldholm Pass",
        105,
        4500,
        16,
        CoreStats::new(34, 26, 23),
        &[QuestReward::resource(ResourceKind::CrimsonEssence, 1)],
    ),
    // === LEVEL 100-120: Crimson Era (3 resources) ===
    QuestCodexEntry::new(
        64,
        QuestType::Raid,
        "Crimson Border Clash - Yukrawa Plains",
        100,
        5400,
        17,
        CoreStats::new(32, 25, 23),
        &[
            QuestReward::resource(ResourceKind::CrimsonEssence, 2),
            QuestReward::experience(300),
        ],
    ),
    QuestCodexEntry::new(
        65,
        QuestType::Job,
        "Crimson Supply Run - Rongden Delta",
        102,
        7200,
        18,
        CoreStats::new(34, 24, 23),
        &[
            QuestReward::resource(ResourceKind::Gold, 340),
            QuestReward::experience(300),
            QuestReward::resource(ResourceKind::CrimsonEssence, 1),
        ],
    ),
    QuestCodexEntry::new(
        66,
        QuestType::Farm,
        "Crimson Quarry - Ard Trading Road",
        105,
        5400,
        16,
        CoreStats::new(34, 26, 23),
        &[QuestReward::resource(ResourceKind::CrimsonEssence, 2)],
    ),
    QuestCodexEntry::new(
        67,
        QuestType::Farm,
        "Crimson Vein Delve - Yukrawa Plains",
        106,
        4500,
        16,
        CoreStats::new(34, 27, 23),
        &[QuestReward::resource(ResourceKind::CrimsonEssence, 1)],
    ),
    QuestCodexEntry::new(
        68,
        QuestType::Raid,
        "Crimson Smuggler Run - Oldholm Pass",
        107,
        7200,
        19,
        CoreStats::new(35, 27, 23),
        &[
            QuestReward::resource(ResourceKind::Gold, 450),
            QuestReward::resource(ResourceKind::CrimsonEssence, 3),
        ],
    ),
    QuestCodexEntry::new(
        69,
        QuestType::Job,
        "Crimson Reinforcement - Oldholm Pass",
        110,
        7200,
        19,
        CoreStats::new(36, 26, 25),
        &[
            QuestReward::resource(ResourceKind::Gold, 360),
            QuestReward::experience(325),
            QuestReward::resource(ResourceKind::CrimsonEssence, 2),
        ],
    ),
    QuestCodexEntry::new(
        70,
        QuestType::Raid,
        "Crimson Rift Assault - Oldholm Pass",
        112,
        5400,
        19,
        CoreStats::new(36, 28, 25),
        &[
            QuestReward::experience(340),
            QuestReward::resource(ResourceKind::CrimsonEssence, 2),
        ],
    ),
    // === LEVEL 115-134: Fateful Era (3 resources) ===
    QuestCodexEntry::new(
        71,
        QuestType::Farm,
        "Fateful Outriders - Zhan Dunes",
        121,
        5400,
        18,
        CoreStats::new(39, 30, 26),
        &[QuestReward::resource(ResourceKind::FatefulEssence, 2)],
    ),
    QuestCodexEntry::new(
        72,
        QuestType::Raid,
        "Fateful Recon - Zhan Dunes",
        118,
        5400,
        19,
        CoreStats::new(38, 28, 27),
        &[
            QuestReward::resource(ResourceKind::FatefulEssence, 2),
            QuestReward::experience(350),
        ],
    ),
    QuestCodexEntry::new(
        73,
        QuestType::Farm,
        "Fateful Quarry - Zhan Dunes",
        121,
        5400,
        18,
        CoreStats::new(39, 30, 26),
        &[QuestReward::resource(ResourceKind::FatefulEssence, 2)],
    ),
    QuestCodexEntry::new(
        74,
        QuestType::Farm,
        "Fateful Vein Delve - Leesha Swamp",
        122,
        4500,
        18,
        CoreStats::new(40, 30, 26),
        &[QuestReward::resource(ResourceKind::FatefulEssence, 1)],
    ),
    QuestCodexEntry::new(
        75,
        QuestType::Raid,
        "Fateful Smuggler Run - Rongden Delta",
        123,
        7200,
        21,
        CoreStats::new(40, 30, 27),
        &[
            QuestReward::resource(ResourceKind::Gold, 500),
            QuestReward::resource(ResourceKind::FatefulEssence, 3),
        ],
    ),
    QuestCodexEntry::new(
        76,
        QuestType::Farm,
        "Fateful Hunts - Raivat Crossing",
        125,
        5400,
        19,
        CoreStats::new(40, 31, 27),
        &[QuestReward::resource(ResourceKind::RawLeather, 3)],
    ),
    QuestCodexEntry::new(
        77,
        QuestType::Job,
        "Fateful Bulwark - Rongden Delta",
        126,
        5400,
        19,
        CoreStats::new(41, 30, 28),
        &[
            QuestReward::resource(ResourceKind::Gold, 395),
            QuestReward::experience(370),
        ],
    ),
    QuestCodexEntry::new(
        78,
        QuestType::Job,
        "Altair Lines - Raivat Crossing",
        135,
        7200,
        20,
        CoreStats::new(44, 33, 29),
        &[
            QuestReward::resource(ResourceKind::Gold, 405),
            QuestReward::experience(380),
            QuestReward::resource(ResourceKind::AltairEssence, 1),
        ],
    ),
    // === LEVEL 135-141: Altair Era (3 resources) ===
    QuestCodexEntry::new(
        79,
        QuestType::Farm,
        "Altair Prospecting - Blackthorn Range",
        132,
        5400,
        19,
        CoreStats::new(44, 32, 28),
        &[QuestReward::resource(ResourceKind::AltairEssence, 2)],
    ),
    QuestCodexEntry::new(
        80,
        QuestType::Farm,
        "Altair Quarry - Shant Timberline",
        135,
        5400,
        19,
        CoreStats::new(44, 33, 29),
        &[QuestReward::resource(ResourceKind::AltairEssence, 2)],
    ),
    QuestCodexEntry::new(
        81,
        QuestType::Farm,
        "Altair Vein Delve - Blackthorn Range",
        136,
        4500,
        19,
        CoreStats::new(43, 34, 30),
        &[QuestReward::resource(ResourceKind::AltairEssence, 1)],
    ),
    QuestCodexEntry::new(
        82,
        QuestType::Raid,
        "Altair Smuggler Run - Miyakata Foothills",
        137,
        7200,
        22,
        CoreStats::new(43, 34, 30),
        &[
            QuestReward::resource(ResourceKind::Gold, 550),
            QuestReward::resource(ResourceKind::AltairEssence, 3),
        ],
    ),
    QuestCodexEntry::new(
        83,
        QuestType::Raid,
        "Altair Pinnacle Siege - Shant Timberline",
        140,
        5400,
        22,
        CoreStats::new(46, 34, 30),
        &[
            QuestReward::experience(420),
            QuestReward::resource(ResourceKind::AltairEssence, 2),
        ],
    ),
    // === Additional farming options for mid-high tiers ===
    QuestCodexEntry::new(
        84,
        QuestType::Farm,
        "Adamantine Prospect - Rhyolite Veins",
        76,
        4500,
        13,
        CoreStats::new(26, 19, 17),
        &[QuestReward::resource(ResourceKind::AdamantineOre, 2)],
    ),
    QuestCodexEntry::new(
        85,
        QuestType::Farm,
        "Cobalt Drift - Obsidian Hollows",
        86,
        4500,
        14,
        CoreStats::new(29, 21, 19),
        &[QuestReward::resource(ResourceKind::CobaltOre, 2)],
    ),
    QuestCodexEntry::new(
        86,
        QuestType::Farm,
        "Angelic Essence Hunt - Shimmering Peaks",
        97,
        4500,
        15,
        CoreStats::new(32, 24, 21),
        &[QuestReward::resource(ResourceKind::AngelicEssence, 2)],
    ),
    QuestCodexEntry::new(
        87,
        QuestType::Farm,
        "Crimson Essence Hunt - Ember Caverns",
        111,
        4500,
        17,
        CoreStats::new(36, 27, 25),
        &[QuestReward::resource(ResourceKind::CrimsonEssence, 2)],
    ),
    QuestCodexEntry::new(
        88,
        QuestType::Farm,
        "Fateful Essence Hunt - Nightfall Barrens",
        125,
        4500,
        19,
        CoreStats::new(40, 31, 27),
        &[QuestReward::resource(ResourceKind::FatefulEssence, 2)],
    ),
    QuestCodexEntry::new(
        89,
        QuestType::Farm,
        "Altair Essence Hunt - Starbound Crater",
        138,
        4500,
        20,
        CoreStats::new(44, 34, 30),
        &[QuestReward::resource(ResourceKind::AltairEssence, 2)],
    ),
];
#[inline]
pub fn quest_by_id(id: u16) -> Option<&'static QuestCodexEntry> {
    if id == 0 {
        return None;
    }
    let idx = (id - 1) as usize;
    QUEST_CODEX.get(idx)
}
