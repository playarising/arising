use anchor_lang::prelude::*;

use crate::{
    codex::{Attributes, Civilization, CoreStats, Gender},
    level_codex::{
        level_from_experience, total_attribute_points_for_level, total_core_points_for_level,
    },
};

pub const CHARACTER_SEED: &[u8] = b"character";
pub const CHARACTER_MINT_STATE_SEED: &[u8] = b"character_mint_state";

#[account]
pub struct Character {
    pub authority: Pubkey,
    pub bump: u8,
    pub character_id: u64,
    pub gender: Gender,
    pub civilization: Civilization,
    pub experience: u64,
    pub stats: CoreStats,
    pub attributes: Attributes,
    pub cnft_tree: Pubkey,
    pub cnft_leaf_index: u64,
    pub cnft_hash: [u8; 32],
    pub created_at: i64,
}

impl Character {
    pub const LEN: usize = 8  // discriminator
        + 32 // authority
        + 1 // bump
        + 8 // character_id
        + 1 // gender
        + 1 // civilization
        + 8 // experience
        + CoreStats::SIZE
        + Attributes::SIZE
        + 32 // cnft_tree
        + 8 // cnft_leaf_index
        + 32 // cnft_hash
        + 8; // created_at

    pub fn current_level(&self) -> u8 {
        level_from_experience(self.experience)
    }

    pub fn total_core_points(&self) -> u16 {
        total_core_points_for_level(self.current_level())
    }

    pub fn total_attribute_points(&self) -> u16 {
        total_attribute_points_for_level(self.current_level())
    }

    pub fn spent_core_points(&self) -> i32 {
        let base = self.civilization.base_stats();
        let spent_might = self.stats.might as i32 - base.might as i32;
        let spent_speed = self.stats.speed as i32 - base.speed as i32;
        let spent_intellect = self.stats.intellect as i32 - base.intellect as i32;
        spent_might + spent_speed + spent_intellect
    }

    pub fn spent_attribute_points(&self) -> i32 {
        let base = self.civilization.base_attributes();
        let spent_atk = self.attributes.atk as i32 - base.atk as i32;
        let spent_def = self.attributes.def as i32 - base.def as i32;
        let spent_mag_atk = self.attributes.mag_atk as i32 - base.mag_atk as i32;
        let spent_mag_def = self.attributes.mag_def as i32 - base.mag_def as i32;
        let spent_range = self.attributes.range as i32 - base.range as i32;
        let spent_rate = self.attributes.rate as i32 - base.rate as i32;
        spent_atk + spent_def + spent_mag_atk + spent_mag_def + spent_range + spent_rate
    }

    pub fn available_core_points(&self) -> i32 {
        self.total_core_points() as i32 - self.spent_core_points()
    }

    pub fn available_attribute_points(&self) -> i32 {
        self.total_attribute_points() as i32 - self.spent_attribute_points()
    }
}

#[account]
pub struct CharacterMintState {
    pub bump: u8,
    pub next_id: u64,
}

impl CharacterMintState {
    pub const LEN: usize = 8 /* discriminator */ + 1 /* bump */ + 8 /* next_id */;

    pub fn initialize(&mut self, bump: u8) {
        self.bump = bump;
        self.next_id = 1;
    }
}
