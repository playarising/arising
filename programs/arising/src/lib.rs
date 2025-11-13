use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub mod character;
pub mod codex;
pub mod collection;
pub mod errors;
pub mod level_codex;
pub mod nft;

use character::{
    Character, CharacterMintState, CHARACTER_MINT_STATE_SEED, CHARACTER_NFT_MINT_SEED,
    CHARACTER_NFT_TOKEN_SEED, CHARACTER_SEED, COLLECTION_MINT_PDA_SEED, COLLECTION_TOKEN_PDA_SEED,
};
use codex::{Attributes, CharacterClass, Civilization, CoreStats, Gender, PassiveEffect};
use collection::initialize_collection;
use errors::ArisingError;
use nft::{mint_character_nft, CharacterMetadataArgs};

declare_id!("DQRfLQoUg3kXGLR2N7WD8iwmZzmv77C7JQNTFacQtaG3");

#[program]
pub mod arising {
    use super::*;

    pub fn initialize(
        mut ctx: Context<Initialize>,
        collection_name: String,
        collection_symbol: String,
        collection_uri: String,
    ) -> Result<()> {
        initialize_collection(
            &mut ctx.accounts,
            ctx.program_id,
            ctx.bumps.mint_state,
            ctx.bumps.collection_mint,
            ctx.bumps.collection_token_account,
            collection_name,
            collection_symbol,
            collection_uri,
        )
    }

    pub fn mint_character(
        ctx: Context<MintCharacter>,
        civilization: Civilization,
        character_id: u16,
        gender: Gender,
    ) -> Result<()> {
        let mint_state_bump = ctx.bumps.mint_state;

        {
            let mint_state = &ctx.accounts.mint_state;
            mint_state.ensure_collection_binding(
                &ctx.accounts.collection_mint.key(),
                &ctx.accounts.collection_metadata.key(),
                &ctx.accounts.collection_master_edition.key(),
            )?;
            mint_state.ensure_can_mint(civilization, character_id)?;
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
        character.class = None;
        character.experience = 0;
        character.stats = civilization.base_stats();
        character.attributes = civilization.base_attributes();
        character.nft_mint = ctx.accounts.character_mint.key();
        character.nft_metadata = ctx.accounts.metadata.key();
        character.nft_master_edition = ctx.accounts.master_edition.key();
        character.nft_token_account = ctx.accounts.character_token_account.key();
        character.created_at = Clock::get()?.unix_timestamp;
        character.energy = character.energy_capacity();
        character.last_energy_refill = character.created_at;

        ctx.accounts.mint_state.record_mint(civilization)?;

        Ok(())
    }

    pub fn update_civilization_cap(
        ctx: Context<UpdateCivilizationCap>,
        civilization: Civilization,
        new_cap: u16,
    ) -> Result<()> {
        ctx.accounts
            .mint_state
            .ensure_authority(ctx.accounts.authority.key())?;

        let track = ctx.accounts.mint_state.track_mut(civilization);
        let minted = track.next_id.saturating_sub(1);
        require!(new_cap >= minted, ArisingError::CapBelowSupply);

        track.cap = new_cap;
        Ok(())
    }

    pub fn add_experience(
        ctx: Context<ModifyCharacterExperience>,
        civilization: Civilization,
        character_id: u16,
        amount: u64,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.grant_experience(amount)
    }

    pub fn allocate_core_stats(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
        allocation: CoreStats,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        character.allocate_core_stats(allocation)
    }

    pub fn allocate_attributes(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
        allocation: Attributes,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        character.allocate_attributes(allocation)
    }

    pub fn choose_class(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
        class: CharacterClass,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        character.ensure_min_level(10)?;
        require_eq!(
            class.civilization(),
            civilization,
            ArisingError::InvalidClassForCivilization
        );
        character.apply_class_choice(class)
    }

    pub fn consume_energy(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
        amount: u16,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        character.consume_energy(amount)
    }

    pub fn refill_energy(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        let now = Clock::get()?.unix_timestamp;
        character.refill_energy(now)
    }

    pub fn get_character_metadata(
        ctx: Context<GetCharacterMetadata>,
        civilization: Civilization,
        character_id: u16,
    ) -> Result<()> {
        let character = &ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;

        emit!(CharacterMetadataEvent {
            civilization,
            character_id,
            authority: character.authority,
            gender: character.gender,
            level: character.current_level(),
            experience: character.experience,
            class: character.class,
            energy: character.energy,
            energy_capacity: character.energy_capacity(),
            last_energy_refill: character.last_energy_refill,
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
    /// CHECK: PDA created/owned by the program; validated via seeds and manually initialized
    #[account(
        mut,
        seeds = [COLLECTION_MINT_PDA_SEED],
        bump
    )]
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK: PDA created/owned by the program; validated via seeds and manually initialized
    #[account(
        mut,
        seeds = [COLLECTION_TOKEN_PDA_SEED],
        bump
    )]
    pub collection_token_account: UncheckedAccount<'info>,
    /// CHECK: Created via token metadata CPI
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: Created via token metadata CPI
    #[account(mut)]
    pub collection_master_edition: UncheckedAccount<'info>,
    /// CHECK: Program id validated
    pub token_metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
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
pub struct ModifyCharacterExperience<'info> {
    #[account(
        mut,
        seeds = [CHARACTER_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
}

#[derive(Accounts)]
#[instruction(civilization: Civilization, character_id: u16)]
pub struct ModifyCharacterOwned<'info> {
    #[account(
        mut,
        seeds = [CHARACTER_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
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
    pub class: Option<CharacterClass>,
    pub energy: u16,
    pub energy_capacity: u16,
    pub last_energy_refill: i64,
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
