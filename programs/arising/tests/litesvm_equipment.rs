//! Equipment lifecycle happy-path testing.

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

/// Ensures equipping/unequipping handles swapping and mint/burn flows.
#[test]
fn equips_and_swaps_items() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let mint_state = mint_fixture.mint_state;
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

    // Manually boost the character to meet equipment level requirements.
    level_character(&mut ctx, &character, 5);

    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::WoodSword,
        2,
    );
    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::WoodBigShield,
        1,
    );
    tokens::seed_resource_balance(
        &mut ctx,
        &mint_state,
        &authority_pk,
        ResourceKind::WoodHalberd,
        1,
    );

    // Equip a right-hand sword.
    ctx.execute_instruction(
        instructions::equip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::RightHand,
            ResourceKind::WoodSword,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::WoodSword]),
        ),
        &[&authority],
    )
    .assert_success();
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodSword),
        1
    );

    // Equip a shield in the left hand, ensuring swapping works.
    let meta = ctx.execute_instruction(
        instructions::equip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::LeftHand,
            ResourceKind::WoodBigShield,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::WoodBigShield]),
        ),
        &[&authority],
    );
    meta.assert_success();
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodBigShield),
        0
    );

    // Equip a two-handed weapon which should return both prior items.
    ctx.execute_instruction(
        instructions::equip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::LeftHand,
            ResourceKind::WoodHalberd,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(
                &authority_pk,
                &[
                    ResourceKind::WoodHalberd,
                    ResourceKind::WoodBigShield,
                    ResourceKind::WoodSword,
                ],
            ),
        ),
        &[&authority],
    )
    .assert_success();

    let character_after = ctx.get_anchor_account::<arising::character::Character>(&character);
    let left_slot = character_after.equipment.slots[EquipmentSlot::LeftHand as usize];
    let right_slot = character_after.equipment.slots[EquipmentSlot::RightHand as usize];
    assert_eq!(left_slot.item, Some(ResourceKind::WoodHalberd));
    assert_eq!(right_slot.item, Some(ResourceKind::WoodHalberd));
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodSword),
        2
    );
    assert_eq!(
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodBigShield),
        1
    );

    // Unequip to mint the weapon back.
    ctx.execute_instruction(
        instructions::unequip_item(
            Civilization::Ard,
            11,
            EquipmentSlot::RightHand,
            character,
            authority_pk,
            mint_state,
            tokens::reward_accounts(&authority_pk, &[ResourceKind::WoodHalberd]),
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
        tokens::token_balance(&ctx, &authority_pk, ResourceKind::WoodHalberd),
        1
    );
}

/// Helper that sets the character XP to the provided level for tests.
fn level_character(ctx: &mut TestContext, character: &AnchorPubkey, target_level: u8) {
    use arising::codex::LEVEL_CURVE;

    let mut account = ctx.get_anchor_account::<arising::character::Character>(character);
    let mut total_xp = 0u64;
    for entry in LEVEL_CURVE
        .iter()
        .take(target_level.saturating_sub(1) as usize)
    {
        total_xp += entry.xp_to_next as u64;
    }
    account.experience = total_xp;
    account.energy = account.energy_capacity();
    ctx.store_anchor_account(
        *character,
        arising::ID,
        &account,
        arising::character::Character::LEN,
    );
}
