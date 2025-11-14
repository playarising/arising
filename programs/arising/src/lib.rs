use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program_pack::Pack, system_instruction},
};
use anchor_spl::token::{self, spl_token, Mint, Token, TokenAccount};

pub mod character;
pub mod codex;
pub mod collection;
pub mod errors;
pub mod level_codex;
pub mod nft;

use character::{
    Character, CharacterMintState, CharacterQuestState, CharacterRecipeState,
    CHARACTER_MINT_STATE_SEED, CHARACTER_NFT_MINT_SEED, CHARACTER_NFT_TOKEN_SEED, CHARACTER_SEED,
    COLLECTION_MINT_PDA_SEED, COLLECTION_TOKEN_PDA_SEED,
};
use codex::{
    quest_by_id, recipe_by_id, Attributes, CharacterClass, Civilization, CoreStats, Gender,
    PassiveEffect, QuestReward, QuestType, RecipeInput, RecipeOutput, RecipeType, ResourceKind,
};
use collection::initialize_collection;
use errors::ArisingError;
use nft::{mint_character_nft, CharacterMetadataArgs};

declare_id!("FLTa6pSm3WPWNPrqVu1pCuAmjgmVJkhDvyxMcSPSAfLA");

const RESOURCE_MINT_SEED: &[u8] = b"resource_mint";

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
        character.current_quest = None;
        character.current_recipe = None;

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

    pub fn start_quest(
        ctx: Context<ModifyCharacterOwned>,
        civilization: Civilization,
        character_id: u16,
        quest_id: u16,
    ) -> Result<()> {
        let quest = quest_by_id(quest_id).ok_or(ArisingError::InvalidQuest)?;
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        character.ensure_min_level(quest.level_required)?;
        require!(
            character.has_required_stats(quest.minimum_stats),
            ArisingError::QuestRequirementsNotMet
        );

        let now = Clock::get()?.unix_timestamp;
        let energy_cost = quest.energy_cost(character.stats);
        character.consume_energy(energy_cost)?;
        character.start_quest(quest.id, quest.cooldown_seconds, now)?;
        let quest_state = character.ensure_active_quest()?;

        emit!(QuestStartedEvent {
            civilization,
            character_id,
            authority: character.authority,
            quest_id: quest.id,
            quest_type: quest.quest_type,
            quest_name: quest.name.to_string(),
            ready_at: quest_state.ready_at,
            energy_spent: energy_cost,
        });

        Ok(())
    }

    pub fn claim_quest<'info>(
        ctx: Context<'_, '_, 'info, 'info, ClaimQuest<'info>>,
        civilization: Civilization,
        character_id: u16,
        quest_id: u16,
    ) -> Result<()> {
        let quest = quest_by_id(quest_id).ok_or(ArisingError::InvalidQuest)?;
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        let progress = character.ensure_active_quest()?;
        require_eq!(progress.quest_id, quest.id, ArisingError::QuestMismatch);

        let now = Clock::get()?.unix_timestamp;
        require!(now >= progress.ready_at, ArisingError::QuestNotReady);

        let mut experience_reward: u64 = 0;
        let mut resource_rewards: Vec<ResourceRewardEvent> = Vec::new();
        let mut remaining_accounts = ctx.remaining_accounts.iter();

        for reward in quest.rewards.iter().copied() {
            match reward {
                QuestReward::Experience { amount } => {
                    character.grant_experience(amount)?;
                    experience_reward = experience_reward.saturating_add(amount);
                }
                QuestReward::Resource { resource, amount } => {
                    let reward_event = mint_resource_reward(
                        &ctx.accounts.mint_state,
                        &ctx.accounts.token_program,
                        &ctx.accounts.system_program,
                        &ctx.accounts.rent,
                        &ctx.accounts.authority,
                        ctx.program_id,
                        ctx.bumps.mint_state,
                        resource,
                        amount,
                        &mut remaining_accounts,
                    )?;
                    resource_rewards.push(reward_event);
                }
            }
        }

        character.clear_quest()?;

        emit!(QuestClaimedEvent {
            civilization,
            character_id,
            authority: character.authority,
            quest_id: quest.id,
            quest_type: quest.quest_type,
            quest_name: quest.name.to_string(),
            experience_reward,
            resource_rewards,
        });

        Ok(())
    }

    pub fn start_recipe<'info>(
        ctx: Context<'_, '_, 'info, 'info, ProcessRecipe<'info>>,
        civilization: Civilization,
        character_id: u16,
        recipe_id: u16,
    ) -> Result<()> {
        let recipe = recipe_by_id(recipe_id).ok_or(ArisingError::InvalidRecipe)?;
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        character.ensure_min_level(recipe.level_required)?;
        require!(
            character.has_required_stats(recipe.minimum_stats),
            ArisingError::RecipeRequirementsNotMet
        );

        let now = Clock::get()?.unix_timestamp;
        let energy_cost = recipe.energy_cost(character.stats);
        character.consume_energy(energy_cost)?;

        let mut consumed_resources: Vec<ResourceCostEvent> = Vec::new();
        let mut remaining_accounts = ctx.remaining_accounts.iter();
        match recipe.input {
            RecipeInput::Forge {
                raw_material,
                amount,
            } => {
                let cost = consume_resource_from_user(
                    ctx.program_id,
                    &ctx.accounts.token_program,
                    &ctx.accounts.authority,
                    raw_material,
                    amount,
                    &mut remaining_accounts,
                )?;
                consumed_resources.push(cost);
            }
            RecipeInput::Craft {
                base_material,
                base_amount,
                gold_amount,
            } => {
                let base_cost = consume_resource_from_user(
                    ctx.program_id,
                    &ctx.accounts.token_program,
                    &ctx.accounts.authority,
                    base_material,
                    base_amount,
                    &mut remaining_accounts,
                )?;
                consumed_resources.push(base_cost);
                let gold_cost = consume_resource_from_user(
                    ctx.program_id,
                    &ctx.accounts.token_program,
                    &ctx.accounts.authority,
                    ResourceKind::Gold,
                    gold_amount,
                    &mut remaining_accounts,
                )?;
                consumed_resources.push(gold_cost);
            }
        }

        character.start_recipe(recipe.id, recipe.cooldown_seconds, now)?;
        let recipe_state = character.ensure_active_recipe()?;

        emit!(RecipeStartedEvent {
            civilization,
            character_id,
            authority: character.authority,
            recipe_id: recipe.id,
            recipe_type: recipe.recipe_type,
            recipe_name: recipe.name.to_string(),
            ready_at: recipe_state.ready_at,
            energy_spent: energy_cost,
            resources_consumed: consumed_resources,
        });

        Ok(())
    }

    pub fn claim_recipe<'info>(
        ctx: Context<'_, '_, 'info, 'info, ProcessRecipe<'info>>,
        civilization: Civilization,
        character_id: u16,
        recipe_id: u16,
    ) -> Result<()> {
        let recipe = recipe_by_id(recipe_id).ok_or(ArisingError::InvalidRecipe)?;
        let character = &mut ctx.accounts.character;
        character.ensure_identity(civilization, character_id)?;
        character.ensure_authority(ctx.accounts.authority.key())?;
        let progress = character.ensure_active_recipe()?;
        require_eq!(progress.recipe_id, recipe.id, ArisingError::RecipeMismatch);

        let now = Clock::get()?.unix_timestamp;
        require!(now >= progress.ready_at, ArisingError::RecipeNotReady);

        let mut outputs: Vec<ResourceRewardEvent> = Vec::new();
        let mut remaining_accounts = ctx.remaining_accounts.iter();
        match recipe.output {
            RecipeOutput::Resource { resource, amount } => {
                let event = mint_resource_reward(
                    &ctx.accounts.mint_state,
                    &ctx.accounts.token_program,
                    &ctx.accounts.system_program,
                    &ctx.accounts.rent,
                    &ctx.accounts.authority,
                    ctx.program_id,
                    ctx.bumps.mint_state,
                    resource,
                    amount,
                    &mut remaining_accounts,
                )?;
                outputs.push(event);
            }
        }

        character.clear_recipe()?;

        emit!(RecipeClaimedEvent {
            civilization,
            character_id,
            authority: character.authority,
            recipe_id: recipe.id,
            recipe_type: recipe.recipe_type,
            recipe_name: recipe.name.to_string(),
            resource_rewards: outputs,
        });

        Ok(())
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
            current_recipe: character.current_recipe,
            current_quest: character.current_quest,
        });

        Ok(())
    }
}

