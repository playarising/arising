//! SPL-token utilities for seeding balances inside LiteSVM tests.

use crate::common::{context::TestContext, conversions};
use anchor_lang::solana_program::instruction::AccountMeta as AnchorAccountMeta;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::spl_token;
use arising::codex::ResourceKind;
use solana_account::Account;
use spl_token::solana_program::program_option::COption;
use spl_token::state::{Account as TokenAccount, AccountState, Mint};

const RESOURCE_MINT_SEED: &[u8] = b"resource_mint";

/// Returns the PDA mint for a given resource kind.
pub fn resource_mint(resource: ResourceKind) -> AnchorPubkey {
    AnchorPubkey::find_program_address(&[RESOURCE_MINT_SEED, resource.seed_bytes()], &arising::ID).0
}

/// Builds the mint/ATA metas required to mint rewards to `authority`.
pub fn reward_accounts(
    authority: &AnchorPubkey,
    resources: &[ResourceKind],
) -> Vec<AnchorAccountMeta> {
    resources
        .iter()
        .flat_map(|resource| {
            let mint = resource_mint(*resource);
            let ata = get_associated_token_address(authority, &mint);
            let metadata = AnchorPubkey::find_program_address(
                &[b"metadata", mpl_token_metadata::ID.as_ref(), mint.as_ref()],
                &mpl_token_metadata::ID,
            )
            .0;
            vec![
                AnchorAccountMeta::new(mint, false),
                AnchorAccountMeta::new(metadata, false),
                AnchorAccountMeta::new(ata, false),
            ]
        })
        .collect()
}

/// Seeds a token account with the desired amount of a resource.
pub fn seed_resource_balance(
    ctx: &mut TestContext,
    mint_authority: &AnchorPubkey,
    owner: &AnchorPubkey,
    resource: ResourceKind,
    amount: u64,
) {
    let mint = resource_mint(resource);
    ensure_mint(ctx, *mint_authority, mint);
    let ata = get_associated_token_address(owner, &mint);
    let rent = ctx
        .svm
        .minimum_balance_for_rent_exemption(TokenAccount::LEN);
    let mut token_bytes = vec![0u8; TokenAccount::LEN];
    <TokenAccount as Pack>::pack(
        TokenAccount {
            mint,
            owner: *owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut token_bytes,
    )
    .unwrap();
    let token_account = Account {
        lamports: rent,
        data: token_bytes,
        owner: conversions::to_solana_pubkey(spl_token::id()),
        executable: false,
        rent_epoch: 0,
    };
    ctx.svm
        .set_account(conversions::to_solana_pubkey(ata), token_account)
        .expect("user ATA stored");

    let mint_key = conversions::to_solana_pubkey(mint);
    let mut mint_account = ctx.svm.get_account(&mint_key).expect("mint must exist");
    let mut mint_state = Mint::unpack(&mint_account.data).unwrap();
    mint_state.supply = mint_state.supply.saturating_add(amount);
    <Mint as Pack>::pack(mint_state, &mut mint_account.data).unwrap();
    ctx.svm.set_account(mint_key, mint_account).unwrap();
}

/// Reads the SPL token balance for a specific resource kind.
pub fn token_balance(ctx: &TestContext, owner: &AnchorPubkey, resource: ResourceKind) -> u64 {
    let ata = get_associated_token_address(owner, &resource_mint(resource));
    match ctx.svm.get_account(&conversions::to_solana_pubkey(ata)) {
        Some(account) => TokenAccount::unpack(&account.data).unwrap().amount,
        None => 0,
    }
}

pub fn create_spl_mint(
    ctx: &mut TestContext,
    mint_authority: AnchorPubkey,
    decimals: u8,
) -> AnchorPubkey {
    let mint = AnchorPubkey::new_unique();
    write_mint(ctx, mint, mint_authority, decimals);
    mint
}

pub fn write_token_account(
    ctx: &mut TestContext,
    address: AnchorPubkey,
    mint: AnchorPubkey,
    owner: AnchorPubkey,
    amount: u64,
) {
    let rent = ctx
        .svm
        .minimum_balance_for_rent_exemption(TokenAccount::LEN);
    let mut token_bytes = vec![0u8; TokenAccount::LEN];
    <TokenAccount as Pack>::pack(
        TokenAccount {
            mint,
            owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        &mut token_bytes,
    )
    .unwrap();
    let account = Account {
        lamports: rent,
        data: token_bytes,
        owner: conversions::to_solana_pubkey(spl_token::id()),
        executable: false,
        rent_epoch: 0,
    };
    ctx.svm
        .set_account(conversions::to_solana_pubkey(address), account)
        .expect("token account stored");
}

pub fn create_spl_token_account(
    ctx: &mut TestContext,
    owner: AnchorPubkey,
    mint: AnchorPubkey,
    amount: u64,
) -> AnchorPubkey {
    let address = AnchorPubkey::new_unique();
    write_token_account(ctx, address, mint, owner, amount);
    address
}

pub fn spl_account_balance(ctx: &TestContext, address: AnchorPubkey) -> u64 {
    match ctx.svm.get_account(&conversions::to_solana_pubkey(address)) {
        Some(account) => TokenAccount::unpack(&account.data).unwrap().amount,
        None => 0,
    }
}

/// Initializes the PDA mint if it has not yet been created.
fn ensure_mint(ctx: &mut TestContext, mint_authority: AnchorPubkey, mint: AnchorPubkey) {
    write_mint(ctx, mint, mint_authority, 0);
}

fn write_mint(
    ctx: &mut TestContext,
    mint: AnchorPubkey,
    mint_authority: AnchorPubkey,
    decimals: u8,
) {
    let key = conversions::to_solana_pubkey(mint);
    if ctx.svm.get_account(&key).is_some() {
        return;
    }
    let rent = ctx.svm.minimum_balance_for_rent_exemption(Mint::LEN);
    let mut data = vec![0u8; Mint::LEN];
    <Mint as Pack>::pack(
        Mint {
            mint_authority: Some(mint_authority).into(),
            supply: 0,
            decimals,
            is_initialized: true,
            freeze_authority: Some(mint_authority).into(),
        },
        &mut data,
    )
    .unwrap();
    let account = Account {
        lamports: rent,
        data,
        owner: conversions::to_solana_pubkey(spl_token::id()),
        executable: false,
        rent_epoch: 0,
    };
    ctx.svm.set_account(key, account).unwrap();
}
