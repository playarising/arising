use anchor_lang::prelude::*;
use std::vec::Vec;

use crate::{
    codex::{
        equipment_definition, Attributes, CharacterClass, Civilization, CoreStats,
        EquipmentDefinition, EquipmentSlot, Gender, ResourceKind,
    },
    errors::ArisingError,
    level_codex::{
        level_from_experience, total_attribute_points_for_level, total_core_points_for_level,
    },
};

pub const CHARACTER_SEED: &[u8] = b"character";
pub const CHARACTER_MINT_STATE_SEED: &[u8] = b"character_mint_state";
pub const CHARACTER_NFT_MINT_SEED: &[u8] = b"character_nft_mint";
pub const CHARACTER_NFT_TOKEN_SEED: &[u8] = b"character_nft_token";
pub const COLLECTION_MINT_PDA_SEED: &[u8] = b"collection_mint";
pub const COLLECTION_TOKEN_PDA_SEED: &[u8] = b"collection_token";
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
    pub equipment_attributes: Attributes,
    pub equipment: CharacterEquipment,
    pub energy: u16,
    pub last_energy_refill: i64,
    pub current_quest: Option<CharacterQuestState>,
    pub current_recipe: Option<CharacterRecipeState>,
    pub nft_mint: Pubkey,
    pub nft_metadata: Pubkey,
    pub nft_master_edition: Pubkey,
    pub nft_token_account: Pubkey,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct CharacterQuestState {
    pub quest_id: u16,
    pub started_at: i64,
    pub ready_at: i64,
}

impl CharacterQuestState {
    pub const SIZE: usize = 2 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct CharacterRecipeState {
    pub recipe_id: u16,
    pub started_at: i64,
    pub ready_at: i64,
}

impl CharacterRecipeState {
    pub const SIZE: usize = 2 + 8 + 8;
}

const SLOT_FLAG_TWO_HANDED_PRIMARY: u8 = 1 << 0;
const SLOT_FLAG_LINKED: u8 = 1 << 1;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct EquipmentSlotState {
    pub item: Option<ResourceKind>,
    pub linked_slot: Option<EquipmentSlot>,
    pub flags: u8,
}

impl EquipmentSlotState {
    pub const SIZE: usize = 5;

