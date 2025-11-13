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
            Civilization::Ard => PassiveEffect::JobsBonusGold10,
            Civilization::Hartenn => PassiveEffect::ForgeOutput15,
            Civilization::Ikarans => PassiveEffect::FarmRawMats10,
            Civilization::Zhand => PassiveEffect::RaidsEnergyDiscount10,
            Civilization::Shinkari => PassiveEffect::CraftQuality5,
            Civilization::Tarki => PassiveEffect::JobsBonusGold10,
        }
    }

    pub const fn theme(self) -> &'static str {
        match self {
            Civilization::Ard => "Knightly humans",
            Civilization::Hartenn => "Dwarves",
            Civilization::Ikarans => "Wood elves",
            Civilization::Zhand => "Desert elves",
            Civilization::Shinkari => "Eastern humans",
            Civilization::Tarki => "Viking-like humans",
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PassiveEffect {
    JobsBonusGold10,
    ForgeOutput15,
    FarmRawMats10,
    RaidsEnergyDiscount10,
    CraftQuality5,
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
}

pub struct CivilizationCodexEntry {
    pub civilization: Civilization,
    pub theme: &'static str,
    pub base_stats: CoreStats,
    pub base_attributes: Attributes,
    pub passive: PassiveEffect,
}

pub const CIVILIZATION_CODEX: [CivilizationCodexEntry; 6] = [
    CivilizationCodexEntry {
        civilization: Civilization::Ard,
        theme: Civilization::Ard.theme(),
        base_stats: Civilization::Ard.base_stats(),
        base_attributes: Civilization::Ard.base_attributes(),
        passive: Civilization::Ard.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Hartenn,
        theme: Civilization::Hartenn.theme(),
        base_stats: Civilization::Hartenn.base_stats(),
        base_attributes: Civilization::Hartenn.base_attributes(),
        passive: Civilization::Hartenn.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Ikarans,
        theme: Civilization::Ikarans.theme(),
        base_stats: Civilization::Ikarans.base_stats(),
        base_attributes: Civilization::Ikarans.base_attributes(),
        passive: Civilization::Ikarans.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Zhand,
        theme: Civilization::Zhand.theme(),
        base_stats: Civilization::Zhand.base_stats(),
        base_attributes: Civilization::Zhand.base_attributes(),
        passive: Civilization::Zhand.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Shinkari,
        theme: Civilization::Shinkari.theme(),
        base_stats: Civilization::Shinkari.base_stats(),
        base_attributes: Civilization::Shinkari.base_attributes(),
        passive: Civilization::Shinkari.passive(),
    },
    CivilizationCodexEntry {
        civilization: Civilization::Tarki,
        theme: Civilization::Tarki.theme(),
        base_stats: Civilization::Tarki.base_stats(),
        base_attributes: Civilization::Tarki.base_attributes(),
        passive: Civilization::Tarki.passive(),
    },
];
