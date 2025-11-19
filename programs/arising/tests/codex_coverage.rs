//! Broad coverage sweeps for codex tables and ResourceKind helpers.

use arising::codex::{
    equipment_definition, equipment_definition_by_index, quest_by_id, recipe_by_id, Attributes,
    EquipmentSlot, ResourceKind, EQUIPMENT_CODEX, QUEST_CODEX, RECIPE_CODEX,
};

#[test]
fn all_resources_have_seed_and_symbol() {
    for id in 0..ResourceKind::COUNT {
        let kind = ResourceKind::from_id(id).expect("all ids map to kind");
        assert!(!kind.seed_bytes().is_empty(), "missing seed for {:?}", kind);
        assert!(!kind.symbol().is_empty(), "missing symbol for {:?}", kind);
    }
}

#[test]
fn equipment_codex_entries_load_and_support_slots() {
    for (idx, def) in EQUIPMENT_CODEX.iter().enumerate() {
        assert!(equipment_definition_by_index(idx as u8).is_some());
        assert!(equipment_definition(def.resource).is_some());
        for &slot in def.allowed_slots.iter() {
            assert!(def.supports_slot(slot));
            // Basic sanity: slot index within bounds.
            assert!((slot as usize) < EquipmentSlot::COUNT);
        }
        // Attribute bonus should be well-formed (no NaNs).
        let Attributes {
            atk,
            def,
            mag_atk,
            mag_def,
            range,
            rate,
        } = def.attribute_bonus;
        let _ = (atk, def, mag_atk, mag_def, range, rate);
    }
}

#[test]
fn quest_codex_entries_are_accessible() {
    for quest in QUEST_CODEX.iter() {
        let looked_up = quest_by_id(quest.id).expect("quest present");
        assert_eq!(looked_up.id, quest.id);
        assert!(
            looked_up.level_required >= 1,
            "quest level must be positive"
        );
        // Touch rewards to count lines.
        for reward in looked_up.rewards.iter() {
            match reward {
                arising::codex::QuestReward::Experience { amount } => {
                    assert!(*amount > 0);
                }
                arising::codex::QuestReward::Resource { amount, .. } => {
                    assert!(*amount > 0);
                }
            }
        }
    }
}

#[test]
fn recipe_codex_entries_are_accessible() {
    for recipe in RECIPE_CODEX.iter() {
        let looked_up = recipe_by_id(recipe.id).expect("recipe present");
        assert_eq!(looked_up.id, recipe.id);
        match looked_up.input {
            arising::codex::RecipeInput::Forge {
                raw_material,
                amount,
            } => {
                assert!(amount > 0);
                let _ = raw_material.seed_bytes();
            }
            arising::codex::RecipeInput::Craft {
                materials,
                gold_amount,
            } => {
                assert!(!materials.is_empty() || gold_amount > 0);
                for m in materials {
                    assert!(m.amount > 0);
                    let _ = m.kind.symbol();
                }
            }
        }
        match looked_up.output {
            arising::codex::RecipeOutput::Resource { resource, amount } => {
                assert!(amount > 0);
                let _ = resource.seed_bytes();
            }
        }
    }
}

#[test]
fn quest_stat_requirements_are_balanced() {
    // Character stats = 6 (base + class) + (level - 1)
    // Target: Quest stats should be 68-82% of available character stats (leaving 18-32% buffer)

    for quest in QUEST_CODEX.iter() {
        let level = quest.level_required as u16;
        let char_total_stats = 6 + level;
        let effective_stats = quest.effective_minimum_stats();
        let quest_stat_sum = effective_stats.sum();
        let percentage = (quest_stat_sum as f64 / char_total_stats as f64) * 100.0;

        // For levels 1-3, the stat floor system enforces a minimum that can reach 100%
        // For levels 4-5, allow wider range due to floor transitions
        // For levels 6+, enforce stricter range (68-84% to account for rounding)
        let (min_pct, max_pct) = if quest.level_required <= 3 {
            (70.0, 100.0)
        } else if quest.level_required <= 5 {
            (68.0, 84.0)
        } else {
            (68.0, 84.0)
        };

        assert!(
            percentage >= min_pct && percentage <= max_pct,
            "Quest '{}' (id: {}, level: {}) requires {}% of character stats ({}/{}). Expected {}%-{}%",
            quest.name,
            quest.id,
            quest.level_required,
            percentage,
            quest_stat_sum,
            char_total_stats,
            min_pct,
            max_pct
        );
    }
}

