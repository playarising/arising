use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;

pub mod character;
pub mod codex;
pub mod errors;
pub mod level_codex;
pub mod nft;

use character::{
    Character, CharacterMintState, CHARACTER_MINT_STATE_SEED, CHARACTER_NFT_MINT_SEED,
    CHARACTER_NFT_TOKEN_SEED, CHARACTER_SEED,
};
use codex::{Attributes, Civilization, CoreStats, Gender, PassiveEffect};
use errors::ArisingError;
use nft::{master_edition_pda, metadata_pda, mint_character_nft, CharacterMetadataArgs};

declare_id!("aris36Gz3rvSvgdDxwfBcveTN449VssKQipM5XJBuip");

#[program]
pub mod arising {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let metadata_program = Pubkey::new_from_array(TOKEN_METADATA_PROGRAM_ID.to_bytes());
        require_keys_eq!(
            ctx.accounts.token_metadata_program.key(),
            metadata_program,
            ArisingError::InvalidTokenMetadataProgram
        );

        let expected_metadata =
            metadata_pda(&ctx.accounts.collection_mint.key(), &metadata_program);
        require_keys_eq!(
            ctx.accounts.collection_metadata.key(),
            expected_metadata,
            ArisingError::InvalidCollectionMetadata
        );

        let expected_master_edition =
            master_edition_pda(&ctx.accounts.collection_mint.key(), &metadata_program);
        require_keys_eq!(
            ctx.accounts.collection_master_edition.key(),
            expected_master_edition,
            ArisingError::InvalidCollectionMasterEdition
        );

        ctx.accounts.mint_state.initialize(
            ctx.bumps.mint_state,
            ctx.accounts.authority.key(),
            ctx.accounts.collection_mint.key(),
            ctx.accounts.collection_metadata.key(),
            ctx.accounts.collection_master_edition.key(),
        );

