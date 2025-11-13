use anchor_lang::prelude::*;

pub mod character;
pub mod cnft;
pub mod codex;
pub mod level_codex;

use character::{Character, CharacterMintState, CHARACTER_MINT_STATE_SEED, CHARACTER_SEED};
use cnft::{mint_character_cnft, CharacterMetadataArgs};
use codex::{Civilization, Gender};

declare_id!("81kwbZgTse8KX8nhzgsju7D2PWyrsorDxBVcTivrQoHL");

#[program]
pub mod arising {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.mint_state.initialize(ctx.bumps.mint_state);
        Ok(())
    }

    pub fn mint_character(
        ctx: Context<MintCharacter>,
        character_id: u64,
        gender: Gender,
        civilization: Civilization,
    ) -> Result<()> {
        let mint_state = &mut ctx.accounts.mint_state;

        require_eq!(
            character_id,
            mint_state.next_id,
            ArisingError::InvalidCharacterIdSequence
        );

        let metadata = CharacterMetadataArgs {
            name: format!("Arising: {}", civilization.display_name()),
            symbol: "ARISING".to_owned(),
            uri: format!("https://mints.playarising.com/metadata/{}", character_id),
        };

        let cnft_result = mint_character_cnft(
            &ctx.accounts.cnft,
            &ctx.accounts.authority,
            &metadata,
            character_id,
            &ctx.accounts.system_program,
        )?;

        let character = &mut ctx.accounts.character;

        character.authority = ctx.accounts.authority.key();
        character.bump = ctx.bumps.character;
        character.character_id = character_id;
        character.gender = gender;
        character.civilization = civilization;
        character.experience = 0;
        character.stats = civilization.base_stats();
        character.attributes = civilization.base_attributes();
        character.cnft_tree = cnft_result.merkle_tree;
        character.cnft_leaf_index = cnft_result.leaf_index;
        character.cnft_hash = cnft_result.leaf_hash;
        character.created_at = Clock::get()?.unix_timestamp;

        mint_state.next_id = mint_state
            .next_id
            .checked_add(1)
            .ok_or(ArisingError::ArithmeticOverflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = CharacterMintState::LEN,
        seeds = [CHARACTER_MINT_STATE_SEED],
        bump
    )]
    pub mint_state: Account<'info, CharacterMintState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(character_id: u64)]
pub struct MintCharacter<'info> {
    #[account(
        init,
        payer = authority,
        space = Character::LEN,
        seeds = [CHARACTER_SEED, authority.key().as_ref(), &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CHARACTER_MINT_STATE_SEED],
        bump = mint_state.bump
    )]
    pub mint_state: Account<'info, CharacterMintState>,
    pub cnft: MintCharacterCnft<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintCharacterCnft<'info> {
    /// CHECK: Validated by Bubblegum CPI
    pub tree_authority: UncheckedAccount<'info>,
    /// CHECK: Validated by Bubblegum CPI
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: Validated by Bubblegum CPI
    pub tree_delegate: UncheckedAccount<'info>,
    /// CHECK: Authorized to manage the collection
    pub collection_authority: UncheckedAccount<'info>,
    /// CHECK: Optional PDA recorded by Bubblegum
    pub collection_authority_record_pda: UncheckedAccount<'info>,
    /// CHECK: Collection mint
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK: Collection metadata
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: Collection edition
    pub collection_edition: UncheckedAccount<'info>,
    /// CHECK: Bubblegum signer PDA
    pub bubblegum_signer: UncheckedAccount<'info>,
    /// CHECK: Token metadata program required by Bubblegum CPI
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: Program addresses enforced when CNFT minting feature is enabled
    pub bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: Program addresses enforced when CNFT minting feature is enabled
    pub compression_program: UncheckedAccount<'info>,
    /// CHECK: Program addresses enforced when CNFT minting feature is enabled
    pub log_wrapper: UncheckedAccount<'info>,
}

#[error_code]
pub enum ArisingError {
    #[msg("Invalid Bubblegum program provided")]
    InvalidBubblegumProgram,
    #[msg("Invalid compression program provided")]
    InvalidCompressionProgram,
    #[msg("Invalid log wrapper program provided")]
    InvalidLogWrapperProgram,
    #[msg("Arithmetic underflow encountered")]
    ArithmeticUnderflow,
    #[msg("Arithmetic overflow encountered")]
    ArithmeticOverflow,
    #[msg("Character id must match the next sequential mint id")]
    InvalidCharacterIdSequence,
}
