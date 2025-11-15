#[path = "common/mod.rs"]
mod common;

use arising::codex::{Civilization, Gender};
use arising::errors::ArisingError;
use common::{
    clock,
    context::{TestContext, TransactionResultExt},
    conversions, fixtures,
    instructions::{self},
    tokens,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

const CIV: Civilization = Civilization::Ard;

#[test]
fn purchase_and_redeem_passes_enforce_limits() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let mint_state = mint_fixture.mint_state;
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 5_000_000_000).unwrap();

    let character = fixtures::create_character(&mut ctx, CIV, 1, &authority, Gender::Male, None);

    let user_usdc_account = tokens::create_spl_token_account(
        &mut ctx,
        conversions::keypair_pubkey(&authority),
        mint_fixture.usdc_mint,
        20 * 1_000_000,
    );

    ctx.execute_instruction(
        instructions::buy_energy_pass(
            CIV,
            1,
            4,
            character,
            conversions::keypair_pubkey(&authority),
            mint_state,
            user_usdc_account,
            mint_fixture.usdc_treasury,
        ),
        &[&authority],
    )
    .assert_success();

    let post_balance = tokens::spl_account_balance(&ctx, user_usdc_account);
    assert_eq!(post_balance, 20_000_000 - 8_000_000);
    let treasury_balance = tokens::spl_account_balance(&ctx, mint_fixture.usdc_treasury);
    assert_eq!(treasury_balance, 8_000_000);

    // Drain energy to validate refill behavior.
    let mut account = ctx.get_anchor_account::<arising::character::Character>(&character);
    account.energy = 1;
    ctx.store_anchor_account(
        character,
        arising::ID,
        &account,
        arising::character::Character::LEN,
    );

    for _ in 0..2 {
        ctx.execute_instruction(
            instructions::redeem_energy_pass(
                CIV,
                1,
                character,
                conversions::keypair_pubkey(&authority),
            ),
            &[&authority],
        )
        .assert_success();
    }

    // Third redemption hits daily limit of 3.
    ctx.execute_instruction(
        instructions::redeem_energy_pass(
            CIV,
            1,
            character,
            conversions::keypair_pubkey(&authority),
        ),
        &[&authority],
    )
    .assert_success();

    ctx.execute_instruction(
        instructions::redeem_energy_pass(
            CIV,
            1,
            character,
            conversions::keypair_pubkey(&authority),
        ),
        &[&authority],
    )
    .assert_anchor_error(ArisingError::TooManyPassUses);

    // Purchasing beyond the window fails until the clock is advanced.
    ctx.execute_instruction(
        instructions::buy_energy_pass(
            CIV,
            1,
            2,
            character,
            conversions::keypair_pubkey(&authority),
            mint_state,
            user_usdc_account,
            mint_fixture.usdc_treasury,
        ),
        &[&authority],
    )
    .assert_anchor_error(ArisingError::TooManyPassPurchases);

    clock::warp_seconds(&mut ctx, 24 * 60 * 60 + 1);
    ctx.execute_instruction(
        instructions::buy_energy_pass(
            CIV,
            1,
            4,
            character,
            conversions::keypair_pubkey(&authority),
            mint_state,
            user_usdc_account,
            mint_fixture.usdc_treasury,
        ),
        &[&authority],
    )
    .assert_success();
}

#[test]
fn withdraw_requires_authority() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let mint_state = mint_fixture.mint_state;
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 5_000_000_000).unwrap();
    let character = fixtures::create_character(&mut ctx, CIV, 2, &authority, Gender::Female, None);
    let user_usdc_account = tokens::create_spl_token_account(
        &mut ctx,
        conversions::keypair_pubkey(&authority),
        mint_fixture.usdc_mint,
        4 * 1_000_000,
    );
    ctx.execute_instruction(
        instructions::buy_energy_pass(
            CIV,
            2,
            2,
            character,
            conversions::keypair_pubkey(&authority),
            mint_state,
            user_usdc_account,
            mint_fixture.usdc_treasury,
        ),
        &[&authority],
    )
    .assert_success();

    let outsider = Keypair::new();
    ctx.svm.airdrop(&outsider.pubkey(), 1_000_000_000).unwrap();
    let destination = tokens::create_spl_token_account(
        &mut ctx,
        conversions::keypair_pubkey(&outsider),
        mint_fixture.usdc_mint,
        0,
    );

    ctx.execute_instruction(
        instructions::withdraw_usdc_treasury(
            mint_state,
            conversions::keypair_pubkey(&outsider),
            mint_fixture.usdc_treasury,
            destination,
            1_000_000,
        ),
        &[&outsider],
    )
    .assert_anchor_error(ArisingError::Unauthorized);

    let admin = ctx.payer();
    ctx.execute_instruction(
        instructions::withdraw_usdc_treasury(
            mint_state,
            conversions::keypair_pubkey(admin),
            mint_fixture.usdc_treasury,
            destination,
            1_000_000,
        ),
        &[],
    )
    .assert_success();

    assert_eq!(tokens::spl_account_balance(&ctx, destination), 1_000_000);
}
