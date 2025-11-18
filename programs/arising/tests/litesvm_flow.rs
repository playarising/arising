//! End-to-end quest, forge, and craft validation.

#[path = "common/mod.rs"]
mod common;

use arising::codex::{Civilization, Gender, ResourceKind};
use common::{
    clock,
    context::{TestContext, TransactionResultExt},
    conversions, fixtures, instructions, recipes, tokens,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

/// Test quest and recipe flow
#[test]
fn quest_and_recipe_flow() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let mint_state = mint_fixture.mint_state;
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 5_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);

    let gatherer = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        1,
        &authority,
        Gender::Male,
        None,
    );
    let forger = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        2,
        &authority,
        Gender::Female,
        None,
    );
    let crafter = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        3,
        &authority,
        Gender::Male,
        None,
    );

    // Run a job quest to earn starting gold
    ctx.execute_instruction(
        instructions::start_quest(Civilization::Ard, 1, 1, gatherer, authority_pk),
        &[&authority],
    )
    .assert_success();
    clock::fast_forward_quest(&mut ctx, &gatherer);
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
    .assert_success();

    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Gold),
        100
    );

    // Gather raw wood twice to have enough material for forging
    for _ in 0..2 {
        ctx.execute_instruction(
            instructions::start_quest(Civilization::Ard, 1, 2, gatherer, authority_pk),
            &[&authority],
        )
        .assert_success();
        clock::fast_forward_quest(&mut ctx, &gatherer);
        ctx.execute_instruction(
            instructions::claim_quest(
                Civilization::Ard,
                1,
                2,
                gatherer,
                authority_pk,
                mint_state,
                tokens::reward_accounts(&authority_pk, &[ResourceKind::Wood]),
            ),
            &[&authority],
        )
        .assert_success();
    }

    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Wood),
        2
    );

    // Forge two planks from the gathered wood
    for _ in 0..2 {
        ctx.execute_instruction(
            instructions::start_recipe(
                Civilization::Ard,
                2,
                1,
                forger,
                authority_pk,
                mint_state,
                recipes::recipe_input_accounts(&authority_pk, 1),
            ),
            &[&authority],
        )
        .assert_success();
        clock::fast_forward_recipe(&mut ctx, &forger);
        ctx.execute_instruction(
            instructions::claim_recipe(
                Civilization::Ard,
                2,
                1,
                forger,
                authority_pk,
                mint_state,
                recipes::recipe_output_accounts(&authority_pk, 1),
            ),
            &[&authority],
        )
        .assert_success();
    }

    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Wood),
        0
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Plank),
        2
    );

    // Seed leather needed for the crafted weapon
    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::Leather,
        1,
    );

    // Craft a wooden sword using the planks, leather, and gold
    ctx.execute_instruction(
        instructions::start_recipe(
            Civilization::Ard,
            3,
            18,
            crafter,
            authority_pk,
            mint_state,
            recipes::recipe_input_accounts(&authority_pk, 18),
        ),
        &[&authority],
    )
    .assert_success();
    clock::fast_forward_recipe(&mut ctx, &crafter);
    ctx.execute_instruction(
        instructions::claim_recipe(
            Civilization::Ard,
            3,
            18,
            crafter,
            authority_pk,
            mint_state,
            recipes::recipe_output_accounts(&authority_pk, 18),
        ),
        &[&authority],
    )
    .assert_success();

    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Gold),
        50
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Plank),
        0
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodSword),
        1
    );

    let forger_after = ctx.get_anchor_account::<arising::character::Character>(&forger);
    assert!(forger_after.current_recipe.is_none());
    let crafter_after = ctx.get_anchor_account::<arising::character::Character>(&crafter);
    assert!(crafter_after.current_recipe.is_none());
}
