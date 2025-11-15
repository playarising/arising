pub mod base;
pub mod civilizations;
pub mod equipment;
pub mod level;
pub mod quests;
pub mod recipes;
pub mod resources;

pub use base::{
    Attributes, CharacterClass, Civilization, ClassBonus, CoreStats, Gender, PassiveEffect,
};
pub use civilizations::{CivilizationCodexEntry, CIVILIZATION_CODEX};
pub use equipment::{
    equipment_definition, equipment_definition_by_index, equipment_definition_index,
    EquipmentCategory, EquipmentDefinition, EquipmentSlot, EQUIPMENT_CODEX,
};
pub use level::{
    level_from_experience, next_requirement_after, requirement_for,
    total_attribute_points_for_level, total_core_points_for_level, LevelCurveEntry, LEVEL_CAP,
    LEVEL_CURVE,
};
pub use quests::{quest_by_id, QuestCodexEntry, QuestReward, QuestType, QUEST_CODEX};
pub use recipes::{
    recipe_by_id, RecipeCodexEntry, RecipeInput, RecipeOutput, RecipeType, RECIPE_CODEX,
};
pub use resources::ResourceKind;
