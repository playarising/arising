#[path = "common/mod.rs"]
mod common;

use arising::codex::{Civilization, Gender, ResourceKind};
use common::{
    clock,
    context::{TestContext, TransactionResultExt},
    conversions, fixtures, instructions, tokens,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn quest_and_recipe_flow() {
    let mut ctx = TestContext::new();
    let mint_state = fixtures::create_mint_state(&mut ctx);
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

    tokens::seed_resource_balance(&mut ctx, &mint_state, &authority_pk, ResourceKind::Log, 1);

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
            tokens::reward_accounts(&authority_pk, &[ResourceKind::Log]),
        ),
        &[&authority],
    )
    .assert_success();

    for _ in 0..2 {
        let instr = instructions::start_recipe(
            Civilization::Ard,
            2,
            1,
            forger,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::Log]),
        );
        dbg!(&instr.accounts);
        ctx.execute_instruction(instr, &[&authority])
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
                tokens::reward_accounts(&authority_pk, &[ResourceKind::Plank]),
            ),
            &[&authority],
        )
        .assert_success();
    }

    ctx.execute_instruction(
        instructions::start_recipe(
            Civilization::Ard,
            3,
            2,
            crafter,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::Plank, ResourceKind::Gold]),
        ),
        &[&authority],
    )
    .assert_success();
    clock::fast_forward_recipe(&mut ctx, &crafter);
    ctx.execute_instruction(
        instructions::claim_recipe(
            Civilization::Ard,
            3,
            2,
            crafter,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::WoodenSword]),
        ),
        &[&authority],
    )
    .assert_success();

    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Gold),
        50
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Log),
        0
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::Plank),
        0
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodenSword),
        1
    );

    let crafter_after = ctx.get_anchor_account::<arising::character::Character>(&crafter);
    assert!(crafter_after.current_recipe.is_none());
}
