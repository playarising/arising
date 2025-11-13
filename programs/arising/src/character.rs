use anchor_lang::prelude::*;

use crate::{
    codex::{Attributes, CharacterClass, Civilization, CoreStats, Gender},
    errors::ArisingError,
    level_codex::{
        level_from_experience, total_attribute_points_for_level, total_core_points_for_level,
    },
};

pub const CHARACTER_SEED: &[u8] = b"character";
pub const CHARACTER_MINT_STATE_SEED: &[u8] = b"character_mint_state";
pub const CHARACTER_NFT_MINT_SEED: &[u8] = b"character_nft_mint";
pub const CHARACTER_NFT_TOKEN_SEED: &[u8] = b"character_nft_token";
pub const COLLECTION_MINT_SEED: &[u8] = b"collection_mint";
pub const COLLECTION_TOKEN_SEED: &[u8] = b"collection_token";
pub const BASE_ENERGY: u16 = 10;
pub const ENERGY_REFILL_COOLDOWN_SECS: i64 = 24 * 60 * 60;

#[account]
pub struct Character {
    pub authority: Pubkey,
    pub bump: u8,
    pub civilization: Civilization,
    pub civilization_character_id: u16,
    pub gender: Gender,
    pub class: Option<CharacterClass>,
    pub experience: u64,
    pub stats: CoreStats,
    pub attributes: Attributes,
    pub energy: u16,
    pub last_energy_refill: i64,
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
        + 1 // class option tag
        + 1 // class value
        + 8 // experience
        + CoreStats::SIZE
        + Attributes::SIZE
        + 2 // energy
        + 8 // last_energy_refill
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

    pub fn energy_capacity(&self) -> u16 {
        Self::capacity_for_level(self.current_level())
    }

    fn capacity_for_level(level: u8) -> u16 {
        BASE_ENERGY + level.saturating_sub(1) as u16
    }

    pub fn ensure_identity(&self, civilization: Civilization, character_id: u16) -> Result<()> {
        require_eq!(
            self.civilization,
            civilization,
            ArisingError::InvalidCharacterAccount
        );
        require_eq!(
            self.civilization_character_id,
            character_id,
            ArisingError::InvalidCharacterAccount
        );
        Ok(())
    }

    pub fn grant_experience(&mut self, amount: u64) -> Result<()> {
        let previous_level = self.current_level();
        self.experience = self
            .experience
            .checked_add(amount)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        let new_level = self.current_level();
        if new_level > previous_level {
            let prev_capacity = Self::capacity_for_level(previous_level);
            let new_capacity = self.energy_capacity();
            if new_capacity > prev_capacity {
                let gain = new_capacity - prev_capacity;
                let increased = self
                    .energy
                    .checked_add(gain)
                    .ok_or(ArisingError::ArithmeticOverflow)?;
                self.energy = increased.min(new_capacity);
            }
        }
        Ok(())
    }

    pub fn ensure_min_level(&self, min_level: u8) -> Result<()> {
        require!(
            self.current_level() >= min_level,
            ArisingError::ClassLevelRequirement
        );
        Ok(())
    }

    fn add_core_stats(&mut self, delta: CoreStats) -> Result<()> {
        self.stats.might = self
            .stats
            .might
            .checked_add(delta.might)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.stats.speed = self
            .stats
            .speed
            .checked_add(delta.speed)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.stats.intellect = self
            .stats
            .intellect
            .checked_add(delta.intellect)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        Ok(())
    }

    fn add_attributes(&mut self, delta: Attributes) -> Result<()> {
        self.attributes.atk = self
            .attributes
            .atk
            .checked_add(delta.atk)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.attributes.def = self
            .attributes
            .def
            .checked_add(delta.def)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.attributes.mag_atk = self
            .attributes
            .mag_atk
            .checked_add(delta.mag_atk)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.attributes.mag_def = self
            .attributes
            .mag_def
            .checked_add(delta.mag_def)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.attributes.range = self
            .attributes
            .range
            .checked_add(delta.range)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.attributes.rate = self
            .attributes
            .rate
            .checked_add(delta.rate)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn allocate_core_stats(&mut self, allocation: CoreStats) -> Result<()> {
        let allocation_total =
            allocation.might as i32 + allocation.speed as i32 + allocation.intellect as i32;
        require!(allocation_total > 0, ArisingError::EmptyAllocation);

        let available = self.available_core_points();
        require!(
            allocation_total <= available,
            ArisingError::InsufficientCorePoints
        );

        self.add_core_stats(allocation)
    }

    pub fn allocate_attributes(&mut self, allocation: Attributes) -> Result<()> {
        require!(
            allocation.atk >= 0
                && allocation.def >= 0
                && allocation.mag_atk >= 0
                && allocation.mag_def >= 0
                && allocation.range >= 0
                && allocation.rate >= 0,
            ArisingError::NegativeAllocation
        );

        let allocation_total = allocation.atk as i32
            + allocation.def as i32
            + allocation.mag_atk as i32
            + allocation.mag_def as i32
            + allocation.range as i32
            + allocation.rate as i32;
        require!(allocation_total > 0, ArisingError::EmptyAllocation);

        let available = self.available_attribute_points();
        require!(
            allocation_total <= available,
            ArisingError::InsufficientAttributePoints
        );

        self.add_attributes(allocation)
    }

    pub fn apply_class_choice(&mut self, class: CharacterClass) -> Result<()> {
        require!(self.class.is_none(), ArisingError::ClassAlreadyChosen);
        require_eq!(
            class.civilization(),
            self.civilization,
            ArisingError::InvalidClassForCivilization
        );
        let bonus = class.bonuses();
        self.add_core_stats(bonus.stats)?;
        self.add_attributes(bonus.attributes)?;
        self.class = Some(class);
        Ok(())
    }

    pub fn consume_energy(&mut self, amount: u16) -> Result<()> {
        require!(amount > 0, ArisingError::InsufficientEnergy);
        require!(self.energy >= amount, ArisingError::InsufficientEnergy);
        self.energy = self
            .energy
            .checked_sub(amount)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn refill_energy(&mut self, now: i64) -> Result<()> {
        let elapsed = now
            .checked_sub(self.last_energy_refill)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        require!(
            elapsed >= ENERGY_REFILL_COOLDOWN_SECS,
            ArisingError::EnergyRefillCooldown
        );
        self.energy = self.energy_capacity();
        self.last_energy_refill = now;
        Ok(())
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
