use anchor_lang::prelude::*;

use crate::{
    codex::{Attributes, Civilization, CoreStats, Gender},
    level_codex::{
        level_from_experience, total_attribute_points_for_level, total_core_points_for_level,
    },
};

pub const CHARACTER_SEED: &[u8] = b"character";
pub const CHARACTER_MINT_STATE_SEED: &[u8] = b"character_mint_state";
pub const CHARACTER_NFT_MINT_SEED: &[u8] = b"character_nft_mint";
pub const CHARACTER_NFT_TOKEN_SEED: &[u8] = b"character_nft_token";

#[account]
pub struct Character {
    pub authority: Pubkey,
    pub bump: u8,
    pub civilization: Civilization,
    pub civilization_character_id: u16,
    pub gender: Gender,
    pub experience: u64,
    pub stats: CoreStats,
    pub attributes: Attributes,
    pub nft_mint: Pubkey,
    pub nft_metadata: Pubkey,
    pub nft_master_edition: Pubkey,
    pub nft_token_account: Pubkey,
    pub created_at: i64,
}

impl Character {
    pub const LEN: usize = 8  // discriminator
        + 32 // authority
        + 1 // bump
        + 1 // civilization enum
        + 2 // civilization_character_id
        + 1 // gender
        + 8 // experience
        + CoreStats::SIZE
        + Attributes::SIZE
        + 32 // nft_mint
        + 32 // nft_metadata
        + 32 // nft_master_edition
        + 32 // nft_token_account
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct CivilizationMintConfig {
    pub next_id: u16,
    pub cap: u16,
}

impl CivilizationMintConfig {
    pub const SIZE: usize = 2 + 2;

    pub const fn new() -> Self {
        Self {
            next_id: 1,
            cap: 1000,
        }
    }
}

#[account]
pub struct CharacterMintState {
    pub authority: Pubkey,
    pub bump: u8,
    pub collection_mint: Pubkey,
    pub collection_metadata: Pubkey,
    pub collection_master_edition: Pubkey,
    pub civilization_mints: [CivilizationMintConfig; Civilization::COUNT],
}

impl CharacterMintState {
    pub const LEN: usize = 8 /* discriminator */
        + 32 /* authority */
        + 1 /* bump */
        + 32 /* collection_mint */
        + 32 /* collection_metadata */
        + 32 /* collection_master_edition */
        + Civilization::COUNT * CivilizationMintConfig::SIZE;

    pub fn initialize(
        &mut self,
        bump: u8,
        authority: Pubkey,
        collection_mint: Pubkey,
        collection_metadata: Pubkey,
        collection_master_edition: Pubkey,
    ) {
        self.bump = bump;
        self.authority = authority;
        self.collection_mint = collection_mint;
        self.collection_metadata = collection_metadata;
        self.collection_master_edition = collection_master_edition;
        self.civilization_mints = [CivilizationMintConfig::new(); Civilization::COUNT];
    }

    pub fn track_mut(&mut self, civilization: Civilization) -> &mut CivilizationMintConfig {
        &mut self.civilization_mints[civilization.as_index()]
    }

    pub fn track(&self, civilization: Civilization) -> &CivilizationMintConfig {
        &self.civilization_mints[civilization.as_index()]
    }
}
