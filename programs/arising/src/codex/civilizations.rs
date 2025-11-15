use super::base::{Attributes, Civilization, CoreStats, PassiveEffect};

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