    pub fn is_primary(&self) -> bool {
        self.flags & SLOT_FLAG_LINKED == 0
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct CharacterEquipment {
    pub slots: [EquipmentSlotState; EquipmentSlot::COUNT],
}

impl Default for CharacterEquipment {
    fn default() -> Self {
        Self {
            slots: [EquipmentSlotState::default(); EquipmentSlot::COUNT],
        }
    }
}

impl CharacterEquipment {
    pub const SIZE: usize = EquipmentSlot::COUNT * EquipmentSlotState::SIZE;

    fn slot_index(slot: EquipmentSlot) -> usize {
        slot as usize
    }

    fn slot_state(&self, slot: EquipmentSlot) -> &EquipmentSlotState {
        &self.slots[Self::slot_index(slot)]
    }

    fn slot_state_mut(&mut self, slot: EquipmentSlot) -> &mut EquipmentSlotState {
        &mut self.slots[Self::slot_index(slot)]
    }

    fn resolve_primary_slot(&self, slot: EquipmentSlot) -> EquipmentSlot {
        let state = self.slot_state(slot);
        if state.flags & SLOT_FLAG_LINKED != 0 {
            state.linked_slot.unwrap_or(slot)
        } else {
            slot
        }
    }

    fn clear_slot(&mut self, slot: EquipmentSlot) {
        *self.slot_state_mut(slot) = EquipmentSlotState::default();
    }

    fn remove_from_primary(&mut self, slot: EquipmentSlot) -> Option<ResourceKind> {
        let state = *self.slot_state(slot);
        if !state.is_primary() {
            return None;
        }
        let resource = state.item?;
        if let Some(linked) = state.linked_slot {
            self.clear_slot(linked);
        }
        self.clear_slot(slot);
        Some(resource)
    }

    fn assign_single(&mut self, slot: EquipmentSlot, resource: ResourceKind) {
        let state = self.slot_state_mut(slot);
        state.item = Some(resource);
        state.linked_slot = None;
        state.flags = 0;
    }

    fn assign_two_handed(
        &mut self,
        primary: EquipmentSlot,
        secondary: EquipmentSlot,
        resource: ResourceKind,
    ) {
        {
            let state = self.slot_state_mut(primary);
            state.item = Some(resource);
            state.linked_slot = Some(secondary);
            state.flags = SLOT_FLAG_TWO_HANDED_PRIMARY;
        }
        {
            let state = self.slot_state_mut(secondary);
            state.item = Some(resource);
            state.linked_slot = Some(primary);
            state.flags = SLOT_FLAG_LINKED;
        }
    }
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
        + Attributes::SIZE // equipment_attributes
        + CharacterEquipment::SIZE
        + 2 // energy
        + 8 // last_energy_refill
        + 1 // quest option tag
        + CharacterQuestState::SIZE
        + 1 // recipe option tag
        + CharacterRecipeState::SIZE
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

    fn adjust_attribute_set(target: &mut Attributes, delta: Attributes) -> Result<()> {
        target.atk = target
            .atk
            .checked_add(delta.atk)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        target.def = target
            .def
            .checked_add(delta.def)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        target.mag_atk = target
            .mag_atk
            .checked_add(delta.mag_atk)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        target.mag_def = target
            .mag_def
            .checked_add(delta.mag_def)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        target.range = target
            .range
            .checked_add(delta.range)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        target.rate = target
            .rate
            .checked_add(delta.rate)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        Ok(())
    }

    fn adjust_equipment_attributes(&mut self, delta: Attributes) -> Result<()> {
        Self::adjust_attribute_set(&mut self.equipment_attributes, delta)
    }

    pub fn total_attributes(&self) -> Result<Attributes> {
        let mut combined = self.attributes;
        Self::adjust_attribute_set(&mut combined, self.equipment_attributes)?;
        Ok(combined)
    }

    fn negate_attributes(attrs: Attributes) -> Attributes {
        Attributes::new(
            -attrs.atk,
            -attrs.def,
            -attrs.mag_atk,
            -attrs.mag_def,
            -attrs.range,
            -attrs.rate,
        )
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

    pub fn ensure_authority(&self, authority: Pubkey) -> Result<()> {
        require_keys_eq!(authority, self.authority, ArisingError::Unauthorized);
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
        if amount > 0 {
            require!(self.energy >= amount, ArisingError::InsufficientEnergy);
            self.energy = self
                .energy
                .checked_sub(amount)
                .ok_or(ArisingError::ArithmeticOverflow)?;
        }
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

    pub fn has_required_stats(&self, required: CoreStats) -> bool {
        self.stats.might >= required.might
            && self.stats.speed >= required.speed
            && self.stats.intellect >= required.intellect
    }

    pub fn equip_item(
        &mut self,
        slot: EquipmentSlot,
        definition: EquipmentDefinition,
    ) -> Result<Vec<ResourceKind>> {
        self.ensure_min_level(definition.level_required)
            .map_err(|_| ArisingError::EquipmentLevelRequirement)?;
        require!(
            definition.supports_slot(slot),
            ArisingError::InvalidEquipmentSlot
        );

        if definition.occupies_both_hands {
            require!(slot.is_hand(), ArisingError::EquipmentRequiresHandSlot);
        }

        let mut removed: Vec<ResourceKind> = Vec::new();
        let mut processed = [false; EquipmentSlot::COUNT];
        let mut slots_to_clear = Vec::new();
        slots_to_clear.push(slot);
        if definition.occupies_both_hands {
            slots_to_clear.push(
                slot.companion_hand()
                    .ok_or(ArisingError::EquipmentRequiresHandSlot)?,
            );
        }

        for target in slots_to_clear.into_iter() {
            let primary = self.equipment.resolve_primary_slot(target);
            let idx = primary as usize;
            if processed[idx] {
                continue;
            }
            if let Some(resource) = self.equipment.remove_from_primary(primary) {
                let prev_def =
                    equipment_definition(resource).ok_or(ArisingError::ItemNotEquippable)?;
                self.adjust_equipment_attributes(Self::negate_attributes(
                    prev_def.attribute_bonus,
                ))?;
                removed.push(resource);
            }
            processed[idx] = true;
        }

        if definition.occupies_both_hands {
            let secondary = slot
                .companion_hand()
                .ok_or(ArisingError::EquipmentRequiresHandSlot)?;
            self.equipment
                .assign_two_handed(slot, secondary, definition.resource);
        } else {
            self.equipment.assign_single(slot, definition.resource);
        }

        self.adjust_equipment_attributes(definition.attribute_bonus)?;
        Ok(removed)
    }

    pub fn unequip_slot(&mut self, slot: EquipmentSlot) -> Result<ResourceKind> {
        let primary = self.equipment.resolve_primary_slot(slot);
        let resource = self
            .equipment
            .remove_from_primary(primary)
            .ok_or(ArisingError::EquipmentSlotEmpty)?;

        if let Some(definition) = equipment_definition(resource) {
            self.adjust_equipment_attributes(Self::negate_attributes(definition.attribute_bonus))?;
        }

        Ok(resource)
    }

    pub fn start_quest(&mut self, quest_id: u16, cooldown_seconds: u32, now: i64) -> Result<()> {
        require!(
            self.current_quest.is_none(),
            ArisingError::QuestAlreadyInProgress
        );
        let ready_in = i64::from(cooldown_seconds);
        let ready_at = now
            .checked_add(ready_in)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.current_quest = Some(CharacterQuestState {
            quest_id,
            started_at: now,
            ready_at,
        });
        Ok(())
    }

    pub fn ensure_active_quest(&self) -> Result<&CharacterQuestState> {
        self.current_quest
            .as_ref()
            .ok_or(ArisingError::QuestNotInProgress.into())
    }

    pub fn clear_quest(&mut self) -> Result<CharacterQuestState> {
        self.current_quest
            .take()
            .ok_or(ArisingError::QuestNotInProgress.into())
    }

    pub fn start_recipe(&mut self, recipe_id: u16, cooldown_seconds: u32, now: i64) -> Result<()> {
        require!(
            self.current_recipe.is_none(),
            ArisingError::RecipeAlreadyInProgress
        );
        let ready_in = i64::from(cooldown_seconds);
        let ready_at = now
            .checked_add(ready_in)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        self.current_recipe = Some(CharacterRecipeState {
            recipe_id,
            started_at: now,
            ready_at,
        });
        Ok(())
    }

    pub fn ensure_active_recipe(&self) -> Result<&CharacterRecipeState> {
        self.current_recipe
            .as_ref()
            .ok_or(ArisingError::RecipeNotInProgress.into())
    }

    pub fn clear_recipe(&mut self) -> Result<CharacterRecipeState> {
        self.current_recipe
            .take()
            .ok_or(ArisingError::RecipeNotInProgress.into())
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

    pub fn ensure_authority(&self, authority: Pubkey) -> Result<()> {
        require_keys_eq!(authority, self.authority, ArisingError::Unauthorized);
        Ok(())
    }

    pub fn ensure_collection_binding(
        &self,
        collection_mint: &Pubkey,
        collection_metadata: &Pubkey,
        collection_master_edition: &Pubkey,
    ) -> Result<()> {
        require_keys_eq!(
            *collection_mint,
            self.collection_mint,
            ArisingError::InvalidCollectionMint
        );
        require_keys_eq!(
            *collection_metadata,
            self.collection_metadata,
            ArisingError::InvalidCollectionMetadata
        );
        require_keys_eq!(
            *collection_master_edition,
            self.collection_master_edition,
            ArisingError::InvalidCollectionMasterEdition
        );
        Ok(())
    }

    pub fn ensure_can_mint(&self, civilization: Civilization, character_id: u16) -> Result<()> {
        let track = self.track(civilization);
        require_eq!(
            character_id,
            track.next_id,
            ArisingError::InvalidCharacterIdSequence
        );
        require!(
            character_id <= track.cap,
            ArisingError::CivilizationCapReached
        );
        Ok(())
    }

    pub fn record_mint(&mut self, civilization: Civilization) -> Result<()> {
        let track = self.track_mut(civilization);
        track.next_id = track
            .next_id
            .checked_add(1)
            .ok_or(ArisingError::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn track_mut(&mut self, civilization: Civilization) -> &mut CivilizationMintConfig {
        &mut self.civilization_mints[civilization.as_index()]
    }

    pub fn track(&self, civilization: Civilization) -> &CivilizationMintConfig {
        &self.civilization_mints[civilization.as_index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        codex::{
            equipment_definition, Attributes, CharacterClass, Civilization, EquipmentSlot, Gender,
            ResourceKind,
        },
        level_codex::{LEVEL_CAP, LEVEL_CURVE},
    };
    use anchor_lang::error::Error as AnchorError;

    fn grant_levels(character: &mut Character, target_level: u8) {
        assert!(
            target_level >= 1 && target_level <= LEVEL_CAP,
            "target level must be within curve"
        );
        while character.current_level() < target_level {
            let current_level = character.current_level();
            let curve_index = (current_level - 1) as usize;
            character
                .grant_experience(LEVEL_CURVE[curve_index].xp_to_next as u64)
                .expect("experience applied");
        }
    }

    fn sample_character(civilization: Civilization) -> Character {
        Character {
            authority: Pubkey::default(),
            bump: 0,
            civilization,
            civilization_character_id: 1,
            gender: Gender::Male,
            class: None,
            experience: 0,
            stats: civilization.base_stats(),
            attributes: civilization.base_attributes(),
            equipment_attributes: Attributes::zero(),
            equipment: CharacterEquipment::default(),
            energy: BASE_ENERGY,
            last_energy_refill: 0,
            current_quest: None,
            current_recipe: None,
            nft_mint: Pubkey::default(),
            nft_metadata: Pubkey::default(),
            nft_master_edition: Pubkey::default(),
            nft_token_account: Pubkey::default(),
            created_at: 0,
        }
    }

    fn assert_error(err: AnchorError, expected: ArisingError) {
        assert_eq!(err, AnchorError::from(expected));
    }

    #[test]
    fn level_curve_is_respected() {
        let mut character = sample_character(Civilization::Tarki);
        assert_eq!(character.current_level(), 1);

        for entry in LEVEL_CURVE.iter() {
            assert_eq!(
                character.current_level(),
                entry.level,
                "character level should match curve entry before applying XP"
            );
            character
                .grant_experience(entry.xp_to_next as u64)
                .expect("xp granted");
            let expected_level = (entry.level + 1).min(LEVEL_CAP);
            assert_eq!(
                character.current_level(),
                expected_level,
                "character should reach next level when curve XP is applied"
            );
        }

        assert_eq!(character.current_level(), LEVEL_CAP);
    }

    #[test]
    fn grant_experience_updates_energy_capacity() {
        let mut character = sample_character(Civilization::Ard);
        character.energy = Character::capacity_for_level(1);
        character
            .grant_experience(LEVEL_CURVE[0].xp_to_next as u64)
            .expect("level up");
        assert_eq!(character.current_level(), 2);
        assert_eq!(character.energy, Character::capacity_for_level(2));

        let mut multi_level = sample_character(Civilization::Ard);
        multi_level.energy = Character::capacity_for_level(1) - 2;
        let xp_to_level_three = (LEVEL_CURVE[0].xp_to_next + LEVEL_CURVE[1].xp_to_next) as u64;
        multi_level
            .grant_experience(xp_to_level_three)
            .expect("multi level up");
        assert_eq!(multi_level.current_level(), 3);
        let gained = Character::capacity_for_level(3) - Character::capacity_for_level(1);
        assert_eq!(
            multi_level.energy,
            Character::capacity_for_level(1) - 2 + gained
        );
    }

    #[test]
    fn allocate_core_stats_requires_available_points() {
        let mut character = sample_character(Civilization::Hartenn);
        let err = character
            .allocate_core_stats(CoreStats::new(1, 0, 0))
            .unwrap_err();
        assert_error(err, ArisingError::InsufficientCorePoints);

        character
            .grant_experience(LEVEL_CURVE[0].xp_to_next as u64)
            .expect("gain first level");
        assert_eq!(character.available_core_points(), 1);
        character
            .allocate_core_stats(CoreStats::new(1, 0, 0))
            .expect("spend core point");
        assert_eq!(character.available_core_points(), 0);
        assert_eq!(
            character.stats.might,
            Civilization::Hartenn.base_stats().might + 1
        );
    }

    #[test]
    fn refill_energy_enforces_cooldown() {
        let mut character = sample_character(Civilization::Zhand);
        character.energy = 0;
        character.last_energy_refill = 0;
        let err = character
            .refill_energy(ENERGY_REFILL_COOLDOWN_SECS - 1)
            .unwrap_err();
        assert_error(err, ArisingError::EnergyRefillCooldown);
        character
            .refill_energy(ENERGY_REFILL_COOLDOWN_SECS)
            .expect("cooldown satisfied");
        assert_eq!(character.energy, character.energy_capacity());
    }

    #[test]
    fn quest_and_recipe_state_machine_enforces_exclusivity() {
        let mut character = sample_character(Civilization::Ard);
        character.start_quest(7, 10, 1).expect("should start quest");
        assert!(character.current_quest.is_some());
        let err = character.start_quest(8, 20, 2).unwrap_err();
        assert_error(err, ArisingError::QuestAlreadyInProgress);
        let quest_state = character.clear_quest().expect("quest cleared");
        assert_eq!(quest_state.ready_at, 11);
        assert!(character.current_quest.is_none());

        character
            .start_recipe(3, 15, 5)
            .expect("should start recipe");
        let err = character.start_recipe(4, 25, 6).unwrap_err();
        assert_error(err, ArisingError::RecipeAlreadyInProgress);
        character.clear_recipe().expect("recipe cleared");
        assert!(character.current_recipe.is_none());
    }

    #[test]
    fn allocate_attributes_validates_input() {
        let mut character = sample_character(Civilization::Shinkari);
        let err = character
            .allocate_attributes(Attributes::new(-1, 0, 0, 0, 0, 0))
            .unwrap_err();
        assert_error(err, ArisingError::NegativeAllocation);

        let err = character
            .allocate_attributes(Attributes::zero())
            .unwrap_err();
        assert_error(err, ArisingError::EmptyAllocation);

        grant_levels(&mut character, 6);
        assert_eq!(character.available_attribute_points(), 1);
        character
            .allocate_attributes(Attributes::new(1, 0, 0, 0, 0, 0))
            .expect("spend attribute point");
        assert_eq!(
            character.attributes.atk,
            Civilization::Shinkari.base_attributes().atk + 1
        );

        let err = character
            .allocate_attributes(Attributes::new(1, 1, 0, 0, 0, 0))
            .unwrap_err();
        assert_error(err, ArisingError::InsufficientAttributePoints);
    }

    #[test]
    fn consume_energy_prevents_overdraw() {
        let mut character = sample_character(Civilization::Hartenn);
        character.energy = 5;
        character
            .consume_energy(0)
            .expect("zero-cost actions should pass");
        assert_eq!(character.energy, 5);
        character.consume_energy(3).expect("spend some energy");
        assert_eq!(character.energy, 2);
        let err = character.consume_energy(3).unwrap_err();
        assert_error(err, ArisingError::InsufficientEnergy);
        assert_eq!(character.energy, 2, "energy should remain unchanged");
    }

    #[test]
    fn ensure_active_quest_and_recipe_require_state() {
        let mut character = sample_character(Civilization::Zhand);
        let err = character.ensure_active_quest().unwrap_err();
        assert_error(err, ArisingError::QuestNotInProgress);
        let err = character.clear_quest().unwrap_err();
        assert_error(err, ArisingError::QuestNotInProgress);

        character
            .start_quest(1, 5, 10)
            .expect("start quest for state check");
        character.ensure_active_quest().expect("quest present");
        character.clear_quest().expect("quest cleared");

        let err = character.ensure_active_recipe().unwrap_err();
        assert_error(err, ArisingError::RecipeNotInProgress);
        let err = character.clear_recipe().unwrap_err();
        assert_error(err, ArisingError::RecipeNotInProgress);

        character
            .start_recipe(1, 5, 20)
            .expect("start recipe for state check");
        character.ensure_active_recipe().expect("recipe present");
        character.clear_recipe().expect("recipe cleared");
    }

    #[test]
    fn apply_class_choice_validates_civilization_and_duplicates() {
        let mut character = sample_character(Civilization::Ard);
        let err = character
            .apply_class_choice(CharacterClass::HartennEngineer)
            .unwrap_err();
        assert_error(err, ArisingError::InvalidClassForCivilization);

        character
            .apply_class_choice(CharacterClass::ArdKnight)
            .expect("valid class choice");
        assert_eq!(character.class, Some(CharacterClass::ArdKnight));

        let err = character
            .apply_class_choice(CharacterClass::ArdTemplar)
            .unwrap_err();
        assert_error(err, ArisingError::ClassAlreadyChosen);
    }

    #[test]
    fn equipping_items_swaps_existing_slots() {
        let mut character = sample_character(Civilization::Ard);
        let sword = equipment_definition(ResourceKind::WoodenSword).unwrap();
        let shield = equipment_definition(ResourceKind::WoodenShield).unwrap();
        let greatsword = equipment_definition(ResourceKind::TrainingGreatsword).unwrap();

        grant_levels(&mut character, 3);

        character
            .equip_item(EquipmentSlot::RightHand, sword)
            .expect("equip sword");
        assert_eq!(character.equipment_attributes.atk, 2);

        character
            .equip_item(EquipmentSlot::LeftHand, shield)
            .expect("equip shield");
        assert_eq!(character.equipment_attributes.def, 2);

        let removed = character
            .equip_item(EquipmentSlot::LeftHand, greatsword)
            .expect("equip two handed");
        assert_eq!(removed.len(), 2);
        assert!(removed.contains(&ResourceKind::WoodenSword));
        assert!(removed.contains(&ResourceKind::WoodenShield));
        assert_eq!(character.equipment_attributes.atk, 4);
        assert_eq!(character.equipment_attributes.def, 0);
    }

    #[test]
    fn unequipping_two_handed_from_secondary_slot_clears_item() {
        let mut character = sample_character(Civilization::Ard);
        let greatsword = equipment_definition(ResourceKind::TrainingGreatsword).unwrap();
        grant_levels(&mut character, 3);
        character
            .equip_item(EquipmentSlot::RightHand, greatsword)
            .expect("equip two handed");
        let removed = character
            .unequip_slot(EquipmentSlot::LeftHand)
            .expect("unequip from off hand");
        assert_eq!(removed, ResourceKind::TrainingGreatsword);
        assert!(character.equipment.slots[EquipmentSlot::LeftHand as usize]
            .item
            .is_none());
        assert!(character.equipment.slots[EquipmentSlot::RightHand as usize]
            .item
            .is_none());
        assert_eq!(character.equipment_attributes.atk, 0);
    }

    #[test]
    fn equipment_respects_level_requirement() {
        let mut character = sample_character(Civilization::Ard);
        let greatsword = equipment_definition(ResourceKind::TrainingGreatsword).unwrap();
        let err = character
            .equip_item(EquipmentSlot::RightHand, greatsword)
            .unwrap_err();
        assert_error(err, ArisingError::EquipmentLevelRequirement);

        grant_levels(&mut character, 3);
        character
            .equip_item(EquipmentSlot::RightHand, greatsword)
            .expect("level requirement met");
    }
}