fn mint_resource_reward<'info>(
    mint_state: &Account<'info, CharacterMintState>,
    token_program: &Program<'info, Token>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    authority: &Signer<'info>,
    program_id: &Pubkey,
    mint_state_bump: u8,
    resource: ResourceKind,
    amount: u64,
    remaining_accounts: &mut std::slice::Iter<'info, AccountInfo<'info>>,
) -> Result<ResourceRewardEvent> {
    let mint_info = remaining_accounts
        .next()
        .ok_or(ArisingError::MissingRewardAccounts)?;
    let destination_info = remaining_accounts
        .next()
        .ok_or(ArisingError::MissingRewardAccounts)?;

    let (expected_mint, mint_bump) =
        Pubkey::find_program_address(&[RESOURCE_MINT_SEED, resource.seed_bytes()], program_id);
    require_keys_eq!(
        mint_info.key(),
        expected_mint,
        ArisingError::InvalidResourceMint
    );

    if mint_info.data_is_empty() {
        initialize_resource_mint(
            mint_state,
            token_program,
            system_program,
            rent,
            authority,
            mint_info,
            resource,
            mint_bump,
        )?;
    }

    let destination_account = Account::<TokenAccount>::try_from(destination_info)
        .map_err(|_| ArisingError::InvalidResourceAccount)?;
    require_keys_eq!(
        destination_account.owner,
        authority.key(),
        ArisingError::InvalidResourceAccount
    );
    require_keys_eq!(
        destination_account.mint,
        mint_info.key(),
        ArisingError::InvalidResourceAccount
    );

    let signer_seeds: &[&[u8]] = &[CHARACTER_MINT_STATE_SEED, &[mint_state_bump]];

    token::mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::MintTo {
                mint: mint_info.clone(),
                to: destination_account.to_account_info(),
                authority: mint_state.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )?;

    Ok(ResourceRewardEvent {
        resource,
        amount,
        destination: destination_account.key(),
    })
}

