use crate::common::{context::TestContext, conversions};
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use arising::character::{
    Character, CharacterMintState, CivilizationMintConfig, CHARACTER_MINT_STATE_SEED,
    CHARACTER_SEED,
};
use arising::codex::{Civilization, CoreStats, Gender};
use solana_keypair::Keypair;

pub fn create_mint_state(ctx: &mut TestContext) -> AnchorPubkey {
    let (mint_state, bump) =
        AnchorPubkey::find_program_address(&[CHARACTER_MINT_STATE_SEED], &arising::ID);
    let payer = conversions::keypair_pubkey(ctx.payer());
    let account = CharacterMintState {
        authority: payer,
        bump,
        collection_mint: AnchorPubkey::new_unique(),
        collection_metadata: AnchorPubkey::new_unique(),
        collection_master_edition: AnchorPubkey::new_unique(),
        civilization_mints: [CivilizationMintConfig::new(); Civilization::COUNT],
    };
    ctx.store_anchor_account(mint_state, arising::ID, &account, CharacterMintState::LEN);
    mint_state
}

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
        energy: arising::character::BASE_ENERGY,
        last_energy_refill: 0,
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
