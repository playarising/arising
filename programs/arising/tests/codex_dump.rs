use std::{fs, fs::File, io::Write, path::Path};

use anchor_lang::prelude::Pubkey;
use arising::codex::{
    Attributes, CoreStats, EquipmentCategory, EquipmentSlot, QuestReward, QuestType, RecipeInput,
    RecipeOutput, RecipeType, ResourceKind, EQUIPMENT_CODEX, QUEST_CODEX, RECIPE_CODEX,
};
use arising::ID;
use serde::Serialize;

const OUTPUT_DIR: &str = "../../target/codex";
const RESOURCE_MINT_SEED: &[u8] = b"resource_mint";
type DynResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Serialize)]
struct CoreStatsOut {
    might: u16,
    speed: u16,
    intellect: u16,
}

#[derive(Serialize)]
struct AttributesOut {
    atk: i16,
    def: i16,
    mag_atk: i16,
    mag_def: i16,
    range: i16,
    rate: i16,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum QuestRewardOut {
    Experience {
        amount: u64,
    },
    Resource {
        resource_id: u16,
        resource: String,
        amount: u64,
    },
}

#[derive(Serialize)]
struct QuestOut {
    id: u16,
    quest_type: String,
    name: String,
    level_required: u8,
    cooldown_seconds: u32,
    base_energy_cost: u16,
    minimum_stats: CoreStatsOut,
    rewards: Vec<QuestRewardOut>,
    energy_cost_calculation: EnergyCostCalculationOut,
}

#[derive(Serialize)]
struct EnergyCostCalculationOut {
    /// Minimum possible energy cost (with perfect stats, high level)
    min_energy_cost: u16,
    /// Maximum possible energy cost (with minimal stats, low level)
    max_energy_cost: u16,
    /// Energy cost at exact requirements (character stats = quest requirements)
    at_requirements: u16,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum RecipeInputOut {
    Forge {
        raw_material_id: u16,
        raw_material: String,
        amount: u64,
    },
    Craft {
        materials: Vec<RecipeMaterialOut>,
        gold_amount: u64,
    },
}

#[derive(Serialize)]
struct RecipeMaterialOut {
    resource_id: u16,
    resource: String,
    amount: u64,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum RecipeOutputOut {
    Resource {
        resource_id: u16,
        resource: String,
        amount: u64,
    },
}

#[derive(Serialize)]
struct RecipeOut {
    id: u16,
    recipe_type: String,
    name: String,
    level_required: u8,
    cooldown_seconds: u32,
    base_energy_cost: u16,
    minimum_stats: CoreStatsOut,
    input: RecipeInputOut,
    output: RecipeOutputOut,
}

#[derive(Serialize)]
struct EquipmentOut {
    index: usize,
    resource_id: u16,
    resource: String,
    category: String,
    allowed_slots: Vec<String>,
    occupies_both_hands: bool,
    occupies_forearm: bool,
    level_required: u8,
    attribute_bonus: AttributesOut,
}

#[derive(Serialize)]
struct ResourceMintOut {
    resource_id: u16,
    resource: String,
    mint: String,
}

fn core_stats_out(stats: CoreStats) -> CoreStatsOut {
    CoreStatsOut {
        might: stats.might,
        speed: stats.speed,
        intellect: stats.intellect,
    }
}

fn attributes_out(attrs: Attributes) -> AttributesOut {
    AttributesOut {
        atk: attrs.atk,
        def: attrs.def,
        mag_atk: attrs.mag_atk,
        mag_def: attrs.mag_def,
        range: attrs.range,
        rate: attrs.rate,
    }
}

fn quest_type_name(q: QuestType) -> &'static str {
    match q {
        QuestType::Job => "Job",
        QuestType::Farm => "Farm",
        QuestType::Raid => "Raid",
    }
}

fn recipe_type_name(r: RecipeType) -> &'static str {
    match r {
        RecipeType::Forge => "Forge",
        RecipeType::Craft => "Craft",
    }
}

fn category_name(c: EquipmentCategory) -> &'static str {
    match c {
        EquipmentCategory::Weapon => "Weapon",
        EquipmentCategory::Armor => "Armor",
        EquipmentCategory::Accessory => "Accessory",
        EquipmentCategory::Back => "Back",
    }
}

fn slot_name(slot: EquipmentSlot) -> &'static str {
    match slot {
        EquipmentSlot::LeftHand => "LeftHand",
        EquipmentSlot::RightHand => "RightHand",
        EquipmentSlot::LeftForearm => "LeftForearm",
        EquipmentSlot::RightForearm => "RightForearm",
        EquipmentSlot::Chest => "Chest",
        EquipmentSlot::Legs => "Legs",
        EquipmentSlot::Boots => "Boots",
        EquipmentSlot::Helmet => "Helmet",
        EquipmentSlot::RingOne => "RingOne",
        EquipmentSlot::RingTwo => "RingTwo",
        EquipmentSlot::AmuletOne => "AmuletOne",
        EquipmentSlot::AmuletTwo => "AmuletTwo",
        EquipmentSlot::Back => "Back",
    }
}

