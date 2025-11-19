//! Failure-mode coverage for quests, recipes, and permissions.

#[path = "common/mod.rs"]
mod common;

use arising::character::Character;
use arising::codex::{Civilization, CoreStats, Gender, ResourceKind};
use arising::errors::ArisingError;
use common::{
    context::{TestContext, TransactionResultExt},
    conversions, fixtures, instructions, recipes, tokens,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

/// Verifies quests cannot be claimed before their cooldown expires.
#[test]
fn cannot_claim_before_ready() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let mint_state = mint_fixture.mint_state;
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);
    let gatherer = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        1,
        &authority,
        Gender::Male,
        Some(CoreStats::new(3, 2, 2)), // Meet quest 1 requirements after stat floor
    );

    // Give character enough energy for the quest
    let mut char_account: Character = ctx.get_anchor_account(&gatherer);
    char_account.energy = 20; // Ensure enough energy for quest
    ctx.store_anchor_account(gatherer, arising::ID, &char_account, Character::LEN);

    // Start the quest but do not advance the clock long enough.
    ctx.execute_instruction(
        instructions::start_quest(Civilization::Ard, 1, 1, gatherer, authority_pk),
        &[&authority],
    )
    .assert_success();

    // Attempt to claim and expect an Anchor error.
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

/// Ensures stat requirements are enforced during quest start.
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

/// Validates recipe processing errors when accounts point at the wrong mint.
#[test]
fn recipe_consumes_available_resources() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let mint_state = mint_fixture.mint_state;
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);
    let crafter = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        9,
        &authority,
        Gender::Female,
        Some(CoreStats::new(5, 4, 4)), // Provide stats that meet recipe 17's requirements (4,4,4 minimum)
    );

    // Give character enough energy for the recipe
    let mut char_account: Character = ctx.get_anchor_account(&crafter);
    char_account.energy = 30; // Ensure enough energy for recipe
    ctx.store_anchor_account(crafter, arising::ID, &char_account, Character::LEN);

    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::Gold,
        100,
    );

    // Provide only the gold accounts, which should trip validation.
    ctx.execute_instruction(
        instructions::start_recipe(
            Civilization::Ard,
            9,
            17,
            crafter,
            authority_pk,
            mint_state,
            recipes::recipe_input_accounts(&authority_pk, 17),
        ),
        &[&authority],
    )
    .assert_anchor_error(ArisingError::InvalidResourceAccount);
}
