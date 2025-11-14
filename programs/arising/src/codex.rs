use anchor_lang::prelude::*;
use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Civilization {
    Ard,
    Hartenn,
    Ikarans,
    Zhand,
    Shinkari,
    Tarki,
}

impl Civilization {
    pub const COUNT: usize = 6;

    pub const fn as_index(self) -> usize {
        match self {
            Civilization::Ard => 0,
            Civilization::Hartenn => 1,
            Civilization::Ikarans => 2,
            Civilization::Zhand => 3,
            Civilization::Shinkari => 4,
            Civilization::Tarki => 5,
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Civilization::Ard => "ard",
            Civilization::Hartenn => "hartenn",
            Civilization::Ikarans => "ikarans",
            Civilization::Zhand => "zhand",
            Civilization::Shinkari => "shinkari",
            Civilization::Tarki => "tarki",
        }
    }

    pub fn metadata_uri(self, character_id: u16) -> String {
        format!(
            "https://mints-{}.playarising.com/metadata/{}",
            self.slug(),
            character_id
        )
    }

    pub const fn seed_bytes(self) -> [u8; 1] {
        [self as u8]
    }

    pub const fn base_stats(self) -> CoreStats {
        match self {
            Civilization::Ard => CoreStats::new(3, 2, 1),
            Civilization::Hartenn => CoreStats::new(3, 1, 2),
            Civilization::Ikarans => CoreStats::new(1, 3, 2),
            Civilization::Zhand => CoreStats::new(1, 2, 3),
            Civilization::Shinkari => CoreStats::new(2, 3, 1),
            Civilization::Tarki => CoreStats::new(3, 2, 1),
        }
    }

    pub const fn base_attributes(self) -> Attributes {
        match self {
            Civilization::Ard => Attributes::new(2, 1, 0, 0, 0, 0),
            Civilization::Hartenn => Attributes::new(1, 0, 0, 2, 0, 0),
            Civilization::Ikarans => Attributes::new(1, 0, 0, 0, 1, 1),
            Civilization::Zhand => Attributes::new(0, 0, 2, 0, 0, 1),
            Civilization::Shinkari => Attributes::new(1, 0, 1, 0, 0, 1),
            Civilization::Tarki => Attributes::new(2, 0, 0, 0, 0, 1),
        }
    }

    pub const fn passive(self) -> PassiveEffect {
        match self {
            Civilization::Ard => PassiveEffect::KnightsHonor,
            Civilization::Hartenn => PassiveEffect::ForgeMastery,
            Civilization::Ikarans => PassiveEffect::SylvanGrace,
            Civilization::Zhand => PassiveEffect::DuneCunning,
            Civilization::Shinkari => PassiveEffect::ArtisanFocus,
            Civilization::Tarki => PassiveEffect::RaidersFortune,
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Civilization::Ard => "Ard",
            Civilization::Hartenn => "Hartenn",
            Civilization::Ikarans => "I'karans",
            Civilization::Zhand => "Zhand",
            Civilization::Shinkari => "Shinkari",
            Civilization::Tarki => "Tark'i",
        }
    }
}

impl fmt::Display for Civilization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PassiveEffect {
    KnightsHonor,
    ForgeMastery,
    SylvanGrace,
    DuneCunning,
    ArtisanFocus,
    RaidersFortune,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterClass {
    ArdKnight,
    ArdTemplar,
    HartennEngineer,
    HartennWarden,
    IkaransRanger,
    IkaransDruid,
    ZhandSandblade,
    ZhandSeer,
    ShinkariSamurai,
    ShinkariOnmyoji,
    TarkiRaider,
    TarkiSkald,
}

pub struct ClassBonus {
    pub stats: CoreStats,
    pub attributes: Attributes,
}

impl CharacterClass {
    pub fn civilization(self) -> Civilization {
        match self {
            CharacterClass::ArdKnight | CharacterClass::ArdTemplar => Civilization::Ard,
            CharacterClass::HartennEngineer | CharacterClass::HartennWarden => {
                Civilization::Hartenn
            }
            CharacterClass::IkaransRanger | CharacterClass::IkaransDruid => Civilization::Ikarans,
            CharacterClass::ZhandSandblade | CharacterClass::ZhandSeer => Civilization::Zhand,
            CharacterClass::ShinkariSamurai | CharacterClass::ShinkariOnmyoji => {
                Civilization::Shinkari
            }
            CharacterClass::TarkiRaider | CharacterClass::TarkiSkald => Civilization::Tarki,
        }
    }

