use anchor_lang::prelude::*;

use super::base::CoreStats;
use super::resources::ResourceKind;

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
        secondary_material: Option<(ResourceKind, u64)>,
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

pub const RECIPE_CODEX: [RecipeCodexEntry; 14] = [
    RecipeCodexEntry::new(
        1,
        RecipeType::Forge,
        "Plank Forging",
        1,
        60 * 60,
        5,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::Wood,
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
            secondary_material: None,
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
            secondary_material: None,
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
            base_amount: 2,
            secondary_material: Some((ResourceKind::BronzeBar, 1)),
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
            base_material: ResourceKind::Leather,
            base_amount: 3,
            secondary_material: None,
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
            base_material: ResourceKind::Leather,
            base_amount: 2,
            secondary_material: None,
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
            base_material: ResourceKind::Leather,
            base_amount: 1,
            secondary_material: None,
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
            base_material: ResourceKind::Leather,
            base_amount: 1,
            secondary_material: None,
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
            base_material: ResourceKind::BronzeBar,
            base_amount: 1,
            secondary_material: None,
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
            base_material: ResourceKind::BronzeBar,
            base_amount: 1,
            secondary_material: None,
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
            base_material: ResourceKind::Leather,
            base_amount: 2,
            secondary_material: None,
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
            base_material: ResourceKind::Leather,
            base_amount: 1,
            secondary_material: Some((ResourceKind::Wood, 1)),
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::HuntingQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        13,
        RecipeType::Forge,
        "Leather Tanning",
        2,
        75 * 60,
        6,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::RawLeather,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::Leather,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        14,
        RecipeType::Forge,
        "Bronze Smelting",
        3,
        90 * 60,
        7,
        CoreStats::new(2, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::BronzeOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeBar,
            amount: 1,
        },
    ),
];

/// Optimized O(1) recipe lookup by ID
#[inline]
pub fn recipe_by_id(id: u16) -> Option<&'static RecipeCodexEntry> {
    if id == 0 {
        return None;
    }
    let idx = (id - 1) as usize;
    RECIPE_CODEX.get(idx)
}
