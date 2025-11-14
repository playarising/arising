#[path = "common/mod.rs"]
mod common;

use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use arising::codex::{Civilization, EquipmentSlot, Gender, ResourceKind};
use common::{
    context::{TestContext, TransactionResultExt},
    conversions, fixtures, instructions, tokens,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn equips_and_swaps_items() {
    let mut ctx = TestContext::new();
    let mint_state = fixtures::create_mint_state(&mut ctx);
    let authority = Keypair::new();
    ctx.svm.airdrop(&authority.pubkey(), 5_000_000_000).unwrap();
    let authority_pk = conversions::keypair_pubkey(&authority);

    let character = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        11,
        &authority,
        Gender::Male,
        None,
    );

    level_character(&mut ctx, &character, 3);

    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::WoodenSword,
        2,
    );
    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::WoodenShield,
        1,
    );
    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::TrainingGreatsword,
        1,
    );

    ctx.execute_instruction(
        instructions::equip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::RightHand,
            ResourceKind::WoodenSword,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::WoodenSword]),
        ),
        &[&authority],
    )
    .assert_success();
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodenSword),
        1
    );

    let meta = ctx.execute_instruction(
        instructions::equip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::LeftHand,
            ResourceKind::WoodenShield,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::WoodenShield]),
        ),
        &[&authority],
    );
    meta.assert_success();
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodenShield),
        0
    );

    ctx.execute_instruction(
        instructions::equip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::LeftHand,
            ResourceKind::TrainingGreatsword,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(
                &authority_pk,
                &[
                    ResourceKind::TrainingGreatsword,
                    ResourceKind::WoodenSword,
                    ResourceKind::WoodenShield,
                ],
            ),
        ),
        &[&authority],
    )
    .assert_success();

    let character_after = ctx.get_anchor_account::<arising::character::Character>(&character);
    let left_slot = character_after.equipment.slots[EquipmentSlot::LeftHand as usize];
    let right_slot = character_after.equipment.slots[EquipmentSlot::RightHand as usize];
    assert_eq!(left_slot.item, Some(ResourceKind::TrainingGreatsword));
    assert_eq!(right_slot.item, Some(ResourceKind::TrainingGreatsword));
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodenSword),
        2
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodenShield),
        1
    );

    ctx.execute_instruction(
        instructions::unequip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::RightHand,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::TrainingGreatsword]),
        ),
        &[&authority],
    )
    .assert_success();

    let character_final = ctx.get_anchor_account::<arising::character::Character>(&character);
    let left_final = character_final.equipment.slots[EquipmentSlot::LeftHand as usize];
    let right_final = character_final.equipment.slots[EquipmentSlot::RightHand as usize];
    assert!(left_final.item.is_none());
    assert!(right_final.item.is_none());
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::TrainingGreatsword),
        1
    );
}

fn level_character(ctx: &mut TestContext, character: &AnchorPubkey, target_level: u8) {
    use arising::level_codex::LEVEL_CURVE;

    let mut account = ctx.get_anchor_account::<arising::character::Character>(character);
    let mut total_xp = 0u64;
    for entry in LEVEL_CURVE.iter().take(target_level.saturating_sub(1) as usize) {
        total_xp += entry.xp_to_next as u64;
    }
    account.experience = total_xp;
    account.energy = account.energy_capacity();
    ctx.store_anchor_account(*character, arising::ID, &account, arising::character::Character::LEN);
}