    pub fn bonuses(self) -> ClassBonus {
        match self {
            CharacterClass::ArdKnight => ClassBonus {
                stats: CoreStats::new(1, 0, 0),
                attributes: Attributes::new(0, 2, 0, 0, 0, 0),
            },
            CharacterClass::ArdTemplar => ClassBonus {
                stats: CoreStats::new(0, 0, 1),
                attributes: Attributes::new(0, 1, 0, 1, 0, 0),
            },
            CharacterClass::HartennEngineer => ClassBonus {
                stats: CoreStats::new(0, 0, 1),
                attributes: Attributes::new(0, 0, 2, 0, 0, 0),
            },
            CharacterClass::HartennWarden => ClassBonus {
                stats: CoreStats::new(1, 0, 0),
                attributes: Attributes::new(1, 1, 0, 0, 0, 0),
            },
            CharacterClass::IkaransRanger => ClassBonus {
                stats: CoreStats::new(0, 1, 0),
                attributes: Attributes::new(1, 0, 0, 0, 1, 0),
            },
            CharacterClass::IkaransDruid => ClassBonus {
                stats: CoreStats::new(0, 0, 1),
                attributes: Attributes::new(0, 0, 0, 2, 0, 0),
            },
            CharacterClass::ZhandSandblade => ClassBonus {
                stats: CoreStats::new(0, 1, 0),
                attributes: Attributes::new(0, 0, 0, 0, 0, 2),
            },
            CharacterClass::ZhandSeer => ClassBonus {
                stats: CoreStats::new(0, 0, 1),
                attributes: Attributes::new(0, 0, 2, 0, 0, 0),
            },
            CharacterClass::ShinkariSamurai => ClassBonus {
                stats: CoreStats::new(1, 0, 0),
                attributes: Attributes::new(1, 0, 0, 0, 0, 1),
            },
            CharacterClass::ShinkariOnmyoji => ClassBonus {
                stats: CoreStats::new(0, 0, 1),
                attributes: Attributes::new(0, 0, 1, 1, 0, 0),
            },
            CharacterClass::TarkiRaider => ClassBonus {
                stats: CoreStats::new(1, 0, 0),
                attributes: Attributes::new(2, 0, 0, 0, 0, 0),
            },
            CharacterClass::TarkiSkald => ClassBonus {
                stats: CoreStats::new(0, 1, 0),
                attributes: Attributes::new(0, 0, 0, 0, 1, 1),
            },
        }
    }