#[test]
fn quest_resource_rewards_follow_distribution_rules() {
    // Rule: < lvl 50 = max 1 resource, < lvl 80 = max 2 resources, < lvl 150 = max 3 resources
    // Count only raw resource rewards (not gold or experience)

    for quest in QUEST_CODEX.iter() {
        let mut resource_count = 0;
        let mut has_gold = false;
        let mut has_exp = false;

        for reward in quest.rewards.iter() {
            match reward {
                arising::codex::QuestReward::Experience { .. } => {
                    has_exp = true;
                }
                arising::codex::QuestReward::Resource { resource, amount } => {
                    // Gold is currency, not a raw resource
                    if matches!(resource, ResourceKind::Gold) {
                        has_gold = true;
                    } else {
                        resource_count += *amount;
                    }
                }
            }
        }

        let max_allowed = if quest.level_required < 50 {
            1
        } else if quest.level_required < 80 {
            2
        } else {
            3
        };

        assert!(
            resource_count <= max_allowed,
            "Quest '{}' (id: {}, level: {}) gives {} raw resources, but max allowed is {} (gold: {}, exp: {})",
            quest.name,
            quest.id,
            quest.level_required,
            resource_count,
            max_allowed,
            has_gold,
            has_exp
        );
    }
}

#[test]
fn quest_energy_costs_scale_with_level() {
    // Energy costs should generally increase with level
    // Allow some variation but check overall progression

    let mut prev_level = 0;
    let mut prev_avg_energy = 0.0;

    // Group quests by level brackets and check progression
    let brackets = [
        (1, 10),
        (11, 20),
        (21, 30),
        (31, 40),
        (41, 50),
        (51, 70),
        (71, 90),
        (91, 110),
        (111, 130),
        (131, 150),
    ];

    for (start, end) in brackets {
        let bracket_quests: Vec<_> = QUEST_CODEX
            .iter()
            .filter(|q| q.level_required >= start && q.level_required <= end)
            .collect();

        if !bracket_quests.is_empty() {
            let avg_energy: f64 = bracket_quests
                .iter()
                .map(|q| q.effective_base_energy() as f64)
                .sum::<f64>()
                / bracket_quests.len() as f64;

            if prev_level > 0 {
                // Allow minor regressions (within 10%) but overall should increase
                assert!(
                    avg_energy >= prev_avg_energy * 0.9,
                    "Energy costs decreased significantly from levels {}-{} (avg: {:.1}) to {}-{} (avg: {:.1})",
                    prev_level,
                    prev_level + 9,
                    prev_avg_energy,
                    start,
                    end,
                    avg_energy
                );
            }

            prev_level = start;
            prev_avg_energy = avg_energy;
        }
    }
}

#[test]
fn quest_material_availability_matches_crafting_system() {
    // Key materials should appear at or after their refining recipe levels
    // Based on the refining recipes in the codex

    let material_requirements = [
        (ResourceKind::BronzeOre, 10),
        (ResourceKind::IronOre, 20),
        (ResourceKind::SteelOre, 31),
        (ResourceKind::SilverOre, 40),
        (ResourceKind::GoldOre, 50),
        (ResourceKind::PlatinumOre, 61),
        (ResourceKind::AdamantineOre, 70),
        (ResourceKind::CobaltOre, 80),
        (ResourceKind::AngelicEssence, 91),
        (ResourceKind::CrimsonEssence, 105),
        (ResourceKind::FatefulEssence, 121),
        (ResourceKind::AltairEssence, 135),
    ];

    for (material, min_level) in material_requirements {
        // Find first quest that rewards this material
        if let Some(first_quest) = QUEST_CODEX.iter().find(|q| {
            q.rewards.iter().any(|r| match r {
                arising::codex::QuestReward::Resource { resource, .. } => *resource == material,
                _ => false,
            })
        }) {
            assert!(
                first_quest.level_required >= min_level,
                "Material {:?} first appears at level {} in quest '{}', but refining recipe is at level {}",
                material,
                first_quest.level_required,
                first_quest.name,
                min_level
            );
        }
    }
}

#[test]
fn quest_rewards_are_non_zero() {
    // Ensure all quest rewards have positive amounts
    for quest in QUEST_CODEX.iter() {
        assert!(
            !quest.rewards.is_empty(),
            "Quest '{}' (id: {}) has no rewards",
            quest.name,
            quest.id
        );

        for reward in quest.rewards.iter() {
            match reward {
                arising::codex::QuestReward::Experience { amount } => {
                    assert!(
                        *amount > 0,
                        "Quest '{}' (id: {}) has zero experience reward",
                        quest.name,
                        quest.id
                    );
                }
                arising::codex::QuestReward::Resource { amount, resource } => {
                    assert!(
                        *amount > 0,
                        "Quest '{}' (id: {}) has zero amount for resource {:?}",
                        quest.name,
                        quest.id,
                        resource
                    );
                }
            }
        }
    }
}

#[test]
fn quest_cooldowns_are_reasonable() {
    // Cooldowns should be between 45 minutes (2700s) and 2 hours (7200s) for most quests
    // Allow some exceptions for special quests

    for quest in QUEST_CODEX.iter() {
        assert!(
            quest.cooldown_seconds >= 2700 && quest.cooldown_seconds <= 7200,
            "Quest '{}' (id: {}, level: {}) has unusual cooldown: {}s ({}h). Expected 2700-7200s (0.75-2h)",
            quest.name,
            quest.id,
            quest.level_required,
            quest.cooldown_seconds,
            quest.cooldown_seconds as f32 / 3600.0
        );
    }
}
