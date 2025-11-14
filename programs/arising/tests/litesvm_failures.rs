#[path = "common/mod.rs"]
mod common;

use arising::codex::{Civilization, CoreStats, Gender, ResourceKind};
use arising::errors::ArisingError;
use common::{
    context::{TestContext, TransactionResultExt},
    conversions, fixtures, instructions, tokens,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn cannot_claim_before_ready() {
    let mut ctx = TestContext::new();
    let mint_state = fixtures::create_mint_state(&mut ctx);
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);
    let gatherer = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        1,
        &authority,
        Gender::Male,
        None,
    );

    ctx.execute_instruction(
        instructions::start_quest(Civilization::Ard, 1, 1, gatherer, authority_pk),
        &[&authority],
    )
    .assert_success();

    ctx.execute_instruction(
        instructions::claim_quest(
            Civilization::Ard,
            1,
            1,
            gatherer,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::Gold]),
        ),
        &[&authority],
    )
    .assert_anchor_error(ArisingError::QuestNotReady);
}

#[test]
fn quest_requires_stats() {
    let mut ctx = TestContext::new();
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);
    let weak_character = fixtures::create_character(
        &mut ctx,
        Civilization::Hartenn,
        4,
        &authority,
        Gender::Male,
        Some(CoreStats::zero()),
    );

    ctx.execute_instruction(
        instructions::start_quest(Civilization::Hartenn, 4, 3, weak_character, authority_pk),
        &[&authority],
    )
    .assert_anchor_error(ArisingError::QuestRequirementsNotMet);
}

#[test]
fn recipe_consumes_available_resources() {
    let mut ctx = TestContext::new();
    let mint_state = fixtures::create_mint_state(&mut ctx);
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);
    let crafter = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        9,
        &authority,
        Gender::Female,
        None,
    );

    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::Gold,
        100,
    );

    ctx.execute_instruction(
        instructions::start_recipe(
            Civilization::Ard,
            9,
            2,
            crafter,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::Plank, ResourceKind::Gold]),
        ),
        &[&authority],
    )
    .assert_anchor_error(ArisingError::InvalidResourceMint);
}