    pub fn options_for(civilization: Civilization) -> &'static [CharacterClass; 2] {
        match civilization {
            Civilization::Ard => &[CharacterClass::ArdKnight, CharacterClass::ArdTemplar],
            Civilization::Hartenn => &[
                CharacterClass::HartennEngineer,
                CharacterClass::HartennWarden,
            ],
            Civilization::Ikarans => &[CharacterClass::IkaransRanger, CharacterClass::IkaransDruid],
            Civilization::Zhand => &[CharacterClass::ZhandSandblade, CharacterClass::ZhandSeer],
            Civilization::Shinkari => &[
                CharacterClass::ShinkariSamurai,
                CharacterClass::ShinkariOnmyoji,
            ],
            Civilization::Tarki => &[CharacterClass::TarkiRaider, CharacterClass::TarkiSkald],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct CoreStats {
    pub might: u16,
    pub speed: u16,
    pub intellect: u16,
}

impl CoreStats {
    pub const SIZE: usize = 2 + 2 + 2;

    pub const fn new(might: u16, speed: u16, intellect: u16) -> Self {
        Self {
            might,
            speed,
            intellect,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Attributes {
    pub atk: i16,
    pub def: i16,
    pub mag_atk: i16,
    pub mag_def: i16,
    pub range: i16,
    pub rate: i16,
}

impl Attributes {
    pub const SIZE: usize = 2 * 6;

    pub const fn new(
        atk: i16,
        def: i16,
        mag_atk: i16,
        mag_def: i16,
        range: i16,
        rate: i16,
    ) -> Self {
        Self {
            atk,
            def,
            mag_atk,
            mag_def,
            range,
            rate,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0, 0, 0, 0, 0, 0)
    }
}

pub struct CivilizationCodexEntry {
    pub civilization: Civilization,
    pub base_stats: CoreStats,
    pub base_attributes: Attributes,
    pub passive: PassiveEffect,
}

pub const CIVILIZATION_CODEX: [CivilizationCodexEntry; 6] = [
    CivilizationCodexEntry {
        civilization: Civilization::Ard,
        base_stats: Civilization::Ard.base_stats(),
        base_attributes: Civilization::Ard.base_attributes(),
        passive: Civilization::Ard.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Hartenn,
        base_stats: Civilization::Hartenn.base_stats(),
        base_attributes: Civilization::Hartenn.base_attributes(),
        passive: Civilization::Hartenn.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Ikarans,
        base_stats: Civilization::Ikarans.base_stats(),
        base_attributes: Civilization::Ikarans.base_attributes(),
        passive: Civilization::Ikarans.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Zhand,
        base_stats: Civilization::Zhand.base_stats(),
        base_attributes: Civilization::Zhand.base_attributes(),
        passive: Civilization::Zhand.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Shinkari,
        base_stats: Civilization::Shinkari.base_stats(),
        base_attributes: Civilization::Shinkari.base_attributes(),
        passive: Civilization::Shinkari.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Tarki,
        base_stats: Civilization::Tarki.base_stats(),
        base_attributes: Civilization::Tarki.base_attributes(),
        passive: Civilization::Tarki.passive(),
    },
];

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum QuestType {
    Job,
    Farm,
    Raid,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResourceKind {
    Gold,
    Log,
    Plank,
    WoodenSword,
    TrainingGreatsword,
    WoodenShield,
    LeatherArmor,
    LeatherLeggings,
    LeatherBoots,
    LeatherHelmet,
    BronzeRing,
    BronzeAmulet,
    TravelersCloak,
    HuntingQuiver,
}

impl ResourceKind {
    pub const fn seed_bytes(self) -> &'static [u8] {
        match self {
            ResourceKind::Gold => b"gold",
            ResourceKind::Log => b"log",
            ResourceKind::Plank => b"plank",
            ResourceKind::WoodenSword => b"wooden_sword",
            ResourceKind::TrainingGreatsword => b"training_greatsword",
            ResourceKind::WoodenShield => b"wooden_shield",
            ResourceKind::LeatherArmor => b"leather_armor",
            ResourceKind::LeatherLeggings => b"leather_leggings",
            ResourceKind::LeatherBoots => b"leather_boots",
            ResourceKind::LeatherHelmet => b"leather_helmet",
            ResourceKind::BronzeRing => b"bronze_ring",
            ResourceKind::BronzeAmulet => b"bronze_amulet",
            ResourceKind::TravelersCloak => b"travelers_cloak",
            ResourceKind::HuntingQuiver => b"hunting_quiver",
        }
    }

    pub const fn decimals(self) -> u8 {
        0
    }

    pub const fn symbol(self) -> &'static str {
        match self {
            ResourceKind::Gold => "GOLD",
            ResourceKind::Log => "LOG",
            ResourceKind::Plank => "PLANK",
            ResourceKind::WoodenSword => "WOOD_SWORD",
            ResourceKind::TrainingGreatsword => "TRAIN_GSWORD",
            ResourceKind::WoodenShield => "WOOD_SHIELD",
            ResourceKind::LeatherArmor => "LEATHER_ARMOR",
            ResourceKind::LeatherLeggings => "LEATHER_LEGS",
            ResourceKind::LeatherBoots => "LEATHER_BOOTS",
            ResourceKind::LeatherHelmet => "LEATHER_HELM",
            ResourceKind::BronzeRing => "BRONZE_RING",
            ResourceKind::BronzeAmulet => "BRONZE_AMULET",
            ResourceKind::TravelersCloak => "TRAVEL_CLOAK",
            ResourceKind::HuntingQuiver => "HUNTING_QUIVER",
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EquipmentSlot {
    LeftHand,
    RightHand,
    Chest,
    Legs,
    Boots,
    Helmet,
    RingOne,
    RingTwo,
    AmuletOne,
    AmuletTwo,
    Back,
}

impl EquipmentSlot {
    pub const COUNT: usize = 11;

    pub const fn is_hand(self) -> bool {
        matches!(self, EquipmentSlot::LeftHand | EquipmentSlot::RightHand)
    }

    pub const fn is_ring(self) -> bool {
        matches!(self, EquipmentSlot::RingOne | EquipmentSlot::RingTwo)
    }

    pub const fn is_amulet(self) -> bool {
        matches!(self, EquipmentSlot::AmuletOne | EquipmentSlot::AmuletTwo)
    }

    pub const fn companion_hand(self) -> Option<EquipmentSlot> {
        match self {
            EquipmentSlot::LeftHand => Some(EquipmentSlot::RightHand),
            EquipmentSlot::RightHand => Some(EquipmentSlot::LeftHand),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EquipmentCategory {
    Weapon,
    Armor,
    Accessory,
    Back,
}

#[derive(Clone, Copy)]
pub struct EquipmentDefinition {
    pub resource: ResourceKind,
    pub category: EquipmentCategory,
    pub allowed_slots: &'static [EquipmentSlot],
    pub occupies_both_hands: bool,
    pub level_required: u8,
    pub attribute_bonus: Attributes,
}

impl EquipmentDefinition {
    pub const fn new(
        resource: ResourceKind,
        category: EquipmentCategory,
        allowed_slots: &'static [EquipmentSlot],
        occupies_both_hands: bool,
        level_required: u8,
        attribute_bonus: Attributes,
    ) -> Self {
        Self {
            resource,
            category,
            allowed_slots,
            occupies_both_hands,
            level_required,
            attribute_bonus,
        }
    }

    pub fn supports_slot(&self, slot: EquipmentSlot) -> bool {
        self.allowed_slots.iter().any(|allowed| *allowed == slot)
    }
}

const HAND_SLOTS: [EquipmentSlot; 2] = [EquipmentSlot::LeftHand, EquipmentSlot::RightHand];
const LEFT_HAND_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::LeftHand];
const CHEST_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Chest];
const LEGS_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Legs];
const BOOTS_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Boots];
const HELMET_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Helmet];
const RING_SLOTS: [EquipmentSlot; 2] = [EquipmentSlot::RingOne, EquipmentSlot::RingTwo];
const AMULET_SLOTS: [EquipmentSlot; 2] = [EquipmentSlot::AmuletOne, EquipmentSlot::AmuletTwo];
const BACK_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Back];

pub const EQUIPMENT_CODEX: [EquipmentDefinition; 11] = [
    EquipmentDefinition::new(
        ResourceKind::WoodenSword,
        EquipmentCategory::Weapon,
        &HAND_SLOTS,
        false,
        1,
        Attributes::new(2, 0, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::TrainingGreatsword,
        EquipmentCategory::Weapon,
        &HAND_SLOTS,
        true,
        3,
        Attributes::new(4, 0, 0, 0, 0, -1),
    ),
    EquipmentDefinition::new(
        ResourceKind::WoodenShield,
        EquipmentCategory::Weapon,
        &LEFT_HAND_SLOT,
        false,
        2,
        Attributes::new(0, 2, 0, 1, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherArmor,
        EquipmentCategory::Armor,
        &CHEST_SLOT,
        false,
        3,
        Attributes::new(0, 2, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherLeggings,
        EquipmentCategory::Armor,
        &LEGS_SLOT,
        false,
        2,
        Attributes::new(0, 1, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherBoots,
        EquipmentCategory::Armor,
        &BOOTS_SLOT,
        false,
        2,
        Attributes::new(0, 0, 0, 0, 0, 1),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherHelmet,
        EquipmentCategory::Armor,
        &HELMET_SLOT,
        false,
        2,
        Attributes::new(0, 1, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::BronzeRing,
        EquipmentCategory::Accessory,
        &RING_SLOTS,
        false,
        1,
        Attributes::new(1, 0, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::BronzeAmulet,
        EquipmentCategory::Accessory,
        &AMULET_SLOTS,
        false,
        2,
        Attributes::new(0, 0, 2, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::TravelersCloak,
        EquipmentCategory::Back,
        &BACK_SLOT,
        false,
        1,
        Attributes::new(0, 0, 0, 1, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::HuntingQuiver,
        EquipmentCategory::Back,
        &BACK_SLOT,
        false,
        2,
        Attributes::new(0, 0, 0, 0, 1, 0),
    ),
];

pub fn equipment_definition(resource: ResourceKind) -> Option<EquipmentDefinition> {
    for definition in EQUIPMENT_CODEX.iter() {
        if definition.resource == resource {
            return Some(*definition);
        }
    }
    None
}

pub fn equipment_definition_by_index(index: u8) -> Option<EquipmentDefinition> {
    EQUIPMENT_CODEX.get(index as usize).copied()
}

pub fn equipment_definition_index(resource: ResourceKind) -> Option<u8> {
    EQUIPMENT_CODEX
        .iter()
        .position(|definition| definition.resource == resource)
        .map(|idx| idx as u8)
}

#[derive(Clone, Copy)]
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

const CARAVAN_GUARD_REWARDS: [QuestReward; 1] = [QuestReward::resource(ResourceKind::Gold, 100)];
const WOODCUTTING_REWARDS: [QuestReward; 1] = [QuestReward::resource(ResourceKind::Log, 1)];
const BANDIT_CAMP_REWARDS: [QuestReward; 1] = [QuestReward::experience(50)];
const SCOUTING_DRILLS_REWARDS: [QuestReward; 1] = [QuestReward::experience(60)];
const TIMBER_STOCKPILE_REWARDS: [QuestReward; 1] = [QuestReward::resource(ResourceKind::Log, 2)];
const ARTISAN_SUPPORT_REWARDS: [QuestReward; 1] = [QuestReward::resource(ResourceKind::Gold, 150)];

pub const QUEST_CODEX: [QuestCodexEntry; 6] = [
    QuestCodexEntry::new(
        1,
        QuestType::Job,
        "Caravan Guard",
        1,
        60 * 60,
        5,
        CoreStats::new(0, 1, 1),
        &CARAVAN_GUARD_REWARDS,
    ),
    QuestCodexEntry::new(
        2,
        QuestType::Farm,
        "Woodcutting",
        1,
        60 * 60,
        5,
        CoreStats::new(1, 1, 0),
        &WOODCUTTING_REWARDS,
    ),
    QuestCodexEntry::new(
        3,
        QuestType::Raid,
        "Bandit Camp",
        1,
        60 * 60,
        7,
        CoreStats::new(1, 0, 1),
        &BANDIT_CAMP_REWARDS,
    ),
    QuestCodexEntry::new(
        4,
        QuestType::Raid,
        "Scouting Drills",
        1,
        45 * 60,
        6,
        CoreStats::new(0, 1, 0),
        &SCOUTING_DRILLS_REWARDS,
    ),
    QuestCodexEntry::new(
        5,
        QuestType::Farm,
        "Timber Stockpile",
        2,
        75 * 60,
        7,
        CoreStats::new(1, 1, 0),
        &TIMBER_STOCKPILE_REWARDS,
    ),
    QuestCodexEntry::new(
        6,
        QuestType::Job,
        "Artisan Support",
        3,
        80 * 60,
        8,
        CoreStats::new(0, 1, 1),
        &ARTISAN_SUPPORT_REWARDS,
    ),
];

pub fn quest_by_id(id: u16) -> Option<&'static QuestCodexEntry> {
    if id == 0 {
        return None;
    }
    let idx = (id - 1) as usize;
    QUEST_CODEX.get(idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_lookup_returns_expected_entries() {
        assert_eq!(quest_by_id(1).unwrap().name, "Caravan Guard");
        assert_eq!(quest_by_id(2).unwrap().name, "Woodcutting");
        assert_eq!(quest_by_id(3).unwrap().name, "Bandit Camp");
    }

    #[test]
    fn quest_energy_cost_and_requirements_behave_as_expected() {
        let quest = quest_by_id(2).expect("woodcutting quest");
        assert!(quest.stats_satisfied(quest.minimum_stats));
        let exact_cost = quest.energy_cost(quest.minimum_stats);
        assert_eq!(exact_cost, quest.base_energy_cost);

        let low_stats = CoreStats::zero();
        assert!(!quest.stats_satisfied(low_stats));
        assert_eq!(
            quest.energy_cost(low_stats),
            (f32::from(quest.base_energy_cost) * 1.5).ceil() as u16
        );

        let overqualified = CoreStats::new(
            quest.minimum_stats.might + 5,
            quest.minimum_stats.speed + 5,
            quest.minimum_stats.intellect + 5,
        );
        assert_eq!(quest.energy_cost(overqualified), 0);
    }

    #[test]
    fn recipe_energy_cost_matches_quest_logic() {
        let recipe = recipe_by_id(2).expect("wooden sword recipe");
        assert!(recipe.stats_satisfied(recipe.minimum_stats));
        assert_eq!(
            recipe.energy_cost(recipe.minimum_stats),
            recipe.base_energy_cost
        );

        let insufficient = CoreStats::zero();
        assert!(!recipe.stats_satisfied(insufficient));
        assert_eq!(
            recipe.energy_cost(insufficient),
            (f32::from(recipe.base_energy_cost) * 1.5).ceil() as u16
        );

        let generous_stats = CoreStats::new(
            recipe.minimum_stats.might + 10,
            recipe.minimum_stats.speed + 10,
            recipe.minimum_stats.intellect + 10,
        );
        assert_eq!(recipe.energy_cost(generous_stats), 0);
    }

    #[test]
    fn equipment_definition_codex_returns_expected_entries() {
        let definition =
            equipment_definition(ResourceKind::WoodenSword).expect("wooden sword definition");
        assert!(definition.supports_slot(EquipmentSlot::LeftHand));
        assert_eq!(definition.attribute_bonus.atk, 2);
        assert!(equipment_definition(ResourceKind::Gold).is_none());

        let shield = equipment_definition(ResourceKind::WoodenShield).expect("shield definition");
        assert!(shield.supports_slot(EquipmentSlot::LeftHand));
        assert_eq!(shield.resource, ResourceKind::WoodenShield);

        let index = equipment_definition_index(ResourceKind::WoodenShield).expect("shield index");
        let from_index = equipment_definition_by_index(index).expect("shield via index");
        assert_eq!(from_index.resource, ResourceKind::WoodenShield);
    }

    #[test]
    fn resource_mint_keys() {
        use anchor_lang::prelude::Pubkey;

        let log = Pubkey::find_program_address(
            &[crate::RESOURCE_MINT_SEED, ResourceKind::Log.seed_bytes()],
            &crate::ID,
        )
        .0;
        let plank = Pubkey::find_program_address(
            &[crate::RESOURCE_MINT_SEED, ResourceKind::Plank.seed_bytes()],
            &crate::ID,
        )
        .0;
        println!("log mint {}", log);
        println!("plank mint {}", plank);
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RecipeType {
    Forge,
    Craft,
}

#[derive(Clone, Copy)]
pub enum RecipeInput {
    Forge {
        raw_material: ResourceKind,
        amount: u64,
    },
    Craft {
        base_material: ResourceKind,
        base_amount: u64,
        gold_amount: u64,
    },
}

#[derive(Clone, Copy)]
pub enum RecipeOutput {
    Resource { resource: ResourceKind, amount: u64 },
}

pub struct RecipeCodexEntry {
    pub id: u16,
    pub recipe_type: RecipeType,
    pub name: &'static str,
    pub level_required: u8,
    pub cooldown_seconds: u32,
    pub base_energy_cost: u16,
    pub minimum_stats: CoreStats,
    pub input: RecipeInput,
    pub output: RecipeOutput,
}

impl RecipeCodexEntry {
    pub const fn new(
        id: u16,
        recipe_type: RecipeType,
        name: &'static str,
        level_required: u8,
        cooldown_seconds: u32,
        base_energy_cost: u16,
        minimum_stats: CoreStats,
        input: RecipeInput,
        output: RecipeOutput,
    ) -> Self {
        Self {
            id,
            recipe_type,
            name,
            level_required,
            cooldown_seconds,
            base_energy_cost,
            minimum_stats,
            input,
            output,
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

pub const RECIPE_CODEX: [RecipeCodexEntry; 12] = [
    RecipeCodexEntry::new(
        1,
        RecipeType::Forge,
        "Plank Forging",
        1,
        60 * 60,
        5,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::Log,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::Plank,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        2,
        RecipeType::Craft,
        "Wooden Sword",
        1,
        2 * 60 * 60,
        7,
        CoreStats::new(1, 0, 1),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 2,
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodenSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        3,
        RecipeType::Craft,
        "Shield Carpentry",
        2,
        90 * 60,
        6,
        CoreStats::new(1, 0, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 2,
            gold_amount: 25,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodenShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        4,
        RecipeType::Craft,
        "Greatsword Assembly",
        3,
        120 * 60,
        8,
        CoreStats::new(2, 0, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 3,
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::TrainingGreatsword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        5,
        RecipeType::Craft,
        "Leather Armor Weaving",
        3,
        100 * 60,
        7,
        CoreStats::new(1, 0, 1),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 90,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::LeatherArmor,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        6,
        RecipeType::Craft,
        "Leather Leggings",
        2,
        70 * 60,
        6,
        CoreStats::new(0, 1, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 70,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::LeatherLeggings,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        7,
        RecipeType::Craft,
        "Leather Boots",
        2,
        60 * 60,
        6,
        CoreStats::new(0, 1, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::LeatherBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        8,
        RecipeType::Craft,
        "Leather Helmet",
        2,
        60 * 60,
        6,
        CoreStats::new(0, 0, 1),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::LeatherHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        9,
        RecipeType::Craft,
        "Bronze Ring",
        1,
        45 * 60,
        5,
        CoreStats::new(0, 0, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 40,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        10,
        RecipeType::Craft,
        "Bronze Amulet",
        2,
        60 * 60,
        6,
        CoreStats::new(0, 0, 1),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        11,
        RecipeType::Craft,
        "Traveler's Cloak",
        1,
        50 * 60,
        5,
        CoreStats::new(0, 0, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 30,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::TravelersCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        12,
        RecipeType::Craft,
        "Hunting Quiver",
        2,
        60 * 60,
        6,
        CoreStats::new(0, 1, 0),
        RecipeInput::Craft {
            base_material: ResourceKind::Plank,
            base_amount: 1,
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::HuntingQuiver,
            amount: 1,
        },
    ),
];

pub fn recipe_by_id(id: u16) -> Option<&'static RecipeCodexEntry> {
    RECIPE_CODEX.iter().find(|recipe| recipe.id == id)
}