fn resource_name(kind: ResourceKind) -> String {
    format!("{:?}", kind)
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> DynResult {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut file = File::create(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn dump_quests() -> DynResult {
    let quests: Vec<QuestOut> = QUEST_CODEX
        .iter()
        .map(|q| {
            let required_stats = q.effective_minimum_stats();

            // Calculate energy cost scenarios
            // Min: High stats (2x requirements), max level
            let high_stats = CoreStats::new(
                required_stats.might.saturating_mul(2).max(100),
                required_stats.speed.saturating_mul(2).max(100),
                required_stats.intellect.saturating_mul(2).max(100),
            );
            let min_energy = q.energy_cost(high_stats, 100);

            // Max: At requirements, at quest level (worst case for modifiers)
            let max_energy = q.energy_cost(required_stats, q.level_required);

            // At requirements: Exact stats, at quest level
            let at_req = q.energy_cost(required_stats, q.level_required);

            QuestOut {
                id: q.id,
                quest_type: quest_type_name(q.quest_type).to_string(),
                name: q.name.to_string(),
                level_required: q.level_required,
                cooldown_seconds: q.cooldown_seconds,
                base_energy_cost: q.effective_base_energy(),
                minimum_stats: core_stats_out(required_stats),
                rewards: q
                    .rewards
                    .iter()
                    .map(|r| match r {
                        QuestReward::Experience { amount } => {
                            QuestRewardOut::Experience { amount: *amount }
                        }
                        QuestReward::Resource { resource, amount } => QuestRewardOut::Resource {
                            resource_id: resource.id(),
                            resource: resource_name(*resource),
                            amount: *amount,
                        },
                    })
                    .collect(),
                energy_cost_calculation: EnergyCostCalculationOut {
                    min_energy_cost: min_energy,
                    max_energy_cost: max_energy,
                    at_requirements: at_req,
                },
            }
        })
        .collect();
    write_json_file(Path::new(OUTPUT_DIR).join("quests.json").as_path(), &quests)
}

fn dump_recipes() -> DynResult {
    let recipes: Vec<RecipeOut> = RECIPE_CODEX
        .iter()
        .map(|r| RecipeOut {
            id: r.id,
            recipe_type: recipe_type_name(r.recipe_type).to_string(),
            name: r.name.to_string(),
            level_required: r.level_required,
            cooldown_seconds: r.cooldown_seconds,
            base_energy_cost: r.effective_base_energy(),
            minimum_stats: core_stats_out(r.effective_minimum_stats()),
            input: match r.input {
                RecipeInput::Forge {
                    raw_material,
                    amount,
                } => RecipeInputOut::Forge {
                    raw_material_id: raw_material.id(),
                    raw_material: resource_name(raw_material),
                    amount,
                },
                RecipeInput::Craft {
                    materials,
                    gold_amount,
                } => RecipeInputOut::Craft {
                    materials: materials
                        .iter()
                        .map(|m| RecipeMaterialOut {
                            resource_id: m.kind.id(),
                            resource: resource_name(m.kind),
                            amount: m.amount,
                        })
                        .collect(),
                    gold_amount,
                },
            },
            output: match r.output {
                RecipeOutput::Resource { resource, amount } => RecipeOutputOut::Resource {
                    resource_id: resource.id(),
                    resource: resource_name(resource),
                    amount,
                },
            },
        })
        .collect();
    write_json_file(
        Path::new(OUTPUT_DIR).join("recipes.json").as_path(),
        &recipes,
    )
}

fn dump_equipment() -> DynResult {
    let equipment: Vec<EquipmentOut> = EQUIPMENT_CODEX
        .iter()
        .enumerate()
        .map(|(index, e)| EquipmentOut {
            index,
            resource_id: e.resource.id(),
            resource: resource_name(e.resource),
            category: category_name(e.category).to_string(),
            allowed_slots: e
                .allowed_slots
                .iter()
                .map(|s| slot_name(*s).to_string())
                .collect(),
            occupies_both_hands: e.occupies_both_hands,
            occupies_forearm: e.occupies_forearm,
            level_required: e.level_required,
            attribute_bonus: attributes_out(e.attribute_bonus),
        })
        .collect();
    write_json_file(
        Path::new(OUTPUT_DIR).join("equipment.json").as_path(),
        &equipment,
    )
}

fn dump_resource_mints() -> DynResult {
    let items: Vec<ResourceMintOut> = (0..ResourceKind::COUNT)
        .filter_map(ResourceKind::from_id)
        .map(|resource| {
            let (mint, _) =
                Pubkey::find_program_address(&[RESOURCE_MINT_SEED, resource.seed_bytes()], &ID);
            ResourceMintOut {
                resource_id: resource.id(),
                resource: resource_name(resource),
                mint: mint.to_string(),
            }
        })
        .collect();
    write_json_file(
        Path::new(OUTPUT_DIR).join("resource_mints.json").as_path(),
        &items,
    )
}

#[test]
fn dump_codex_to_json() {
    dump_quests().unwrap();
    dump_recipes().unwrap();
    dump_equipment().unwrap();
    dump_resource_mints().unwrap();
}
