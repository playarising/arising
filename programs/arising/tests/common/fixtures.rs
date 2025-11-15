//! Builders for PDA accounts needed during LiteSVM tests.

use crate::common::tokens;
use crate::common::{context::TestContext, conversions};
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use anchor_spl::associated_token::get_associated_token_address;
use arising::character::{
    Character, CharacterEquipment, CharacterMintState, CivilizationMintConfig,
    CHARACTER_MINT_STATE_SEED, CHARACTER_SEED,
};
use arising::codex::{Attributes, Civilization, CoreStats, Gender};
use solana_keypair::Keypair;

pub struct MintStateFixture {
    pub mint_state: AnchorPubkey,
    pub usdc_mint: AnchorPubkey,
    pub usdc_treasury: AnchorPubkey,
}

/// Creates and stores the mint state PDA with default civilization caps.
pub fn create_mint_state(ctx: &mut TestContext) -> MintStateFixture {
    let (mint_state, bump) =
        AnchorPubkey::find_program_address(&[CHARACTER_MINT_STATE_SEED], &arising::ID);
    let payer = conversions::keypair_pubkey(ctx.payer());
    let usdc_mint = tokens::create_spl_mint(ctx, payer, 6);
    let usdc_treasury = get_associated_token_address(&mint_state, &usdc_mint);
    tokens::write_token_account(ctx, usdc_treasury, usdc_mint, mint_state, 0);
    let account = CharacterMintState {
        authority: payer,
        bump,
        collection_mint: AnchorPubkey::new_unique(),
        collection_metadata: AnchorPubkey::new_unique(),
        collection_master_edition: AnchorPubkey::new_unique(),
        usdc_mint,
        usdc_treasury,
        civilization_mints: [CivilizationMintConfig::new(); Civilization::COUNT],
    };
    ctx.store_anchor_account(mint_state, arising::ID, &account, CharacterMintState::LEN);
    MintStateFixture {
        mint_state,
        usdc_mint,
        usdc_treasury,
    }
}

/// Writes a character account matching the PDA derivation used on-chain.
pub fn create_character(
    ctx: &mut TestContext,
    civilization: Civilization,
    character_id: u16,
    authority: &Keypair,
    gender: Gender,
    stats_override: Option<CoreStats>,
) -> AnchorPubkey {
    let (character, bump) = AnchorPubkey::find_program_address(
        &[
            CHARACTER_SEED,
            &[civilization as u8],
            &character_id.to_le_bytes(),
        ],
        &arising::ID,
    );

    let stats = stats_override.unwrap_or_else(|| civilization.base_stats());
    let account = Character {
        authority: conversions::keypair_pubkey(authority),
        bump,
        civilization,
        civilization_character_id: character_id,
        gender,
        class: None,
        experience: 0,
        stats,
        attributes: civilization.base_attributes(),
        equipment_attributes: Attributes::zero(),
        equipment: CharacterEquipment::default(),
        energy: arising::character::BASE_ENERGY,
        last_energy_refill: 0,
        energy_passes: arising::character::EnergyPassLedger::default(),
        current_quest: None,
        current_recipe: None,
        nft_mint: AnchorPubkey::default(),
        nft_metadata: AnchorPubkey::default(),
        nft_master_edition: AnchorPubkey::default(),
        nft_token_account: AnchorPubkey::default(),
        created_at: 0,
    };

    ctx.store_anchor_account(character, arising::ID, &account, Character::LEN);
    character
}