fn initialize_resource_mint<'info>(
    mint_state: &Account<'info, CharacterMintState>,
    token_program: &Program<'info, Token>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    authority: &Signer<'info>,
    mint_info: &AccountInfo<'info>,
    resource: ResourceKind,
    mint_bump: u8,
) -> Result<()> {
    let rent_lamports = rent.minimum_balance(spl_token::state::Mint::LEN);
    let create_ix = system_instruction::create_account(
        &authority.key(),
        mint_info.key,
        rent_lamports,
        spl_token::state::Mint::LEN as u64,
        &token_program.key(),
    );
    invoke_signed(
        &create_ix,
        &[
            authority.to_account_info(),
            mint_info.clone(),
            system_program.to_account_info(),
        ],
        &[&[RESOURCE_MINT_SEED, resource.seed_bytes(), &[mint_bump]]],
    )?;

    token::initialize_mint(
        CpiContext::new(
            token_program.to_account_info(),
            token::InitializeMint {
                mint: mint_info.clone(),
                rent: rent.to_account_info(),
            },
        ),
        resource.decimals(),
        &mint_state.key(),
        Some(&mint_state.key()),
    )?;

    Ok(())
}

fn consume_resource_from_user<'info>(
    program_id: &Pubkey,
    token_program: &Program<'info, Token>,
    authority: &Signer<'info>,
    resource: ResourceKind,
    amount: u64,
    remaining_accounts: &mut std::slice::Iter<'info, AccountInfo<'info>>,
) -> Result<ResourceCostEvent> {
    let mint_info = remaining_accounts
        .next()
        .ok_or(ArisingError::MissingRecipeAccounts)?;
    let source_info = remaining_accounts
        .next()
        .ok_or(ArisingError::MissingRecipeAccounts)?;

    let (expected_mint, _) =
        Pubkey::find_program_address(&[RESOURCE_MINT_SEED, resource.seed_bytes()], program_id);
    require_keys_eq!(
        mint_info.key(),
        expected_mint,
        ArisingError::InvalidResourceMint
    );

    let source_account = Account::<TokenAccount>::try_from(source_info)
        .map_err(|_| ArisingError::InvalidResourceAccount)?;
    require_keys_eq!(
        source_account.owner,
        authority.key(),
        ArisingError::InvalidResourceAccount
    );
    require_keys_eq!(
        source_account.mint,
        mint_info.key(),
        ArisingError::InvalidResourceAccount
    );

    token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            token::Burn {
                mint: mint_info.clone(),
                from: source_info.clone(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(ResourceCostEvent { resource, amount })
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
pub struct ClaimQuest<'info> {
    #[account(
        mut,
        seeds = [CHARACTER_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [CHARACTER_MINT_STATE_SEED],
        bump
    )]
    pub mint_state: Account<'info, CharacterMintState>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(civilization: Civilization, character_id: u16)]
pub struct ProcessRecipe<'info> {
    #[account(
        mut,
        seeds = [CHARACTER_SEED, &[civilization as u8], &character_id.to_le_bytes()],
        bump
    )]
    pub character: Account<'info, Character>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [CHARACTER_MINT_STATE_SEED],
        bump
    )]
    pub mint_state: Account<'info, CharacterMintState>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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
    pub current_recipe: Option<CharacterRecipeState>,
    pub current_quest: Option<CharacterQuestState>,
}

#[event]
pub struct QuestStartedEvent {
    pub civilization: Civilization,
    pub character_id: u16,
    pub authority: Pubkey,
    pub quest_id: u16,
    pub quest_type: QuestType,
    pub quest_name: String,
    pub ready_at: i64,
    pub energy_spent: u16,
}

#[event]
pub struct QuestClaimedEvent {
    pub civilization: Civilization,
    pub character_id: u16,
    pub authority: Pubkey,
    pub quest_id: u16,
    pub quest_type: QuestType,
    pub quest_name: String,
    pub experience_reward: u64,
    pub resource_rewards: Vec<ResourceRewardEvent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResourceRewardEvent {
    pub resource: ResourceKind,
    pub amount: u64,
    pub destination: Pubkey,
}

#[event]
pub struct RecipeStartedEvent {
    pub civilization: Civilization,
    pub character_id: u16,
    pub authority: Pubkey,
    pub recipe_id: u16,
    pub recipe_type: RecipeType,
    pub recipe_name: String,
    pub ready_at: i64,
    pub energy_spent: u16,
    pub resources_consumed: Vec<ResourceCostEvent>,
}

#[event]
pub struct RecipeClaimedEvent {
    pub civilization: Civilization,
    pub character_id: u16,
    pub authority: Pubkey,
    pub recipe_id: u16,
    pub recipe_type: RecipeType,
    pub recipe_name: String,
    pub resource_rewards: Vec<ResourceRewardEvent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResourceCostEvent {
    pub resource: ResourceKind,
    pub amount: u64,
}