        Ok(())
    }

    pub fn mint_character(
        ctx: Context<MintCharacter>,
        civilization: Civilization,
        character_id: u16,
        gender: Gender,
    ) -> Result<()> {
        // Extract bump first before any mutable borrows
        let mint_state_bump = ctx.bumps.mint_state;

        // Validate and check in a scope that drops the mutable borrow
        {
            let mint_state = &mut ctx.accounts.mint_state;

            require_keys_eq!(
                ctx.accounts.collection_mint.key(),
                mint_state.collection_mint,
                ArisingError::InvalidCollectionMint
            );
            require_keys_eq!(
                ctx.accounts.collection_metadata.key(),
                mint_state.collection_metadata,
                ArisingError::InvalidCollectionMetadata
            );
            require_keys_eq!(
                ctx.accounts.collection_master_edition.key(),
                mint_state.collection_master_edition,
                ArisingError::InvalidCollectionMasterEdition
            );

            // Validate character_id against the track
            let track = mint_state.track_mut(civilization);
            require_eq!(
                character_id,
                track.next_id,
                ArisingError::InvalidCharacterIdSequence
            );
            require!(
                character_id <= track.cap,
                ArisingError::CivilizationCapReached
            );
        }

        let metadata = CharacterMetadataArgs {
            name: format!("Arising: {}", civilization.display_name()),
            symbol: "ARISING".to_owned(),
            uri: civilization.metadata_uri(character_id),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[CHARACTER_MINT_STATE_SEED, &[mint_state_bump]]];
        mint_character_nft(&ctx.accounts, &metadata, signer_seeds)?;

        let character = &mut ctx.accounts.character;
        character.authority = ctx.accounts.authority.key();
        character.bump = ctx.bumps.character;
        character.civilization = civilization;
        character.civilization_character_id = character_id;
        character.gender = gender;
        character.experience = 0;
        character.stats = civilization.base_stats();
        character.attributes = civilization.base_attributes();
        character.nft_mint = ctx.accounts.character_mint.key();
        character.nft_metadata = ctx.accounts.metadata.key();
        character.nft_master_edition = ctx.accounts.master_edition.key();
        character.nft_token_account = ctx.accounts.character_token_account.key();
        character.created_at = Clock::get()?.unix_timestamp;

        // Increment the next_id after successfully minting
        let mint_state = &mut ctx.accounts.mint_state;
        let track = mint_state.track_mut(civilization);
        track.next_id = track
            .next_id
            .checked_add(1)
            .ok_or(ArisingError::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn update_civilization_cap(
        ctx: Context<UpdateCivilizationCap>,
        civilization: Civilization,
        new_cap: u16,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.mint_state.authority,
            ArisingError::Unauthorized
        );

        let track = ctx.accounts.mint_state.track_mut(civilization);
        let minted = track.next_id.saturating_sub(1);
        require!(new_cap >= minted, ArisingError::CapBelowSupply);

        track.cap = new_cap;
        Ok(())
    }

    pub fn get_character_metadata(
        ctx: Context<GetCharacterMetadata>,
        civilization: Civilization,
        character_id: u16,
    ) -> Result<()> {
        let character = &ctx.accounts.character;
        require_eq!(
            character.civilization,
            civilization,
            ArisingError::InvalidCharacterAccount
        );
        require_eq!(
            character.civilization_character_id,
            character_id,
            ArisingError::InvalidCharacterAccount
        );

        emit!(CharacterMetadataEvent {
            civilization,
            character_id,
            authority: character.authority,
            gender: character.gender,
            level: character.current_level(),
            experience: character.experience,
            base_stats: civilization.base_stats(),
            stats: character.stats,
            base_attributes: civilization.base_attributes(),
            attributes: character.attributes,
            available_core_points: character.available_core_points(),
            available_attribute_points: character.available_attribute_points(),
            passive: civilization.passive(),
            nft_mint: character.nft_mint,
            nft_metadata: character.nft_metadata,
            nft_master_edition: character.nft_master_edition,
            nft_token_account: character.nft_token_account,
            created_at: character.created_at,
        });

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
    /// CHECK: Collection mint validated via metadata PDA checks
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK: PDA checked in handler
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: PDA checked in handler
    pub collection_master_edition: UncheckedAccount<'info>,
    /// CHECK: Program id validated
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(civilization: Civilization, character_id: u16)]
pub struct MintCharacter<'info> {
    #[account(
        init,
        payer = authority,
        space = Character::LEN,
        seeds = [CHARACTER_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [CHARACTER_MINT_STATE_SEED],
        bump
    )]
    pub mint_state: Account<'info, CharacterMintState>,
    #[account(
        init,
        payer = authority,
        seeds = [CHARACTER_NFT_MINT_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump,
        mint::decimals = 0,
        mint::authority = mint_state,
        mint::freeze_authority = mint_state
    )]
    pub character_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        seeds = [CHARACTER_NFT_TOKEN_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump,
        token::mint = character_mint,
        token::authority = authority
    )]
    pub character_token_account: Account<'info, TokenAccount>,
    /// CHECK: PDA validated in handler
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: PDA validated in handler
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    /// CHECK: Collection mint validated against state
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK: Mutated by collection verification CPI
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: Checked against stored value
    #[account(mut)]
    pub collection_master_edition: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Validated inside mint helper
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCivilizationCap<'info> {
    #[account(
        mut,
        seeds = [CHARACTER_MINT_STATE_SEED],
        bump
    )]
    pub mint_state: Account<'info, CharacterMintState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(civilization: Civilization, character_id: u16)]
pub struct GetCharacterMetadata<'info> {
    #[account(
        seeds = [CHARACTER_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
}

#[event]
pub struct CharacterMetadataEvent {
    pub civilization: Civilization,
    pub character_id: u16,
    pub authority: Pubkey,
    pub gender: Gender,
    pub level: u8,
    pub experience: u64,
    pub base_stats: CoreStats,
    pub stats: CoreStats,
    pub base_attributes: Attributes,
    pub attributes: Attributes,
    pub available_core_points: i32,
    pub available_attribute_points: i32,
    pub passive: PassiveEffect,
    pub nft_mint: Pubkey,
    pub nft_metadata: Pubkey,
    pub nft_master_edition: Pubkey,
    pub nft_token_account: Pubkey,
    pub created_at: i64,
}
