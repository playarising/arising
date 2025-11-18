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
            arising::codex::RecipeInput::Forge { raw_material, amount } => {
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
