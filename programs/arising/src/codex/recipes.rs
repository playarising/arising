use anchor_lang::prelude::*;

use super::base::CoreStats;
use super::resources::ResourceKind;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub struct RecipeMaterial {
    pub kind: ResourceKind,
    pub amount: u64,
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
        materials: &'static [RecipeMaterial],
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

pub const RECIPE_CODEX: [RecipeCodexEntry; 616] = [
    RecipeCodexEntry::new(
        1,
        RecipeType::Forge,
        "Plank Forging",
        1,
        3600,
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
        RecipeType::Forge,
        "Leather Tanning",
        2,
        4500,
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
        3,
        RecipeType::Forge,
        "Bronze Refining",
        10,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::BronzeOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeBar,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        4,
        RecipeType::Forge,
        "Iron Refining",
        20,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::IronOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        5,
        RecipeType::Forge,
        "Steel Refining",
        31,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::SteelOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        6,
        RecipeType::Forge,
        "Silver Refining",
        40,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::SilverOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        7,
        RecipeType::Forge,
        "Gold Refining",
        50,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::GoldOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        8,
        RecipeType::Forge,
        "Platinum Refining",
        61,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::PlatinumOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        9,
        RecipeType::Forge,
        "Adamantine Refining",
        70,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::AdamantineOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        10,
        RecipeType::Forge,
        "Cobalt Refining",
        80,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::CobaltOre,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        11,
        RecipeType::Forge,
        "Angelic Refining",
        91,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::AngelicEssence,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        12,
        RecipeType::Forge,
        "Crimson Refining",
        105,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::CrimsonEssence,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        13,
        RecipeType::Forge,
        "Fateful Refining",
        121,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::FatefulEssence,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        14,
        RecipeType::Forge,
        "Altair Refining",
        135,
        5400,
        8,
        CoreStats::new(1, 0, 0),
        RecipeInput::Forge {
            raw_material: ResourceKind::AltairEssence,
            amount: 1,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairIngot,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        15,
        RecipeType::Craft,
        "Wood BigShield",
        1,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        16,
        RecipeType::Craft,
        "Wood SmallShield",
        1,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        17,
        RecipeType::Craft,
        "Wood Sword",
        1,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        18,
        RecipeType::Craft,
        "Wood Dagger",
        2,
        7200,
        11,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        19,
        RecipeType::Craft,
        "Wood Bow",
        2,
        7200,
        11,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        20,
        RecipeType::Craft,
        "Wood Crossbow",
        3,
        7200,
        12,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 70,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        21,
        RecipeType::Craft,
        "Wood Spear",
        3,
        7200,
        12,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 70,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        22,
        RecipeType::Craft,
        "Wood Lance",
        4,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        23,
        RecipeType::Craft,
        "Wood Halberd",
        4,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        24,
        RecipeType::Craft,
        "Wood Rod",
        5,
        7200,
        14,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 90,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        25,
        RecipeType::Craft,
        "Wood Staff",
        5,
        7200,
        14,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 90,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        26,
        RecipeType::Craft,
        "Wood Rifle",
        6,
        7200,
        15,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 100,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        27,
        RecipeType::Craft,
        "Wood Trident",
        6,
        7200,
        15,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 100,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        28,
        RecipeType::Craft,
        "Wood Warhammer",
        7,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 110,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        29,
        RecipeType::Craft,
        "Wood Quiver",
        7,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 110,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::WoodQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        30,
        RecipeType::Craft,
        "Cooper Chest",
        1,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        31,
        RecipeType::Craft,
        "Cooper Legs",
        2,
        7200,
        11,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        32,
        RecipeType::Craft,
        "Cooper Boots",
        2,
        7200,
        11,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        33,
        RecipeType::Craft,
        "Cooper Helm",
        2,
        7200,
        11,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        34,
        RecipeType::Craft,
        "Cooper Ring",
        1,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 50,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        35,
        RecipeType::Craft,
        "Cooper Amulet",
        2,
        7200,
        11,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        36,
        RecipeType::Craft,
        "Cooper Cloak",
        3,
        7200,
        12,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 70,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        37,
        RecipeType::Craft,
        "Bronze BigShield",
        10,
        7200,
        10,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 65,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        38,
        RecipeType::Craft,
        "Bronze SmallShield",
        10,
        7200,
        10,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 65,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        39,
        RecipeType::Craft,
        "Bronze Sword",
        10,
        7200,
        10,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 65,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        40,
        RecipeType::Craft,
        "Bronze Dagger",
        11,
        7200,
        11,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 78,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        41,
        RecipeType::Craft,
        "Bronze Bow",
        11,
        7200,
        11,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 78,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        42,
        RecipeType::Craft,
        "Bronze Crossbow",
        12,
        7200,
        12,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 91,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        43,
        RecipeType::Craft,
        "Bronze Spear",
        12,
        7200,
        12,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 91,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        44,
        RecipeType::Craft,
        "Bronze Lance",
        13,
        7200,
        13,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 104,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        45,
        RecipeType::Craft,
        "Bronze Halberd",
        13,
        7200,
        13,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 104,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        46,
        RecipeType::Craft,
        "Bronze Rod",
        14,
        7200,
        14,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 117,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        47,
        RecipeType::Craft,
        "Bronze Staff",
        14,
        7200,
        14,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 117,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        48,
        RecipeType::Craft,
        "Bronze Rifle",
        15,
        7200,
        15,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        49,
        RecipeType::Craft,
        "Bronze Trident",
        15,
        7200,
        15,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        50,
        RecipeType::Craft,
        "Bronze Warhammer",
        16,
        7200,
        16,
        CoreStats::new(3, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 143,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::BronzeWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        51,
        RecipeType::Craft,
        "Nova Chest",
        113,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 425,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        52,
        RecipeType::Craft,
        "Nova Legs",
        114,
        7200,
        11,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        53,
        RecipeType::Craft,
        "Nova Boots",
        114,
        7200,
        11,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        54,
        RecipeType::Craft,
        "Nova Helm",
        114,
        7200,
        11,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        55,
        RecipeType::Craft,
        "Nova Ring",
        113,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 425,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        56,
        RecipeType::Craft,
        "Nova Amulet",
        114,
        7200,
        11,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        57,
        RecipeType::Craft,
        "Nova Cloak",
        115,
        7200,
        12,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 595,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        58,
        RecipeType::Craft,
        "Iron BigShield",
        20,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        59,
        RecipeType::Craft,
        "Iron SmallShield",
        20,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        60,
        RecipeType::Craft,
        "Iron Sword",
        20,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        61,
        RecipeType::Craft,
        "Iron Dagger",
        21,
        7200,
        11,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 96,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        62,
        RecipeType::Craft,
        "Iron Bow",
        21,
        7200,
        11,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 96,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        63,
        RecipeType::Craft,
        "Iron Crossbow",
        22,
        7200,
        12,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 112,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        64,
        RecipeType::Craft,
        "Iron Spear",
        22,
        7200,
        12,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 112,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        65,
        RecipeType::Craft,
        "Iron Lance",
        23,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 128,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        66,
        RecipeType::Craft,
        "Iron Halberd",
        23,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 128,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        67,
        RecipeType::Craft,
        "Iron Rod",
        24,
        7200,
        14,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 144,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        68,
        RecipeType::Craft,
        "Iron Staff",
        24,
        7200,
        14,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 144,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        69,
        RecipeType::Craft,
        "Iron Rifle",
        25,
        7200,
        15,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        70,
        RecipeType::Craft,
        "Iron Trident",
        25,
        7200,
        15,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        71,
        RecipeType::Craft,
        "Iron Warhammer",
        26,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 176,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        72,
        RecipeType::Craft,
        "Iron Chest",
        20,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        73,
        RecipeType::Craft,
        "Iron Legs",
        21,
        7200,
        11,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 96,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        74,
        RecipeType::Craft,
        "Iron Boots",
        21,
        7200,
        11,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 96,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        75,
        RecipeType::Craft,
        "Iron Helm",
        21,
        7200,
        11,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 96,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        76,
        RecipeType::Craft,
        "Iron Ring",
        20,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 80,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        77,
        RecipeType::Craft,
        "Iron Amulet",
        21,
        7200,
        11,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 96,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        78,
        RecipeType::Craft,
        "Iron Cloak",
        22,
        7200,
        12,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 112,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        79,
        RecipeType::Craft,
        "Steel BigShield",
        31,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 105,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        80,
        RecipeType::Craft,
        "Steel SmallShield",
        31,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 105,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        81,
        RecipeType::Craft,
        "Steel Sword",
        31,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 105,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        82,
        RecipeType::Craft,
        "Steel Dagger",
        32,
        7200,
        11,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 126,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        83,
        RecipeType::Craft,
        "Steel Bow",
        32,
        7200,
        11,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 126,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        84,
        RecipeType::Craft,
        "Steel Crossbow",
        33,
        7200,
        12,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 147,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        85,
        RecipeType::Craft,
        "Steel Spear",
        33,
        7200,
        12,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 147,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        86,
        RecipeType::Craft,
        "Steel Lance",
        34,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 168,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        87,
        RecipeType::Craft,
        "Steel Halberd",
        34,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 168,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        88,
        RecipeType::Craft,
        "Steel Rod",
        35,
        7200,
        14,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 189,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        89,
        RecipeType::Craft,
        "Steel Staff",
        35,
        7200,
        14,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 189,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        90,
        RecipeType::Craft,
        "Steel Rifle",
        36,
        7200,
        15,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        91,
        RecipeType::Craft,
        "Steel Trident",
        36,
        7200,
        15,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        92,
        RecipeType::Craft,
        "Steel Warhammer",
        37,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 231,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        93,
        RecipeType::Craft,
        "Steel Chest",
        31,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 105,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        94,
        RecipeType::Craft,
        "Steel Legs",
        32,
        7200,
        11,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 126,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        95,
        RecipeType::Craft,
        "Steel Boots",
        32,
        7200,
        11,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 126,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        96,
        RecipeType::Craft,
        "Steel Helm",
        32,
        7200,
        11,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 126,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        97,
        RecipeType::Craft,
        "Steel Ring",
        31,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 105,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        98,
        RecipeType::Craft,
        "Steel Amulet",
        32,
        7200,
        11,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 126,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        99,
        RecipeType::Craft,
        "Steel Cloak",
        33,
        7200,
        12,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 147,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        100,
        RecipeType::Craft,
        "Silver BigShield",
        40,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        101,
        RecipeType::Craft,
        "Silver SmallShield",
        40,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        102,
        RecipeType::Craft,
        "Silver Sword",
        40,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        103,
        RecipeType::Craft,
        "Silver Dagger",
        41,
        7200,
        11,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 156,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        104,
        RecipeType::Craft,
        "Silver Bow",
        41,
        7200,
        11,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 156,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        105,
        RecipeType::Craft,
        "Silver Crossbow",
        42,
        7200,
        12,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 182,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        106,
        RecipeType::Craft,
        "Silver Spear",
        42,
        7200,
        12,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 182,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        107,
        RecipeType::Craft,
        "Silver Lance",
        43,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 208,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        108,
        RecipeType::Craft,
        "Silver Halberd",
        43,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 208,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        109,
        RecipeType::Craft,
        "Silver Rod",
        44,
        7200,
        14,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        110,
        RecipeType::Craft,
        "Silver Staff",
        44,
        7200,
        14,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        111,
        RecipeType::Craft,
        "Silver Rifle",
        45,
        7200,
        15,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 260,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        112,
        RecipeType::Craft,
        "Silver Trident",
        45,
        7200,
        15,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 260,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        113,
        RecipeType::Craft,
        "Silver Warhammer",
        46,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 286,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        114,
        RecipeType::Craft,
        "Silver Chest",
        40,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        115,
        RecipeType::Craft,
        "Silver Legs",
        41,
        7200,
        11,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 156,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        116,
        RecipeType::Craft,
        "Silver Boots",
        41,
        7200,
        11,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 156,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        117,
        RecipeType::Craft,
        "Silver Helm",
        41,
        7200,
        11,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 156,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        118,
        RecipeType::Craft,
        "Silver Ring",
        40,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 130,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        119,
        RecipeType::Craft,
        "Silver Amulet",
        41,
        7200,
        11,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 156,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        120,
        RecipeType::Craft,
        "Silver Cloak",
        42,
        7200,
        12,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 182,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        121,
        RecipeType::Craft,
        "Gold BigShield",
        50,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        122,
        RecipeType::Craft,
        "Gold SmallShield",
        50,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        123,
        RecipeType::Craft,
        "Gold Sword",
        50,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        124,
        RecipeType::Craft,
        "Gold Dagger",
        51,
        7200,
        11,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 192,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        125,
        RecipeType::Craft,
        "Gold Bow",
        51,
        7200,
        11,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 192,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        126,
        RecipeType::Craft,
        "Gold Crossbow",
        52,
        7200,
        12,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 224,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        127,
        RecipeType::Craft,
        "Gold Spear",
        52,
        7200,
        12,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 224,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        128,
        RecipeType::Craft,
        "Gold Lance",
        53,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 256,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        129,
        RecipeType::Craft,
        "Gold Halberd",
        53,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 256,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        130,
        RecipeType::Craft,
        "Gold Rod",
        54,
        7200,
        14,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 288,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        131,
        RecipeType::Craft,
        "Gold Staff",
        54,
        7200,
        14,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 288,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        132,
        RecipeType::Craft,
        "Gold Rifle",
        55,
        7200,
        15,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 320,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        133,
        RecipeType::Craft,
        "Gold Trident",
        55,
        7200,
        15,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 320,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        134,
        RecipeType::Craft,
        "Gold Warhammer",
        56,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 352,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        135,
        RecipeType::Craft,
        "Gold Quiver",
        56,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 352,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        136,
        RecipeType::Craft,
        "Gold Chest",
        50,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        137,
        RecipeType::Craft,
        "Gold Legs",
        51,
        7200,
        11,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 192,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        138,
        RecipeType::Craft,
        "Gold Boots",
        51,
        7200,
        11,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 192,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        139,
        RecipeType::Craft,
        "Gold Helm",
        51,
        7200,
        11,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 192,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        140,
        RecipeType::Craft,
        "Gold Ring",
        50,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 160,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        141,
        RecipeType::Craft,
        "Gold Amulet",
        51,
        7200,
        11,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 192,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        142,
        RecipeType::Craft,
        "Gold Cloak",
        52,
        7200,
        12,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 224,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        143,
        RecipeType::Craft,
        "Platinum BigShield",
        61,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 195,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        144,
        RecipeType::Craft,
        "Platinum SmallShield",
        61,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 195,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        145,
        RecipeType::Craft,
        "Platinum Sword",
        61,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 195,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        146,
        RecipeType::Craft,
        "Platinum Dagger",
        62,
        7200,
        11,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        147,
        RecipeType::Craft,
        "Platinum Bow",
        62,
        7200,
        11,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        148,
        RecipeType::Craft,
        "Platinum Crossbow",
        63,
        7200,
        12,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 273,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        149,
        RecipeType::Craft,
        "Platinum Spear",
        63,
        7200,
        12,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 273,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        150,
        RecipeType::Craft,
        "Platinum Lance",
        64,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 312,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        151,
        RecipeType::Craft,
        "Platinum Halberd",
        64,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 312,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        152,
        RecipeType::Craft,
        "Platinum Rod",
        65,
        7200,
        14,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 351,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        153,
        RecipeType::Craft,
        "Platinum Staff",
        65,
        7200,
        14,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 351,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        154,
        RecipeType::Craft,
        "Platinum Rifle",
        66,
        7200,
        15,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        155,
        RecipeType::Craft,
        "Platinum Trident",
        66,
        7200,
        15,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        156,
        RecipeType::Craft,
        "Platinum Warhammer",
        67,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 429,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        157,
        RecipeType::Craft,
        "Platinum Quiver",
        67,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 429,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        158,
        RecipeType::Craft,
        "Platinum Chest",
        61,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 195,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        159,
        RecipeType::Craft,
        "Platinum Legs",
        62,
        7200,
        11,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        160,
        RecipeType::Craft,
        "Platinum Boots",
        62,
        7200,
        11,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        161,
        RecipeType::Craft,
        "Platinum Helm",
        62,
        7200,
        11,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        162,
        RecipeType::Craft,
        "Platinum Ring",
        61,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 195,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        163,
        RecipeType::Craft,
        "Platinum Amulet",
        62,
        7200,
        11,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 234,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        164,
        RecipeType::Craft,
        "Platinum Cloak",
        63,
        7200,
        12,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 273,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        165,
        RecipeType::Craft,
        "Adamantine BigShield",
        70,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 235,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        166,
        RecipeType::Craft,
        "Adamantine SmallShield",
        70,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 235,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        167,
        RecipeType::Craft,
        "Adamantine Sword",
        70,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 235,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        168,
        RecipeType::Craft,
        "Adamantine Dagger",
        71,
        7200,
        11,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 282,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        169,
        RecipeType::Craft,
        "Adamantine Bow",
        71,
        7200,
        11,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 282,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        170,
        RecipeType::Craft,
        "Adamantine Crossbow",
        72,
        7200,
        12,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 329,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        171,
        RecipeType::Craft,
        "Adamantine Spear",
        72,
        7200,
        12,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 329,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        172,
        RecipeType::Craft,
        "Adamantine Lance",
        73,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 376,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        173,
        RecipeType::Craft,
        "Adamantine Halberd",
        73,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 376,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        174,
        RecipeType::Craft,
        "Adamantine Rod",
        74,
        7200,
        14,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 423,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        175,
        RecipeType::Craft,
        "Adamantine Staff",
        74,
        7200,
        14,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 423,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        176,
        RecipeType::Craft,
        "Adamantine Rifle",
        75,
        7200,
        15,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 470,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        177,
        RecipeType::Craft,
        "Adamantine Trident",
        75,
        7200,
        15,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 470,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        178,
        RecipeType::Craft,
        "Adamantine Warhammer",
        76,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 517,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        179,
        RecipeType::Craft,
        "Adamantine Quiver",
        76,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 517,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        180,
        RecipeType::Craft,
        "Adamantine Chest",
        70,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 235,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        181,
        RecipeType::Craft,
        "Adamantine Legs",
        71,
        7200,
        11,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 282,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        182,
        RecipeType::Craft,
        "Adamantine Boots",
        71,
        7200,
        11,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 282,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        183,
        RecipeType::Craft,
        "Adamantine Helm",
        71,
        7200,
        11,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 282,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        184,
        RecipeType::Craft,
        "Adamantine Ring",
        70,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 235,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        185,
        RecipeType::Craft,
        "Adamantine Amulet",
        71,
        7200,
        11,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 282,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        186,
        RecipeType::Craft,
        "Adamantine Cloak",
        72,
        7200,
        12,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 329,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        187,
        RecipeType::Craft,
        "Cobalt BigShield",
        80,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 280,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        188,
        RecipeType::Craft,
        "Cobalt SmallShield",
        80,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 280,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        189,
        RecipeType::Craft,
        "Cobalt Sword",
        80,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 280,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        190,
        RecipeType::Craft,
        "Cobalt Dagger",
        81,
        7200,
        11,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 336,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        191,
        RecipeType::Craft,
        "Cobalt Bow",
        81,
        7200,
        11,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 336,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        192,
        RecipeType::Craft,
        "Cobalt Crossbow",
        82,
        7200,
        12,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 392,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        193,
        RecipeType::Craft,
        "Cobalt Spear",
        82,
        7200,
        12,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 392,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        194,
        RecipeType::Craft,
        "Cobalt Lance",
        83,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 448,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        195,
        RecipeType::Craft,
        "Cobalt Halberd",
        83,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 448,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        196,
        RecipeType::Craft,
        "Cobalt Rod",
        84,
        7200,
        14,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 504,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        197,
        RecipeType::Craft,
        "Cobalt Staff",
        84,
        7200,
        14,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 504,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        198,
        RecipeType::Craft,
        "Cobalt Rifle",
        85,
        7200,
        15,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 560,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        199,
        RecipeType::Craft,
        "Cobalt Trident",
        85,
        7200,
        15,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 560,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        200,
        RecipeType::Craft,
        "Cobalt Warhammer",
        86,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 616,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        201,
        RecipeType::Craft,
        "Cobalt Quiver",
        86,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 616,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        202,
        RecipeType::Craft,
        "Cobalt Chest",
        80,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 280,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        203,
        RecipeType::Craft,
        "Cobalt Legs",
        81,
        7200,
        11,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 336,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        204,
        RecipeType::Craft,
        "Cobalt Boots",
        81,
        7200,
        11,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 336,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        205,
        RecipeType::Craft,
        "Cobalt Helm",
        81,
        7200,
        11,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 336,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        206,
        RecipeType::Craft,
        "Cobalt Ring",
        80,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 280,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        207,
        RecipeType::Craft,
        "Cobalt Amulet",
        81,
        7200,
        11,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 336,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        208,
        RecipeType::Craft,
        "Cobalt Cloak",
        82,
        7200,
        12,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 392,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        209,
        RecipeType::Craft,
        "Angelic BigShield",
        91,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 330,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        210,
        RecipeType::Craft,
        "Angelic SmallShield",
        91,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 330,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        211,
        RecipeType::Craft,
        "Angelic Sword",
        91,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 330,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        212,
        RecipeType::Craft,
        "Angelic Dagger",
        92,
        7200,
        11,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 396,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        213,
        RecipeType::Craft,
        "Angelic Bow",
        92,
        7200,
        11,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 396,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        214,
        RecipeType::Craft,
        "Angelic Crossbow",
        93,
        7200,
        12,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 462,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        215,
        RecipeType::Craft,
        "Angelic Spear",
        93,
        7200,
        12,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 462,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        216,
        RecipeType::Craft,
        "Angelic Lance",
        94,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 528,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        217,
        RecipeType::Craft,
        "Angelic Halberd",
        94,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 528,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        218,
        RecipeType::Craft,
        "Angelic Rod",
        95,
        7200,
        14,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 594,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        219,
        RecipeType::Craft,
        "Angelic Staff",
        95,
        7200,
        14,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 594,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        220,
        RecipeType::Craft,
        "Angelic Rifle",
        96,
        7200,
        15,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        221,
        RecipeType::Craft,
        "Angelic Trident",
        96,
        7200,
        15,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        222,
        RecipeType::Craft,
        "Angelic Warhammer",
        97,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 726,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        223,
        RecipeType::Craft,
        "Angelic Chest",
        91,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 330,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        224,
        RecipeType::Craft,
        "Angelic Legs",
        92,
        7200,
        11,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 396,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        225,
        RecipeType::Craft,
        "Angelic Boots",
        92,
        7200,
        11,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 396,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        226,
        RecipeType::Craft,
        "Angelic Helm",
        92,
        7200,
        11,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 396,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        227,
        RecipeType::Craft,
        "Angelic Ring",
        91,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 330,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        228,
        RecipeType::Craft,
        "Angelic Amulet",
        92,
        7200,
        11,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 396,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        229,
        RecipeType::Craft,
        "Angelic Cloak",
        93,
        7200,
        12,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 462,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        230,
        RecipeType::Craft,
        "Crimson BigShield",
        105,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        231,
        RecipeType::Craft,
        "Crimson SmallShield",
        105,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        232,
        RecipeType::Craft,
        "Crimson Sword",
        105,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        233,
        RecipeType::Craft,
        "Crimson Dagger",
        106,
        7200,
        11,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 468,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        234,
        RecipeType::Craft,
        "Crimson Bow",
        106,
        7200,
        11,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 468,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        235,
        RecipeType::Craft,
        "Crimson Crossbow",
        107,
        7200,
        12,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 546,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        236,
        RecipeType::Craft,
        "Crimson Spear",
        107,
        7200,
        12,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 546,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        237,
        RecipeType::Craft,
        "Crimson Lance",
        108,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 624,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        238,
        RecipeType::Craft,
        "Crimson Halberd",
        108,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 624,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        239,
        RecipeType::Craft,
        "Crimson Rod",
        109,
        7200,
        14,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 702,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        240,
        RecipeType::Craft,
        "Crimson Staff",
        109,
        7200,
        14,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 702,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        241,
        RecipeType::Craft,
        "Crimson Rifle",
        110,
        7200,
        15,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 780,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        242,
        RecipeType::Craft,
        "Crimson Trident",
        110,
        7200,
        15,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 780,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        243,
        RecipeType::Craft,
        "Crimson Warhammer",
        111,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 858,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        244,
        RecipeType::Craft,
        "Crimson Quiver",
        111,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 858,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        245,
        RecipeType::Craft,
        "Crimson Chest",
        105,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        246,
        RecipeType::Craft,
        "Crimson Legs",
        106,
        7200,
        11,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 468,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        247,
        RecipeType::Craft,
        "Crimson Boots",
        106,
        7200,
        11,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 468,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        248,
        RecipeType::Craft,
        "Crimson Helm",
        106,
        7200,
        11,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 468,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        249,
        RecipeType::Craft,
        "Crimson Ring",
        105,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 390,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        250,
        RecipeType::Craft,
        "Crimson Amulet",
        106,
        7200,
        11,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 468,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        251,
        RecipeType::Craft,
        "Crimson Cloak",
        107,
        7200,
        12,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 546,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        252,
        RecipeType::Craft,
        "Fateful BigShield",
        121,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 460,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        253,
        RecipeType::Craft,
        "Fateful SmallShield",
        121,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 460,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        254,
        RecipeType::Craft,
        "Fateful Sword",
        121,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 460,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        255,
        RecipeType::Craft,
        "Fateful Dagger",
        122,
        7200,
        11,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 552,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        256,
        RecipeType::Craft,
        "Fateful Bow",
        122,
        7200,
        11,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 552,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        257,
        RecipeType::Craft,
        "Fateful Crossbow",
        123,
        7200,
        12,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 644,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        258,
        RecipeType::Craft,
        "Fateful Spear",
        123,
        7200,
        12,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 644,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulSpear,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        259,
        RecipeType::Craft,
        "Fateful Lance",
        124,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 736,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulLance,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        260,
        RecipeType::Craft,
        "Fateful Halberd",
        124,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 736,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        261,
        RecipeType::Craft,
        "Fateful Rod",
        125,
        7200,
        14,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 828,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        262,
        RecipeType::Craft,
        "Fateful Staff",
        125,
        7200,
        14,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 828,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        263,
        RecipeType::Craft,
        "Fateful Rifle",
        126,
        7200,
        15,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 920,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        264,
        RecipeType::Craft,
        "Fateful Trident",
        126,
        7200,
        15,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 920,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        265,
        RecipeType::Craft,
        "Fateful Warhammer",
        127,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 1012,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        266,
        RecipeType::Craft,
        "Fateful Quiver",
        127,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 1012,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulQuiver,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        267,
        RecipeType::Craft,
        "Fateful Chest",
        121,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 460,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        268,
        RecipeType::Craft,
        "Fateful Legs",
        122,
        7200,
        11,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 552,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        269,
        RecipeType::Craft,
        "Fateful Boots",
        122,
        7200,
        11,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 552,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        270,
        RecipeType::Craft,
        "Fateful Helm",
        122,
        7200,
        11,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 552,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        271,
        RecipeType::Craft,
        "Fateful Ring",
        121,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 460,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        272,
        RecipeType::Craft,
        "Fateful Amulet",
        122,
        7200,
        11,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 552,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        273,
        RecipeType::Craft,
        "Fateful Cloak",
        123,
        7200,
        12,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 644,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        274,
        RecipeType::Craft,
        "Altair BigShield",
        135,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 550,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairBigShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        275,
        RecipeType::Craft,
        "Altair SmallShield",
        135,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 550,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairSmallShield,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        276,
        RecipeType::Craft,
        "Altair Sword",
        135,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 550,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairSword,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        277,
        RecipeType::Craft,
        "Altair Dagger",
        136,
        7200,
        11,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairDagger,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        278,
        RecipeType::Craft,
        "Altair Bow",
        136,
        7200,
        11,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairBow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        279,
        RecipeType::Craft,
        "Altair Crossbow",
        137,
        7200,
        12,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 770,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairCrossbow,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        280,
        RecipeType::Craft,
        "Altair Halberd",
        138,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 880,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairHalberd,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        281,
        RecipeType::Craft,
        "Altair Rod",
        139,
        7200,
        14,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 990,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairRod,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        282,
        RecipeType::Craft,
        "Altair Staff",
        139,
        7200,
        14,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 990,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairStaff,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        283,
        RecipeType::Craft,
        "Altair Rifle",
        140,
        7200,
        15,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 1100,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairRifle,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        284,
        RecipeType::Craft,
        "Altair Trident",
        140,
        7200,
        15,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 1100,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairTrident,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        285,
        RecipeType::Craft,
        "Altair Warhammer",
        141,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 5,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 1210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairWarhammer,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        286,
        RecipeType::Craft,
        "Altair Chest",
        135,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 550,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairChest,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        287,
        RecipeType::Craft,
        "Altair Legs",
        136,
        7200,
        11,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        288,
        RecipeType::Craft,
        "Altair Boots",
        136,
        7200,
        11,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairBoots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        289,
        RecipeType::Craft,
        "Altair Helm",
        136,
        7200,
        11,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairHelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        290,
        RecipeType::Craft,
        "Altair Ring",
        135,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 550,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairRing,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        291,
        RecipeType::Craft,
        "Altair Amulet",
        136,
        7200,
        11,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairAmulet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        292,
        RecipeType::Craft,
        "Altair Cloak",
        137,
        7200,
        12,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 770,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairCloak,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        293,
        RecipeType::Craft,
        "Cooper Battleplate",
        1,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 6,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 55,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        294,
        RecipeType::Craft,
        "Cooper Chainmail",
        1,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 55,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        295,
        RecipeType::Craft,
        "Cooper Chestplate",
        1,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 55,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        296,
        RecipeType::Craft,
        "Cooper Shirt",
        1,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 55,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        297,
        RecipeType::Craft,
        "Cooper Shoes",
        2,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        298,
        RecipeType::Craft,
        "Cooper Stompers",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        299,
        RecipeType::Craft,
        "Cooper Warboots",
        2,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        300,
        RecipeType::Craft,
        "Cooper Bracelets",
        2,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        301,
        RecipeType::Craft,
        "Cooper Gauntlets",
        2,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        302,
        RecipeType::Craft,
        "Cooper Gloves",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        303,
        RecipeType::Craft,
        "Cooper Handguards",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        304,
        RecipeType::Craft,
        "Cooper Vambraces",
        2,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        305,
        RecipeType::Craft,
        "Cooper Warfist",
        2,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        306,
        RecipeType::Craft,
        "Cooper Cap",
        2,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        307,
        RecipeType::Craft,
        "Cooper ChainCoif",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        308,
        RecipeType::Craft,
        "Cooper Coif",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        309,
        RecipeType::Craft,
        "Cooper Greathelm",
        2,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        310,
        RecipeType::Craft,
        "Cooper Helmet",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        311,
        RecipeType::Craft,
        "Cooper ChainLegs",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        312,
        RecipeType::Craft,
        "Cooper Greaves",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        313,
        RecipeType::Craft,
        "Cooper Legguards",
        2,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        314,
        RecipeType::Craft,
        "Cooper Legplates",
        2,
        7200,
        16,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        315,
        RecipeType::Craft,
        "Cooper Pants",
        2,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        316,
        RecipeType::Craft,
        "Cooper Plateskirt",
        2,
        7200,
        13,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        317,
        RecipeType::Craft,
        "Cooper Skirt",
        2,
        7200,
        10,
        CoreStats::new(2, 2, 1),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 60,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CooperSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        318,
        RecipeType::Craft,
        "Iron Battleplate",
        20,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 150,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        319,
        RecipeType::Craft,
        "Iron Chainmail",
        20,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 150,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        320,
        RecipeType::Craft,
        "Iron Chestplate",
        20,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 150,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        321,
        RecipeType::Craft,
        "Iron Shirt",
        20,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 150,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        322,
        RecipeType::Craft,
        "Iron Shoes",
        21,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        323,
        RecipeType::Craft,
        "Iron Stompers",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        324,
        RecipeType::Craft,
        "Iron Warboots",
        21,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        325,
        RecipeType::Craft,
        "Iron Bracelets",
        21,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        326,
        RecipeType::Craft,
        "Iron Gauntlets",
        21,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        327,
        RecipeType::Craft,
        "Iron Gloves",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        328,
        RecipeType::Craft,
        "Iron Handguards",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        329,
        RecipeType::Craft,
        "Iron Vambraces",
        21,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        330,
        RecipeType::Craft,
        "Iron Warfist",
        21,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        331,
        RecipeType::Craft,
        "Iron Cap",
        21,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        332,
        RecipeType::Craft,
        "Iron ChainCoif",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        333,
        RecipeType::Craft,
        "Iron Coif",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        334,
        RecipeType::Craft,
        "Iron Greathelm",
        21,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        335,
        RecipeType::Craft,
        "Iron Helmet",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        336,
        RecipeType::Craft,
        "Iron ChainLegs",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        337,
        RecipeType::Craft,
        "Iron Greaves",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        338,
        RecipeType::Craft,
        "Iron Legguards",
        21,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        339,
        RecipeType::Craft,
        "Iron Legplates",
        21,
        7200,
        16,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        340,
        RecipeType::Craft,
        "Iron Pants",
        21,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        341,
        RecipeType::Craft,
        "Iron Plateskirt",
        21,
        7200,
        13,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        342,
        RecipeType::Craft,
        "Iron Skirt",
        21,
        7200,
        10,
        CoreStats::new(4, 3, 2),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::BronzeBar,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 155,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::IronSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        343,
        RecipeType::Craft,
        "Steel Battleplate",
        31,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 205,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        344,
        RecipeType::Craft,
        "Steel Chainmail",
        31,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 205,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        345,
        RecipeType::Craft,
        "Steel Chestplate",
        31,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 205,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        346,
        RecipeType::Craft,
        "Steel Shirt",
        31,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 205,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        347,
        RecipeType::Craft,
        "Steel Shoes",
        32,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        348,
        RecipeType::Craft,
        "Steel Stompers",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        349,
        RecipeType::Craft,
        "Steel Warboots",
        32,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        350,
        RecipeType::Craft,
        "Steel Bracelets",
        32,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        351,
        RecipeType::Craft,
        "Steel Gauntlets",
        32,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        352,
        RecipeType::Craft,
        "Steel Gloves",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        353,
        RecipeType::Craft,
        "Steel Handguards",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        354,
        RecipeType::Craft,
        "Steel Vambraces",
        32,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        355,
        RecipeType::Craft,
        "Steel Warfist",
        32,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        356,
        RecipeType::Craft,
        "Steel Cap",
        32,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        357,
        RecipeType::Craft,
        "Steel ChainCoif",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        358,
        RecipeType::Craft,
        "Steel Coif",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        359,
        RecipeType::Craft,
        "Steel Greathelm",
        32,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        360,
        RecipeType::Craft,
        "Steel Helmet",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        361,
        RecipeType::Craft,
        "Steel ChainLegs",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        362,
        RecipeType::Craft,
        "Steel Greaves",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        363,
        RecipeType::Craft,
        "Steel Legguards",
        32,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        364,
        RecipeType::Craft,
        "Steel Legplates",
        32,
        7200,
        16,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        365,
        RecipeType::Craft,
        "Steel Pants",
        32,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        366,
        RecipeType::Craft,
        "Steel Plateskirt",
        32,
        7200,
        13,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        367,
        RecipeType::Craft,
        "Steel Skirt",
        32,
        7200,
        10,
        CoreStats::new(5, 4, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::IronIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 210,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SteelSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        368,
        RecipeType::Craft,
        "Silver Battleplate",
        40,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 250,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        369,
        RecipeType::Craft,
        "Silver Chainmail",
        40,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 250,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        370,
        RecipeType::Craft,
        "Silver Chestplate",
        40,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 250,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        371,
        RecipeType::Craft,
        "Silver Shirt",
        40,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 250,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        372,
        RecipeType::Craft,
        "Silver Shoes",
        41,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        373,
        RecipeType::Craft,
        "Silver Stompers",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        374,
        RecipeType::Craft,
        "Silver Warboots",
        41,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        375,
        RecipeType::Craft,
        "Silver Bracelets",
        41,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        376,
        RecipeType::Craft,
        "Silver Gauntlets",
        41,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        377,
        RecipeType::Craft,
        "Silver Gloves",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        378,
        RecipeType::Craft,
        "Silver Handguards",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        379,
        RecipeType::Craft,
        "Silver Vambraces",
        41,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        380,
        RecipeType::Craft,
        "Silver Warfist",
        41,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        381,
        RecipeType::Craft,
        "Silver Cap",
        41,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        382,
        RecipeType::Craft,
        "Silver ChainCoif",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        383,
        RecipeType::Craft,
        "Silver Coif",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        384,
        RecipeType::Craft,
        "Silver Greathelm",
        41,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        385,
        RecipeType::Craft,
        "Silver Helmet",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        386,
        RecipeType::Craft,
        "Silver ChainLegs",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        387,
        RecipeType::Craft,
        "Silver Greaves",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        388,
        RecipeType::Craft,
        "Silver Legguards",
        41,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        389,
        RecipeType::Craft,
        "Silver Legplates",
        41,
        7200,
        16,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        390,
        RecipeType::Craft,
        "Silver Pants",
        41,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        391,
        RecipeType::Craft,
        "Silver Plateskirt",
        41,
        7200,
        13,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        392,
        RecipeType::Craft,
        "Silver Skirt",
        41,
        7200,
        10,
        CoreStats::new(6, 5, 3),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SteelIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 255,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::SilverSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        393,
        RecipeType::Craft,
        "Gold Battleplate",
        50,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 300,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        394,
        RecipeType::Craft,
        "Gold Chainmail",
        50,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 300,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        395,
        RecipeType::Craft,
        "Gold Chestplate",
        50,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 300,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        396,
        RecipeType::Craft,
        "Gold Shirt",
        50,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 300,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        397,
        RecipeType::Craft,
        "Gold Shoes",
        51,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        398,
        RecipeType::Craft,
        "Gold Stompers",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        399,
        RecipeType::Craft,
        "Gold Warboots",
        51,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        400,
        RecipeType::Craft,
        "Gold Bracelets",
        51,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        401,
        RecipeType::Craft,
        "Gold Gauntlets",
        51,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        402,
        RecipeType::Craft,
        "Gold Gloves",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        403,
        RecipeType::Craft,
        "Gold Handguards",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        404,
        RecipeType::Craft,
        "Gold Vambraces",
        51,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        405,
        RecipeType::Craft,
        "Gold Warfist",
        51,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        406,
        RecipeType::Craft,
        "Gold Cap",
        51,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        407,
        RecipeType::Craft,
        "Gold ChainCoif",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        408,
        RecipeType::Craft,
        "Gold Coif",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        409,
        RecipeType::Craft,
        "Gold Greathelm",
        51,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        410,
        RecipeType::Craft,
        "Gold Helmet",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        411,
        RecipeType::Craft,
        "Gold ChainLegs",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        412,
        RecipeType::Craft,
        "Gold Greaves",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        413,
        RecipeType::Craft,
        "Gold Legguards",
        51,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        414,
        RecipeType::Craft,
        "Gold Legplates",
        51,
        7200,
        16,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        415,
        RecipeType::Craft,
        "Gold Pants",
        51,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        416,
        RecipeType::Craft,
        "Gold Plateskirt",
        51,
        7200,
        13,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        417,
        RecipeType::Craft,
        "Gold Skirt",
        51,
        7200,
        10,
        CoreStats::new(8, 6, 4),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::SilverIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 305,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::GoldSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        418,
        RecipeType::Craft,
        "Platinum Battleplate",
        61,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 355,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        419,
        RecipeType::Craft,
        "Platinum Chainmail",
        61,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 355,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        420,
        RecipeType::Craft,
        "Platinum Chestplate",
        61,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 355,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        421,
        RecipeType::Craft,
        "Platinum Shirt",
        61,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 355,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        422,
        RecipeType::Craft,
        "Platinum Shoes",
        62,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        423,
        RecipeType::Craft,
        "Platinum Stompers",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        424,
        RecipeType::Craft,
        "Platinum Warboots",
        62,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        425,
        RecipeType::Craft,
        "Platinum Bracelets",
        62,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        426,
        RecipeType::Craft,
        "Platinum Gauntlets",
        62,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        427,
        RecipeType::Craft,
        "Platinum Gloves",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        428,
        RecipeType::Craft,
        "Platinum Handguards",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        429,
        RecipeType::Craft,
        "Platinum Vambraces",
        62,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        430,
        RecipeType::Craft,
        "Platinum Warfist",
        62,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        431,
        RecipeType::Craft,
        "Platinum Cap",
        62,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        432,
        RecipeType::Craft,
        "Platinum ChainCoif",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        433,
        RecipeType::Craft,
        "Platinum Coif",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        434,
        RecipeType::Craft,
        "Platinum Greathelm",
        62,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        435,
        RecipeType::Craft,
        "Platinum Helmet",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        436,
        RecipeType::Craft,
        "Platinum ChainLegs",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        437,
        RecipeType::Craft,
        "Platinum Greaves",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        438,
        RecipeType::Craft,
        "Platinum Legguards",
        62,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        439,
        RecipeType::Craft,
        "Platinum Legplates",
        62,
        7200,
        16,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        440,
        RecipeType::Craft,
        "Platinum Pants",
        62,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        441,
        RecipeType::Craft,
        "Platinum Plateskirt",
        62,
        7200,
        13,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        442,
        RecipeType::Craft,
        "Platinum Skirt",
        62,
        7200,
        10,
        CoreStats::new(9, 7, 5),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::GoldIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 360,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::PlatinumSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        443,
        RecipeType::Craft,
        "Adamantine Battleplate",
        70,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 400,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        444,
        RecipeType::Craft,
        "Adamantine Chainmail",
        70,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 400,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        445,
        RecipeType::Craft,
        "Adamantine Chestplate",
        70,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 400,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        446,
        RecipeType::Craft,
        "Adamantine Shirt",
        70,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 400,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        447,
        RecipeType::Craft,
        "Adamantine Shoes",
        71,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        448,
        RecipeType::Craft,
        "Adamantine Stompers",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        449,
        RecipeType::Craft,
        "Adamantine Warboots",
        71,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        450,
        RecipeType::Craft,
        "Adamantine Bracelets",
        71,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        451,
        RecipeType::Craft,
        "Adamantine Gauntlets",
        71,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        452,
        RecipeType::Craft,
        "Adamantine Gloves",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        453,
        RecipeType::Craft,
        "Adamantine Handguards",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        454,
        RecipeType::Craft,
        "Adamantine Vambraces",
        71,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        455,
        RecipeType::Craft,
        "Adamantine Warfist",
        71,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        456,
        RecipeType::Craft,
        "Adamantine Cap",
        71,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        457,
        RecipeType::Craft,
        "Adamantine ChainCoif",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        458,
        RecipeType::Craft,
        "Adamantine Coif",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        459,
        RecipeType::Craft,
        "Adamantine Greathelm",
        71,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        460,
        RecipeType::Craft,
        "Adamantine Helmet",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        461,
        RecipeType::Craft,
        "Adamantine ChainLegs",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        462,
        RecipeType::Craft,
        "Adamantine Greaves",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        463,
        RecipeType::Craft,
        "Adamantine Legguards",
        71,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        464,
        RecipeType::Craft,
        "Adamantine Legplates",
        71,
        7200,
        16,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        465,
        RecipeType::Craft,
        "Adamantine Pants",
        71,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantinePants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        466,
        RecipeType::Craft,
        "Adamantine Plateskirt",
        71,
        7200,
        13,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantinePlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        467,
        RecipeType::Craft,
        "Adamantine Skirt",
        71,
        7200,
        10,
        CoreStats::new(11, 9, 7),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::PlatinumIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 405,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AdamantineSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        468,
        RecipeType::Craft,
        "Cobalt Battleplate",
        80,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 450,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        469,
        RecipeType::Craft,
        "Cobalt Chainmail",
        80,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 450,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        470,
        RecipeType::Craft,
        "Cobalt Chestplate",
        80,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 450,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        471,
        RecipeType::Craft,
        "Cobalt Shirt",
        80,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 450,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        472,
        RecipeType::Craft,
        "Cobalt Shoes",
        81,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        473,
        RecipeType::Craft,
        "Cobalt Stompers",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        474,
        RecipeType::Craft,
        "Cobalt Warboots",
        81,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        475,
        RecipeType::Craft,
        "Cobalt Bracelets",
        81,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        476,
        RecipeType::Craft,
        "Cobalt Gauntlets",
        81,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        477,
        RecipeType::Craft,
        "Cobalt Gloves",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        478,
        RecipeType::Craft,
        "Cobalt Handguards",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        479,
        RecipeType::Craft,
        "Cobalt Vambraces",
        81,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        480,
        RecipeType::Craft,
        "Cobalt Warfist",
        81,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        481,
        RecipeType::Craft,
        "Cobalt Cap",
        81,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        482,
        RecipeType::Craft,
        "Cobalt ChainCoif",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        483,
        RecipeType::Craft,
        "Cobalt Coif",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        484,
        RecipeType::Craft,
        "Cobalt Greathelm",
        81,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        485,
        RecipeType::Craft,
        "Cobalt Helmet",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        486,
        RecipeType::Craft,
        "Cobalt ChainLegs",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        487,
        RecipeType::Craft,
        "Cobalt Greaves",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        488,
        RecipeType::Craft,
        "Cobalt Legguards",
        81,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        489,
        RecipeType::Craft,
        "Cobalt Legplates",
        81,
        7200,
        16,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        490,
        RecipeType::Craft,
        "Cobalt Pants",
        81,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        491,
        RecipeType::Craft,
        "Cobalt Plateskirt",
        81,
        7200,
        13,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        492,
        RecipeType::Craft,
        "Cobalt Skirt",
        81,
        7200,
        10,
        CoreStats::new(14, 11, 8),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AdamantineIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 455,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CobaltSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        493,
        RecipeType::Craft,
        "Angelic Battleplate",
        91,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 505,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        494,
        RecipeType::Craft,
        "Angelic Chainmail",
        91,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 505,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        495,
        RecipeType::Craft,
        "Angelic Chestplate",
        91,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 505,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        496,
        RecipeType::Craft,
        "Angelic Shirt",
        91,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 505,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        497,
        RecipeType::Craft,
        "Angelic Shoes",
        92,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        498,
        RecipeType::Craft,
        "Angelic Stompers",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        499,
        RecipeType::Craft,
        "Angelic Warboots",
        92,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        500,
        RecipeType::Craft,
        "Angelic Bracelets",
        92,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        501,
        RecipeType::Craft,
        "Angelic Gauntlets",
        92,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        502,
        RecipeType::Craft,
        "Angelic Gloves",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        503,
        RecipeType::Craft,
        "Angelic Handguards",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        504,
        RecipeType::Craft,
        "Angelic Vambraces",
        92,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        505,
        RecipeType::Craft,
        "Angelic Warfist",
        92,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        506,
        RecipeType::Craft,
        "Angelic Cap",
        92,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        507,
        RecipeType::Craft,
        "Angelic ChainCoif",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        508,
        RecipeType::Craft,
        "Angelic Coif",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        509,
        RecipeType::Craft,
        "Angelic Greathelm",
        92,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        510,
        RecipeType::Craft,
        "Angelic Helmet",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        511,
        RecipeType::Craft,
        "Angelic ChainLegs",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        512,
        RecipeType::Craft,
        "Angelic Greaves",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        513,
        RecipeType::Craft,
        "Angelic Legguards",
        92,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        514,
        RecipeType::Craft,
        "Angelic Legplates",
        92,
        7200,
        16,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        515,
        RecipeType::Craft,
        "Angelic Pants",
        92,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        516,
        RecipeType::Craft,
        "Angelic Plateskirt",
        92,
        7200,
        13,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        517,
        RecipeType::Craft,
        "Angelic Skirt",
        92,
        7200,
        10,
        CoreStats::new(16, 13, 9),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CobaltIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 510,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AngelicSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        518,
        RecipeType::Craft,
        "Crimson Battleplate",
        105,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 575,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        519,
        RecipeType::Craft,
        "Crimson Chainmail",
        105,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 575,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        520,
        RecipeType::Craft,
        "Crimson Chestplate",
        105,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 575,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        521,
        RecipeType::Craft,
        "Crimson Shirt",
        105,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 575,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        522,
        RecipeType::Craft,
        "Crimson Shoes",
        106,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        523,
        RecipeType::Craft,
        "Crimson Stompers",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        524,
        RecipeType::Craft,
        "Crimson Warboots",
        106,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        525,
        RecipeType::Craft,
        "Crimson Bracelets",
        106,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        526,
        RecipeType::Craft,
        "Crimson Gauntlets",
        106,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        527,
        RecipeType::Craft,
        "Crimson Gloves",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        528,
        RecipeType::Craft,
        "Crimson Handguards",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        529,
        RecipeType::Craft,
        "Crimson Vambraces",
        106,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        530,
        RecipeType::Craft,
        "Crimson Warfist",
        106,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        531,
        RecipeType::Craft,
        "Crimson Cap",
        106,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        532,
        RecipeType::Craft,
        "Crimson ChainCoif",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        533,
        RecipeType::Craft,
        "Crimson Coif",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        534,
        RecipeType::Craft,
        "Crimson Greathelm",
        106,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        535,
        RecipeType::Craft,
        "Crimson Helmet",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        536,
        RecipeType::Craft,
        "Crimson ChainLegs",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        537,
        RecipeType::Craft,
        "Crimson Greaves",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        538,
        RecipeType::Craft,
        "Crimson Legguards",
        106,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        539,
        RecipeType::Craft,
        "Crimson Legplates",
        106,
        7200,
        16,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        540,
        RecipeType::Craft,
        "Crimson Pants",
        106,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        541,
        RecipeType::Craft,
        "Crimson Plateskirt",
        106,
        7200,
        13,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        542,
        RecipeType::Craft,
        "Crimson Skirt",
        106,
        7200,
        10,
        CoreStats::new(19, 15, 11),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 580,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::CrimsonSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        543,
        RecipeType::Craft,
        "Nova Battleplate",
        113,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 615,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        544,
        RecipeType::Craft,
        "Nova Chainmail",
        113,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 615,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        545,
        RecipeType::Craft,
        "Nova Chestplate",
        113,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 615,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        546,
        RecipeType::Craft,
        "Nova Shirt",
        113,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 615,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        547,
        RecipeType::Craft,
        "Nova Shoes",
        114,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        548,
        RecipeType::Craft,
        "Nova Stompers",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        549,
        RecipeType::Craft,
        "Nova Warboots",
        114,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        550,
        RecipeType::Craft,
        "Nova Bracelets",
        114,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        551,
        RecipeType::Craft,
        "Nova Gauntlets",
        114,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        552,
        RecipeType::Craft,
        "Nova Gloves",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        553,
        RecipeType::Craft,
        "Nova Handguards",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        554,
        RecipeType::Craft,
        "Nova Vambraces",
        114,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        555,
        RecipeType::Craft,
        "Nova Warfist",
        114,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        556,
        RecipeType::Craft,
        "Nova Cap",
        114,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        557,
        RecipeType::Craft,
        "Nova ChainCoif",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        558,
        RecipeType::Craft,
        "Nova Coif",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        559,
        RecipeType::Craft,
        "Nova Greathelm",
        114,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        560,
        RecipeType::Craft,
        "Nova Helmet",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        561,
        RecipeType::Craft,
        "Nova ChainLegs",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        562,
        RecipeType::Craft,
        "Nova Greaves",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        563,
        RecipeType::Craft,
        "Nova Legguards",
        114,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        564,
        RecipeType::Craft,
        "Nova Legplates",
        114,
        7200,
        16,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        565,
        RecipeType::Craft,
        "Nova Pants",
        114,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        566,
        RecipeType::Craft,
        "Nova Plateskirt",
        114,
        7200,
        13,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        567,
        RecipeType::Craft,
        "Nova Skirt",
        114,
        7200,
        10,
        CoreStats::new(21, 16, 12),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::AngelicIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 620,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::NovaSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        568,
        RecipeType::Craft,
        "Fateful Battleplate",
        121,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 655,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        569,
        RecipeType::Craft,
        "Fateful Chainmail",
        121,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 655,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        570,
        RecipeType::Craft,
        "Fateful Chestplate",
        121,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 655,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        571,
        RecipeType::Craft,
        "Fateful Shirt",
        121,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 655,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        572,
        RecipeType::Craft,
        "Fateful Shoes",
        122,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        573,
        RecipeType::Craft,
        "Fateful Stompers",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        574,
        RecipeType::Craft,
        "Fateful Warboots",
        122,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        575,
        RecipeType::Craft,
        "Fateful Bracelets",
        122,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        576,
        RecipeType::Craft,
        "Fateful Gauntlets",
        122,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        577,
        RecipeType::Craft,
        "Fateful Gloves",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        578,
        RecipeType::Craft,
        "Fateful Handguards",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        579,
        RecipeType::Craft,
        "Fateful Vambraces",
        122,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        580,
        RecipeType::Craft,
        "Fateful Warfist",
        122,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        581,
        RecipeType::Craft,
        "Fateful Cap",
        122,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        582,
        RecipeType::Craft,
        "Fateful ChainCoif",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        583,
        RecipeType::Craft,
        "Fateful Coif",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        584,
        RecipeType::Craft,
        "Fateful Greathelm",
        122,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        585,
        RecipeType::Craft,
        "Fateful Helmet",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        586,
        RecipeType::Craft,
        "Fateful ChainLegs",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        587,
        RecipeType::Craft,
        "Fateful Greaves",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        588,
        RecipeType::Craft,
        "Fateful Legguards",
        122,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        589,
        RecipeType::Craft,
        "Fateful Legplates",
        122,
        7200,
        16,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        590,
        RecipeType::Craft,
        "Fateful Pants",
        122,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        591,
        RecipeType::Craft,
        "Fateful Plateskirt",
        122,
        7200,
        13,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulPlateskirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        592,
        RecipeType::Craft,
        "Fateful Skirt",
        122,
        7200,
        10,
        CoreStats::new(23, 18, 13),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::CrimsonIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 660,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::FatefulSkirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        593,
        RecipeType::Craft,
        "Altair Battleplate",
        135,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 4,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 725,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairBattleplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        594,
        RecipeType::Craft,
        "Altair Chainmail",
        135,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 725,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairChainmail,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        595,
        RecipeType::Craft,
        "Altair Chestplate",
        135,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 725,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairChestplate,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        596,
        RecipeType::Craft,
        "Altair Shirt",
        135,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 725,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairShirt,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        597,
        RecipeType::Craft,
        "Altair Shoes",
        136,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairShoes,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        598,
        RecipeType::Craft,
        "Altair Stompers",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairStompers,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        599,
        RecipeType::Craft,
        "Altair Warboots",
        136,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairWarboots,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        600,
        RecipeType::Craft,
        "Altair Bracelets",
        136,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairBracelets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        601,
        RecipeType::Craft,
        "Altair Gauntlets",
        136,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairGauntlets,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        602,
        RecipeType::Craft,
        "Altair Gloves",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairGloves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        603,
        RecipeType::Craft,
        "Altair Handguards",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairHandguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        604,
        RecipeType::Craft,
        "Altair Vambraces",
        136,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairVambraces,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        605,
        RecipeType::Craft,
        "Altair Warfist",
        136,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairWarfist,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        606,
        RecipeType::Craft,
        "Altair Cap",
        136,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairCap,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        607,
        RecipeType::Craft,
        "Altair ChainCoif",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairChainCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        608,
        RecipeType::Craft,
        "Altair Coif",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairCoif,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        609,
        RecipeType::Craft,
        "Altair Greathelm",
        136,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairGreathelm,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        610,
        RecipeType::Craft,
        "Altair Helmet",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairHelmet,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        611,
        RecipeType::Craft,
        "Altair ChainLegs",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairChainLegs,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        612,
        RecipeType::Craft,
        "Altair Greaves",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairGreaves,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        613,
        RecipeType::Craft,
        "Altair Legguards",
        136,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairLegguards,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        614,
        RecipeType::Craft,
        "Altair Legplates",
        136,
        7200,
        16,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 3,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairLegplates,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        615,
        RecipeType::Craft,
        "Altair Pants",
        136,
        7200,
        10,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 2,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairPants,
            amount: 1,
        },
    ),
    RecipeCodexEntry::new(
        616,
        RecipeType::Craft,
        "Altair Plateskirt",
        136,
        7200,
        13,
        CoreStats::new(27, 22, 16),
        RecipeInput::Craft {
            materials: &[
                RecipeMaterial {
                    kind: ResourceKind::AltairIngot,
                    amount: 2,
                },
                RecipeMaterial {
                    kind: ResourceKind::FatefulIngot,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Leather,
                    amount: 1,
                },
                RecipeMaterial {
                    kind: ResourceKind::Plank,
                    amount: 1,
                },
            ],
            gold_amount: 730,
        },
        RecipeOutput::Resource {
            resource: ResourceKind::AltairPlateskirt,
            amount: 1,
        },
    ),
];

#[inline]
pub fn recipe_by_id(id: u16) -> Option<&'static RecipeCodexEntry> {
    if id == 0 {
        return None;
    }
    let idx = (id - 1) as usize;
    RECIPE_CODEX.get(idx)
}
