use anchor_lang::prelude::*;

use super::base::CoreStats;
use super::resources::ResourceKind;

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

    fn efficiency(&self, stats: CoreStats) -> f32 {
        fn ratio(stat: u16, requirement: u16) -> f32 {
            if requirement == 0 {
                f32::INFINITY
            } else {
                stat as f32 / requirement as f32
            }
        }

        let might = ratio(stats.might, self.minimum_stats.might);
        let speed = ratio(stats.speed, self.minimum_stats.speed);
        let intellect = ratio(stats.intellect, self.minimum_stats.intellect);

        might.min(speed).min(intellect)
    }

    pub fn energy_cost(&self, stats: CoreStats) -> u16 {
        let eff = self.efficiency(stats);
        if eff >= 1.5 {
            return 0;
        }

        let penalty = if eff <= 0.0 {
            1.5
        } else {
            (1.0 / eff).clamp(1.0, 1.5)
        };

        let energy = f32::from(self.base_energy_cost) * penalty;
        energy.ceil().min(u16::MAX as f32) as u16
    }

    pub fn stats_satisfied(&self, stats: CoreStats) -> bool {
        stats.might >= self.minimum_stats.might
            && stats.speed >= self.minimum_stats.speed
            && stats.intellect >= self.minimum_stats.intellect
    }
}

pub const QUEST_CODEX: [QuestCodexEntry; 18] = [
    QuestCodexEntry::new(
        1,
        QuestType::Job,
        "Caravan Guard",
        1,
        60 * 60,
        5,
        CoreStats::new(0, 1, 1),
        &[QuestReward::resource(ResourceKind::Gold, 100)],
    ),
    QuestCodexEntry::new(
        2,
        QuestType::Farm,
        "Woodcutting",
        1,
        60 * 60,
        5,
        CoreStats::new(1, 1, 0),
        &[QuestReward::resource(ResourceKind::Wood, 1)],
    ),
    QuestCodexEntry::new(
        3,
        QuestType::Raid,
        "Bandit Camp",
        1,
        60 * 60,
        7,
        CoreStats::new(1, 0, 1),
        &[QuestReward::experience(50)],
    ),
    QuestCodexEntry::new(
        4,
        QuestType::Raid,
        "Scouting Drills",
        1,
        45 * 60,
        6,
        CoreStats::new(0, 1, 0),
        &[QuestReward::experience(60)],
    ),
    QuestCodexEntry::new(
        5,
        QuestType::Farm,
        "Timber Stockpile",
        2,
        75 * 60,
        7,
        CoreStats::new(1, 1, 0),
        &[QuestReward::resource(ResourceKind::Wood, 2)],
    ),
    QuestCodexEntry::new(
        6,
        QuestType::Job,
        "Artisan Support",
        3,
        80 * 60,
        8,
        CoreStats::new(0, 1, 1),
        &[QuestReward::resource(ResourceKind::Gold, 150)],
    ),
    QuestCodexEntry::new(
        7,
        QuestType::Farm,
        "Hunting Grounds",
        2,
        90 * 60,
        6,
        CoreStats::new(1, 1, 0),
        &[QuestReward::resource(ResourceKind::RawLeather, 1)],
    ),
    QuestCodexEntry::new(
        8,
        QuestType::Farm,
        "Great Hunt",
        3,
        2 * 60 * 60,
        7,
        CoreStats::new(2, 1, 0),
        &[QuestReward::resource(ResourceKind::RawLeather, 2)],
    ),
    QuestCodexEntry::new(
        9,
        QuestType::Farm,
        "Mountain Prospecting",
        3,
        90 * 60,
        7,
        CoreStats::new(2, 0, 1),
        &[QuestReward::resource(ResourceKind::BronzeOre, 1)],
    ),
    QuestCodexEntry::new(
        10,
        QuestType::Farm,
        "Deep Excavation",
        4,
        2 * 60 * 60,
        8,
        CoreStats::new(2, 1, 1),
        &[QuestReward::resource(ResourceKind::BronzeOre, 2)],
    ),
    QuestCodexEntry::new(
        11,
        QuestType::Job,
        "Merchant Escort",
        2,
        75 * 60,
        6,
        CoreStats::new(0, 1, 1),
        &[QuestReward::resource(ResourceKind::Gold, 150)],
    ),
    QuestCodexEntry::new(
        12,
        QuestType::Job,
        "Royal Guard Duty",
        3,
        90 * 60,
        7,
        CoreStats::new(1, 1, 1),
        &[QuestReward::resource(ResourceKind::Gold, 200)],
    ),
    QuestCodexEntry::new(
        13,
        QuestType::Job,
        "Fortified Caravan",
        4,
        2 * 60 * 60,
        8,
        CoreStats::new(2, 1, 1),
        &[QuestReward::resource(ResourceKind::Gold, 275)],
    ),
    QuestCodexEntry::new(
        14,
        QuestType::Job,
        "City Defense",
        5,
        3 * 60 * 60,
        9,
        CoreStats::new(2, 2, 1),
        &[QuestReward::resource(ResourceKind::Gold, 350)],
    ),
    QuestCodexEntry::new(
        15,
        QuestType::Raid,
        "Forest Ambush",
        2,
        75 * 60,
        6,
        CoreStats::new(1, 1, 0),
        &[QuestReward::experience(80)],
    ),
    QuestCodexEntry::new(
        16,
        QuestType::Raid,
        "Ruined Keep",
        3,
        90 * 60,
        7,
        CoreStats::new(1, 0, 2),
        &[QuestReward::experience(120)],
    ),
    QuestCodexEntry::new(
        17,
        QuestType::Raid,
        "Warband Siege",
        4,
        2 * 60 * 60,
        8,
        CoreStats::new(2, 1, 1),
        &[QuestReward::experience(180)],
    ),
    QuestCodexEntry::new(
        18,
        QuestType::Raid,
        "Citadel Assault",
        5,
        3 * 60 * 60,
        9,
        CoreStats::new(2, 1, 2),
        &[QuestReward::experience(260)],
    ),
];

/// Optimized O(1) quest lookup by ID
#[inline]
pub fn quest_by_id(id: u16) -> Option<&'static QuestCodexEntry> {
    if id == 0 {
        return None;
    }
    let idx = (id - 1) as usize;
    QUEST_CODEX.get(idx)
}
