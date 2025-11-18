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
    pub const fn experience(amount: u64) -> Self { Self::Experience { amount } }
    pub const fn resource(resource: ResourceKind, amount: u64) -> Self { Self::Resource { resource, amount } }
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

pub const QUEST_CODEX: [QuestCodexEntry; 83] = [
    QuestCodexEntry::new(1, QuestType::Job, "Caravan Guard - Ard Trading Road", 1, 3600, 5, CoreStats::new(0, 1, 1), &[QuestReward::resource(ResourceKind::Gold, 70)]),
    QuestCodexEntry::new(2, QuestType::Farm, "Woodcutting - Oldholm Pass", 1, 3600, 5, CoreStats::new(1, 1, 0), &[QuestReward::resource(ResourceKind::Wood, 1)]),
    QuestCodexEntry::new(3, QuestType::Raid, "Bandit Camp - Yukrawa Plains", 1, 3600, 7, CoreStats::new(1, 0, 1), &[QuestReward::experience(50)]),
    QuestCodexEntry::new(4, QuestType::Raid, "Scouting Drills - Zhan Dunes", 1, 2700, 6, CoreStats::new(0, 1, 0), &[QuestReward::experience(60)]),
    QuestCodexEntry::new(5, QuestType::Farm, "Wood Quarry - Blackthorn Range", 1, 5400, 8, CoreStats::new(1, 1, 0), &[QuestReward::resource(ResourceKind::Wood, 1)]),
    QuestCodexEntry::new(6, QuestType::Farm, "Timber Stockpile - Rongden Delta", 2, 4500, 7, CoreStats::new(1, 1, 0), &[QuestReward::resource(ResourceKind::Wood, 2)]),
    QuestCodexEntry::new(7, QuestType::Farm, "Hunting Grounds - Shant Timberline", 2, 5400, 6, CoreStats::new(1, 1, 0), &[QuestReward::resource(ResourceKind::RawLeather, 1)]),
    QuestCodexEntry::new(8, QuestType::Farm, "Wood Vein Delve - Kamato Sea Coast", 2, 4500, 9, CoreStats::new(1, 2, 1), &[QuestReward::resource(ResourceKind::Wood, 1)]),
    QuestCodexEntry::new(9, QuestType::Job, "Artisan Support - Leesha Swamp", 3, 4800, 8, CoreStats::new(0, 1, 1), &[QuestReward::resource(ResourceKind::Gold, 110)]),
    QuestCodexEntry::new(10, QuestType::Farm, "Great Hunt - Miyakata Foothills", 3, 7200, 7, CoreStats::new(2, 1, 0), &[QuestReward::resource(ResourceKind::RawLeather, 2)]),
    QuestCodexEntry::new(11, QuestType::Raid, "Wood Smuggler Run - Elderwood Edge", 3, 7200, 10, CoreStats::new(2, 1, 1), &[QuestReward::resource(ResourceKind::Gold, 120), QuestReward::resource(ResourceKind::Wood, 1)]),
    QuestCodexEntry::new(12, QuestType::Job, "Bridge Patrol - Oldholm Span", 5, 4200, 6, CoreStats::new(1, 1, 1), &[QuestReward::resource(ResourceKind::Gold, 80)]),
    QuestCodexEntry::new(13, QuestType::Raid, "Forest Ambush - Elderwood Edge", 6, 4800, 8, CoreStats::new(2, 1, 1), &[QuestReward::experience(90), QuestReward::resource(ResourceKind::Wood, 1)]),
    QuestCodexEntry::new(14, QuestType::Job, "Frontier Escort - Elderwood Edge", 8, 4800, 7, CoreStats::new(1, 2, 1), &[QuestReward::resource(ResourceKind::Gold, 90), QuestReward::experience(80)]),
    QuestCodexEntry::new(15, QuestType::Farm, "Bronze Quarry - Dalyar Ruins", 10, 5400, 8, CoreStats::new(1, 1, 0), &[QuestReward::resource(ResourceKind::BronzeOre, 1)]),
    QuestCodexEntry::new(16, QuestType::Farm, "Bronze Vein Delve - Raivat Crossing", 11, 4500, 9, CoreStats::new(1, 2, 1), &[QuestReward::resource(ResourceKind::BronzeOre, 1)]),
    QuestCodexEntry::new(17, QuestType::Raid, "Bronze Smuggler Run - Foyston Reaches", 12, 7200, 11, CoreStats::new(3, 1, 1), &[QuestReward::resource(ResourceKind::Gold, 150), QuestReward::resource(ResourceKind::BronzeOre, 1)]),
    QuestCodexEntry::new(18, QuestType::Raid, "Bronze Keep Skirmish - Raivat Crossing", 14, 5400, 11, CoreStats::new(3, 2, 1), &[QuestReward::experience(160), QuestReward::resource(ResourceKind::BronzeOre, 1)]),
    QuestCodexEntry::new(19, QuestType::Farm, "Leesha Poachers - Elderwood Edge", 15, 7200, 10, CoreStats::new(2, 3, 1), &[QuestReward::resource(ResourceKind::RawLeather, 2)]),
    QuestCodexEntry::new(20, QuestType::Farm, "Iron Survey - Leesha Swamp", 18, 4800, 9, CoreStats::new(2, 2, 1), &[QuestReward::resource(ResourceKind::IronOre, 1)]),
    QuestCodexEntry::new(21, QuestType::Farm, "Iron Quarry - Drakewatch Ridge", 20, 5400, 9, CoreStats::new(2, 2, 1), &[QuestReward::resource(ResourceKind::IronOre, 1)]),
    QuestCodexEntry::new(22, QuestType::Farm, "Iron Vein Delve - Chitu Marsh", 21, 4500, 10, CoreStats::new(2, 3, 1), &[QuestReward::resource(ResourceKind::IronOre, 1)]),
    QuestCodexEntry::new(23, QuestType::Raid, "Iron Smuggler Run - Mor Torneh Icefield", 22, 7200, 12, CoreStats::new(4, 2, 2), &[QuestReward::resource(ResourceKind::Gold, 180), QuestReward::resource(ResourceKind::IronOre, 1)]),
    QuestCodexEntry::new(24, QuestType::Job, "Iron Supply Line - Chitu Marsh", 24, 5400, 9, CoreStats::new(3, 3, 2), &[QuestReward::resource(ResourceKind::Gold, 140), QuestReward::experience(120)]),
    QuestCodexEntry::new(25, QuestType::Job, "Steel Patrol - Avos Crags", 27, 5400, 10, CoreStats::new(3, 3, 2), &[QuestReward::resource(ResourceKind::Gold, 160), QuestReward::experience(140)]),
    QuestCodexEntry::new(26, QuestType::Raid, "Steel Vanguard Skirmish - Denvar Tower", 29, 6000, 12, CoreStats::new(4, 3, 3), &[QuestReward::resource(ResourceKind::SteelOre, 1), QuestReward::experience(180)]),
    QuestCodexEntry::new(27, QuestType::Farm, "Steel Quarry - Avos Crags", 31, 5400, 10, CoreStats::new(3, 3, 1), &[QuestReward::resource(ResourceKind::SteelOre, 2)]),
    QuestCodexEntry::new(28, QuestType::Farm, "Steel Vein Delve - Ard Trading Road", 32, 4500, 11, CoreStats::new(3, 4, 2), &[QuestReward::resource(ResourceKind::SteelOre, 1)]),
    QuestCodexEntry::new(29, QuestType::Raid, "Steel Smuggler Run - Denvar Tower", 33, 7200, 13, CoreStats::new(5, 3, 3), &[QuestReward::resource(ResourceKind::Gold, 220), QuestReward::resource(ResourceKind::SteelOre, 1)]),
    QuestCodexEntry::new(30, QuestType::Farm, "Rhovunor Stalk - Kamato Sea Coast", 35, 7200, 11, CoreStats::new(5, 5, 2), &[QuestReward::resource(ResourceKind::RawLeather, 2)]),
    QuestCodexEntry::new(31, QuestType::Farm, "Silver Outcrop Sweep - Zhan Dunes", 37, 5400, 11, CoreStats::new(4, 4, 2), &[QuestReward::resource(ResourceKind::SilverOre, 1)]),
    QuestCodexEntry::new(32, QuestType::Farm, "Silver Quarry - Oldholm Pass", 40, 5400, 11, CoreStats::new(4, 3, 2), &[QuestReward::resource(ResourceKind::SilverOre, 2)]),
    QuestCodexEntry::new(33, QuestType::Farm, "Silver Vein Delve - Zhan Dunes", 41, 4500, 11, CoreStats::new(4, 5, 2), &[QuestReward::resource(ResourceKind::SilverOre, 1)]),
    QuestCodexEntry::new(34, QuestType::Raid, "Silver Smuggler Run - Yukrawa Plains", 42, 7200, 14, CoreStats::new(6, 4, 3), &[QuestReward::resource(ResourceKind::Gold, 260), QuestReward::resource(ResourceKind::SilverOre, 1)]),
    QuestCodexEntry::new(35, QuestType::Raid, "Silver Vanguard - Zhan Dunes", 44, 6000, 14, CoreStats::new(6, 5, 4), &[QuestReward::experience(240), QuestReward::resource(ResourceKind::SilverOre, 1)]),
    QuestCodexEntry::new(36, QuestType::Raid, "Gold Prospectors' Clash - Leesha Swamp", 47, 6300, 13, CoreStats::new(6, 5, 3), &[QuestReward::resource(ResourceKind::GoldOre, 2)]),
    QuestCodexEntry::new(37, QuestType::Farm, "Gold Quarry - Rongden Delta", 50, 5400, 12, CoreStats::new(5, 4, 3), &[QuestReward::resource(ResourceKind::GoldOre, 2)]),
    QuestCodexEntry::new(38, QuestType::Farm, "Gold Vein Delve - Shant Timberline", 51, 4500, 12, CoreStats::new(5, 5, 3), &[QuestReward::resource(ResourceKind::GoldOre, 1)]),
    QuestCodexEntry::new(39, QuestType::Raid, "Gold Smuggler Run - Leesha Swamp", 52, 7200, 15, CoreStats::new(7, 5, 4), &[QuestReward::resource(ResourceKind::Gold, 300), QuestReward::resource(ResourceKind::GoldOre, 1)]),
    QuestCodexEntry::new(40, QuestType::Job, "Guild Commission - Denvar Tower", 55, 5400, 11, CoreStats::new(5, 5, 4), &[QuestReward::resource(ResourceKind::Gold, 190), QuestReward::experience(180)]),
    QuestCodexEntry::new(41, QuestType::Farm, "Platinum Survey - Miyakata Foothills", 58, 5400, 14, CoreStats::new(7, 6, 4), &[QuestReward::resource(ResourceKind::PlatinumOre, 2)]),
    QuestCodexEntry::new(42, QuestType::Farm, "Platinum Quarry - Miyakata Foothills", 61, 5400, 13, CoreStats::new(6, 5, 3), &[QuestReward::resource(ResourceKind::PlatinumOre, 3)]),
    QuestCodexEntry::new(43, QuestType::Farm, "Platinum Vein Delve - Elderwood Edge", 62, 4500, 13, CoreStats::new(6, 6, 4), &[QuestReward::resource(ResourceKind::PlatinumOre, 1)]),
    QuestCodexEntry::new(44, QuestType::Raid, "Platinum Smuggler Run - Blackthorn Range", 63, 7200, 16, CoreStats::new(8, 6, 5), &[QuestReward::resource(ResourceKind::Gold, 340), QuestReward::resource(ResourceKind::PlatinumOre, 1)]),
    QuestCodexEntry::new(45, QuestType::Farm, "Kamato Pelts - Dalyar Ruins", 65, 7200, 14, CoreStats::new(9, 8, 5), &[QuestReward::resource(ResourceKind::RawLeather, 3)]),
    QuestCodexEntry::new(46, QuestType::Raid, "Platinum Warband - Avos Crags", 67, 6300, 17, CoreStats::new(8, 7, 5), &[QuestReward::experience(320), QuestReward::resource(ResourceKind::PlatinumOre, 1)]),
    QuestCodexEntry::new(47, QuestType::Farm, "Adamantine Quarry - Kamato Sea Coast", 70, 5400, 13, CoreStats::new(6, 6, 4), &[QuestReward::resource(ResourceKind::AdamantineOre, 3)]),
    QuestCodexEntry::new(48, QuestType::Farm, "Adamantine Vein Delve - Foyston Reaches", 71, 4500, 13, CoreStats::new(7, 7, 4), &[QuestReward::resource(ResourceKind::AdamantineOre, 1)]),
    QuestCodexEntry::new(49, QuestType::Raid, "Adamantine Smuggler Run - Dalyar Ruins", 72, 7200, 17, CoreStats::new(9, 7, 6), &[QuestReward::resource(ResourceKind::Gold, 380), QuestReward::resource(ResourceKind::AdamantineOre, 2)]),
    QuestCodexEntry::new(50, QuestType::Job, "Adamantine Convoy - Mor Torneh Icefield", 74, 6000, 13, CoreStats::new(9, 8, 6), &[QuestReward::resource(ResourceKind::Gold, 240), QuestReward::experience(260)]),
    QuestCodexEntry::new(51, QuestType::Raid, "Adamantine Bastion - Kamato Sea Coast", 76, 6600, 17, CoreStats::new(9, 8, 6), &[QuestReward::resource(ResourceKind::AdamantineOre, 2)]),
    QuestCodexEntry::new(52, QuestType::Job, "Cobalt Reconnaissance - Mor Torneh Icefield", 78, 6000, 16, CoreStats::new(10, 8, 6), &[QuestReward::resource(ResourceKind::Gold, 240), QuestReward::experience(310), QuestReward::resource(ResourceKind::CobaltOre, 1)]),
    QuestCodexEntry::new(53, QuestType::Farm, "Cobalt Quarry - Raivat Crossing", 80, 5400, 14, CoreStats::new(7, 6, 5), &[QuestReward::resource(ResourceKind::CobaltOre, 3)]),
    QuestCodexEntry::new(54, QuestType::Farm, "Cobalt Vein Delve - Mor Torneh Icefield", 81, 4500, 14, CoreStats::new(8, 8, 5), &[QuestReward::resource(ResourceKind::CobaltOre, 1)]),
    QuestCodexEntry::new(55, QuestType::Raid, "Cobalt Smuggler Run - Drakewatch Ridge", 82, 7200, 18, CoreStats::new(10, 8, 6), &[QuestReward::resource(ResourceKind::Gold, 430), QuestReward::resource(ResourceKind::CobaltOre, 2)]),
    QuestCodexEntry::new(56, QuestType::Raid, "Cobalt Citadel Raid - Blackthorn Range", 85, 6600, 18, CoreStats::new(11, 9, 7), &[QuestReward::experience(360), QuestReward::resource(ResourceKind::CobaltOre, 1)]),
    QuestCodexEntry::new(57, QuestType::Job, "Angelic Relief Convoy - Denvar Tower", 88, 6600, 15, CoreStats::new(11, 10, 7), &[QuestReward::resource(ResourceKind::Gold, 260), QuestReward::experience(300), QuestReward::resource(ResourceKind::AngelicEssence, 1)]),
    QuestCodexEntry::new(58, QuestType::Farm, "Angelic Quarry - Chitu Marsh", 91, 5400, 15, CoreStats::new(8, 7, 5), &[QuestReward::resource(ResourceKind::AngelicEssence, 4)]),
    QuestCodexEntry::new(59, QuestType::Farm, "Angelic Vein Delve - Denvar Tower", 92, 4500, 15, CoreStats::new(10, 9, 6), &[QuestReward::resource(ResourceKind::AngelicEssence, 1)]),
    QuestCodexEntry::new(60, QuestType::Raid, "Angelic Smuggler Run - Avos Crags", 93, 7200, 19, CoreStats::new(11, 9, 7), &[QuestReward::resource(ResourceKind::Gold, 480), QuestReward::resource(ResourceKind::AngelicEssence, 2)]),
    QuestCodexEntry::new(61, QuestType::Farm, "Angelic Hunts - Foyston Reaches", 95, 7200, 16, CoreStats::new(12, 11, 7), &[QuestReward::resource(ResourceKind::RawLeather, 4)]),
    QuestCodexEntry::new(62, QuestType::Job, "Angelic Relief - Denvar Tower", 96, 6600, 15, CoreStats::new(12, 11, 8), &[QuestReward::resource(ResourceKind::Gold, 280), QuestReward::experience(340)]),
    QuestCodexEntry::new(63, QuestType::Farm, "Crimson Scouts - Oldholm Pass", 98, 6600, 17, CoreStats::new(12, 11, 8), &[QuestReward::resource(ResourceKind::CrimsonEssence, 1)]),
    QuestCodexEntry::new(64, QuestType::Raid, "Crimson Border Clash - Yukrawa Plains", 100, 7200, 18, CoreStats::new(13, 11, 8), &[QuestReward::resource(ResourceKind::CrimsonEssence, 2), QuestReward::experience(360)]),
    QuestCodexEntry::new(65, QuestType::Job, "Crimson Supply Run - Rongden Delta", 102, 6600, 18, CoreStats::new(13, 11, 9), &[QuestReward::resource(ResourceKind::Gold, 280), QuestReward::experience(350), QuestReward::resource(ResourceKind::CrimsonEssence, 1)]),
    QuestCodexEntry::new(66, QuestType::Farm, "Crimson Quarry - Ard Trading Road", 105, 5400, 16, CoreStats::new(9, 8, 6), &[QuestReward::resource(ResourceKind::CrimsonEssence, 4)]),
    QuestCodexEntry::new(67, QuestType::Farm, "Crimson Vein Delve - Yukrawa Plains", 106, 4500, 16, CoreStats::new(11, 10, 7), &[QuestReward::resource(ResourceKind::CrimsonEssence, 1)]),
    QuestCodexEntry::new(68, QuestType::Raid, "Crimson Smuggler Run - Oldholm Pass", 107, 7200, 20, CoreStats::new(12, 10, 8), &[QuestReward::resource(ResourceKind::Gold, 540), QuestReward::resource(ResourceKind::CrimsonEssence, 2)]),
    QuestCodexEntry::new(69, QuestType::Job, "Crimson Reinforcement - Oldholm Pass", 110, 7200, 19, CoreStats::new(14, 12, 9), &[QuestReward::resource(ResourceKind::Gold, 300), QuestReward::experience(380), QuestReward::resource(ResourceKind::CrimsonEssence, 1)]),
    QuestCodexEntry::new(70, QuestType::Raid, "Crimson Rift Assault - Oldholm Pass", 112, 7200, 21, CoreStats::new(14, 12, 9), &[QuestReward::experience(420), QuestReward::resource(ResourceKind::CrimsonEssence, 2)]),
    QuestCodexEntry::new(71, QuestType::Farm, "Fateful Outriders - Zhan Dunes", 115, 7200, 19, CoreStats::new(15, 13, 10), &[QuestReward::resource(ResourceKind::FatefulEssence, 2)]),
    QuestCodexEntry::new(72, QuestType::Raid, "Fateful Recon - Zhan Dunes", 118, 7200, 20, CoreStats::new(15, 13, 10), &[QuestReward::resource(ResourceKind::FatefulEssence, 2), QuestReward::experience(420)]),
    QuestCodexEntry::new(73, QuestType::Farm, "Fateful Quarry - Zhan Dunes", 121, 5400, 18, CoreStats::new(11, 9, 7), &[QuestReward::resource(ResourceKind::FatefulEssence, 5)]),
    QuestCodexEntry::new(74, QuestType::Farm, "Fateful Vein Delve - Leesha Swamp", 122, 4500, 17, CoreStats::new(13, 11, 8), &[QuestReward::resource(ResourceKind::FatefulEssence, 1)]),
    QuestCodexEntry::new(75, QuestType::Raid, "Fateful Smuggler Run - Rongden Delta", 123, 7200, 22, CoreStats::new(14, 12, 9), &[QuestReward::resource(ResourceKind::Gold, 600), QuestReward::resource(ResourceKind::FatefulEssence, 2)]),
    QuestCodexEntry::new(76, QuestType::Farm, "Fateful Hunts - Raivat Crossing", 125, 7200, 19, CoreStats::new(16, 14, 10), &[QuestReward::resource(ResourceKind::RawLeather, 5)]),
    QuestCodexEntry::new(77, QuestType::Job, "Fateful Bulwark - Rongden Delta", 126, 7200, 18, CoreStats::new(16, 14, 10), &[QuestReward::resource(ResourceKind::Gold, 320), QuestReward::experience(400)]),
    QuestCodexEntry::new(78, QuestType::Job, "Altair Lines - Raivat Crossing", 129, 7200, 19, CoreStats::new(16, 14, 11), &[QuestReward::resource(ResourceKind::Gold, 300), QuestReward::experience(410), QuestReward::resource(ResourceKind::AltairEssence, 1)]),
    QuestCodexEntry::new(79, QuestType::Farm, "Altair Prospecting - Blackthorn Range", 132, 7200, 19, CoreStats::new(16, 14, 11), &[QuestReward::resource(ResourceKind::AltairEssence, 2)]),
    QuestCodexEntry::new(80, QuestType::Farm, "Altair Quarry - Shant Timberline", 135, 5400, 19, CoreStats::new(12, 10, 8), &[QuestReward::resource(ResourceKind::AltairEssence, 5)]),
    QuestCodexEntry::new(81, QuestType::Farm, "Altair Vein Delve - Blackthorn Range", 136, 4500, 18, CoreStats::new(15, 12, 9), &[QuestReward::resource(ResourceKind::AltairEssence, 1)]),
    QuestCodexEntry::new(82, QuestType::Raid, "Altair Smuggler Run - Miyakata Foothills", 137, 7200, 23, CoreStats::new(15, 13, 10), &[QuestReward::resource(ResourceKind::Gold, 660), QuestReward::resource(ResourceKind::AltairEssence, 2)]),
    QuestCodexEntry::new(83, QuestType::Raid, "Altair Pinnacle Siege - Shant Timberline", 140, 7800, 24, CoreStats::new(18, 16, 12), &[QuestReward::experience(520), QuestReward::resource(ResourceKind::AltairEssence, 2)]),
];#[inline]
pub fn quest_by_id(id: u16) -> Option<&'static QuestCodexEntry> {
    if id == 0 {
        return None;
    }
    let idx = (id - 1) as usize;
    QUEST_CODEX.get(idx)
}
