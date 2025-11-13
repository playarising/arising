use anchor_lang::{
    prelude::*,
    solana_program::{
        program::invoke_signed, program_pack::Pack, system_instruction, system_program, sysvar,
    },
};
use anchor_spl::token::{self, spl_token, Mint, Token, TokenAccount};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{CollectionDetails, Creator, DataV2},
    ID as TOKEN_METADATA_PROGRAM_ID,
};

pub mod character;
pub mod codex;
pub mod errors;
pub mod level_codex;
pub mod nft;

use character::{
    Character, CharacterMintState, CHARACTER_MINT_STATE_SEED, CHARACTER_NFT_MINT_SEED,
    CHARACTER_NFT_TOKEN_SEED, CHARACTER_SEED, COLLECTION_MINT_PDA_SEED, COLLECTION_TOKEN_PDA_SEED,
};
use codex::{Attributes, CharacterClass, Civilization, CoreStats, Gender, PassiveEffect};
use errors::ArisingError;
use nft::{master_edition_pda, metadata_pda, mint_character_nft, CharacterMetadataArgs};

declare_id!("DQRfLQoUg3kXGLR2N7WD8iwmZzmv77C7JQNTFacQtaG3");

#[program]
pub mod arising {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        collection_name: String,
        collection_symbol: String,
        collection_uri: String,
    ) -> Result<()> {
        let (expected_collection_mint, _) =
            Pubkey::find_program_address(&[COLLECTION_MINT_PDA_SEED], ctx.program_id);
        let (expected_collection_token, _) =
            Pubkey::find_program_address(&[COLLECTION_TOKEN_PDA_SEED], ctx.program_id);

        require_keys_eq!(
            ctx.accounts.collection_mint.key(),
            expected_collection_mint,
            ArisingError::InvalidCollectionMint
        );
        require_keys_eq!(
            ctx.accounts.collection_token_account.key(),
            expected_collection_token,
            ArisingError::InvalidCollectionTokenAccount
        );

        let metadata_program = Pubkey::new_from_array(TOKEN_METADATA_PROGRAM_ID.to_bytes());
        require_keys_eq!(
            ctx.accounts.token_metadata_program.key(),
            metadata_program,
            ArisingError::InvalidTokenMetadataProgram
        );

        let expected_metadata =
            metadata_pda(&ctx.accounts.collection_mint.key(), &metadata_program);
        let expected_master_edition =
            master_edition_pda(&ctx.accounts.collection_mint.key(), &metadata_program);

        require_keys_eq!(
            ctx.accounts.collection_metadata.key(),
            expected_metadata,
            ArisingError::InvalidCollectionMetadata
        );
        require_keys_eq!(
            ctx.accounts.collection_master_edition.key(),
            expected_master_edition,
            ArisingError::InvalidCollectionMasterEdition
        );

        let mint_state_bump = ctx.bumps.mint_state;
        let collection_mint_bump = ctx.bumps.collection_mint;
        let collection_token_bump = ctx.bumps.collection_token_account;

        let signer_seeds: &[&[&[u8]]] = &[&[CHARACTER_MINT_STATE_SEED, &[mint_state_bump]]];
        let collection_mint_signer: &[&[u8]] = &[COLLECTION_MINT_PDA_SEED, &[collection_mint_bump]];
        let collection_token_signer: &[&[u8]] =
            &[COLLECTION_TOKEN_PDA_SEED, &[collection_token_bump]];

        if ctx
            .accounts
            .collection_mint
            .to_account_info()
            .data_is_empty()
        {
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(spl_token::state::Mint::LEN);

            let create_ix = system_instruction::create_account(
                &ctx.accounts.authority.key(),
                &ctx.accounts.collection_mint.key(),
                lamports,
                spl_token::state::Mint::LEN as u64,
                &ctx.accounts.token_program.key(),
            );
            invoke_signed(
                &create_ix,
                &[
                    ctx.accounts.authority.to_account_info(),
                    ctx.accounts.collection_mint.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[collection_mint_signer],
            )?;

            token::initialize_mint(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    token::InitializeMint {
                        mint: ctx.accounts.collection_mint.to_account_info(),
                        rent: ctx.accounts.rent.to_account_info(),
                    },
                ),
                0,
                &ctx.accounts.mint_state.key(),
                Some(&ctx.accounts.mint_state.key()),
            )?;
        }

        if ctx
            .accounts
            .collection_token_account
            .to_account_info()
            .data_is_empty()
        {
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(spl_token::state::Account::LEN);

            let create_ix = system_instruction::create_account(
                &ctx.accounts.authority.key(),
                &ctx.accounts.collection_token_account.key(),
                lamports,
                spl_token::state::Account::LEN as u64,
                &ctx.accounts.token_program.key(),
            );
            invoke_signed(
                &create_ix,
                &[
                    ctx.accounts.authority.to_account_info(),
                    ctx.accounts.collection_token_account.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[collection_token_signer],
            )?;

            token::initialize_account(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeAccount {
                    account: ctx.accounts.collection_token_account.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ))?;
            token::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    token::MintTo {
                        mint: ctx.accounts.collection_mint.to_account_info(),
                        to: ctx.accounts.collection_token_account.to_account_info(),
                        authority: ctx.accounts.mint_state.to_account_info(),
                    },
                    signer_seeds,
                ),
                1,
            )?;
        }

        let metadata_data = DataV2 {
            name: collection_name,
            symbol: collection_symbol,
            uri: collection_uri,
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: ctx.accounts.mint_state.key(),
                verified: true,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        if ctx.accounts.collection_metadata.data_is_empty() {
            let metadata_ix = CreateMetadataAccountV3 {
                metadata: ctx.accounts.collection_metadata.key(),
                mint: ctx.accounts.collection_mint.key(),
                mint_authority: ctx.accounts.mint_state.key(),
                payer: ctx.accounts.authority.key(),
                update_authority: (ctx.accounts.mint_state.key(), true),
                system_program: system_program::ID,
                rent: Some(sysvar::rent::id()),
            }
            .instruction(CreateMetadataAccountV3InstructionArgs {
                data: metadata_data,
                is_mutable: false,
                collection_details: Some(CollectionDetails::V1 { size: 0 }),
            });

            let metadata_infos = [
                ctx.accounts.collection_metadata.to_account_info(),
                ctx.accounts.collection_mint.to_account_info(),
                ctx.accounts.mint_state.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.mint_state.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
            ];
            invoke_signed(&metadata_ix, &metadata_infos, signer_seeds)?;
        }

        if ctx.accounts.collection_master_edition.data_is_empty() {
            let master_ix = CreateMasterEditionV3 {
                edition: ctx.accounts.collection_master_edition.key(),
                mint: ctx.accounts.collection_mint.key(),
                update_authority: ctx.accounts.mint_state.key(),
                mint_authority: ctx.accounts.mint_state.key(),
                payer: ctx.accounts.authority.key(),
                metadata: ctx.accounts.collection_metadata.key(),
                token_program: ctx.accounts.token_program.key(),
                system_program: system_program::ID,
                rent: Some(sysvar::rent::id()),
            }
            .instruction(CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            });

            let master_infos = [
                ctx.accounts.collection_master_edition.to_account_info(),
                ctx.accounts.collection_mint.to_account_info(),
                ctx.accounts.mint_state.to_account_info(),
                ctx.accounts.mint_state.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.collection_metadata.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
            ];
            invoke_signed(&master_ix, &master_infos, signer_seeds)?;
        }

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
        let mint_state_bump = ctx.bumps.mint_state;

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
        require_keys_eq!(
            ctx.accounts.authority.key(),
            character.authority,
            ArisingError::Unauthorized
        );
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
        require_keys_eq!(
            ctx.accounts.authority.key(),
            character.authority,
            ArisingError::Unauthorized
        );
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
        require_keys_eq!(
            ctx.accounts.authority.key(),
            character.authority,
            ArisingError::Unauthorized
        );
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
        require_keys_eq!(
            ctx.accounts.authority.key(),
            character.authority,
            ArisingError::Unauthorized
        );
        character.consume_energy(amount)
    }

    pub fn refill_energy(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
    ) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        require_keys_eq!(
            ctx.accounts.authority.key(),
            character.authority,
            ArisingError::Unauthorized
        );
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
