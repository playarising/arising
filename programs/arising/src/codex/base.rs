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
            "https://mints.playarising.com/{}/{}",
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

    pub const fn sum(&self) -> u16 {
        self.might + self.speed + self.intellect
    }

    pub fn max_components(self, floor: CoreStats) -> Self {
        Self::new(
            self.might.max(floor.might),
            self.speed.max(floor.speed),
            self.intellect.max(floor.intellect),
        )
    }

    pub fn enforce_total_floor(mut self, floor: u16) -> Self {
        let mut deficit = floor.saturating_sub(self.sum());
        while deficit > 0 {
            if self.might <= self.speed && self.might <= self.intellect {
                self.might = self.might.saturating_add(1);
            } else if self.speed <= self.intellect {
                self.speed = self.speed.saturating_add(1);
            } else {
                self.intellect = self.intellect.saturating_add(1);
            }
            deficit -= 1;
        }
        self
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
