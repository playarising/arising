//! Validates that key instructions emit the expected Anchor events.

#[path = "common/mod.rs"]
mod common;

use anchor_lang::{
    solana_program::instruction::Instruction as AnchorInstruction, AnchorDeserialize,
    Discriminator, InstructionData, ToAccountMetas,
};
use arising::{
    accounts,
    character::{Character, CharacterMintState, ENERGY_REFILL_COOLDOWN_SECS},
    codex::{Attributes, CharacterClass, Civilization, CoreStats, Gender},
    instruction,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use common::{
    clock,
    context::TestContext,
    conversions::{self, convert_instruction},
    fixtures,
};

fn build_instruction<A: ToAccountMetas, D: InstructionData>(
    accounts: A,
    data: D,
) -> solana_instruction::Instruction {
    let ix = AnchorInstruction {
        program_id: arising::ID,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };
    convert_instruction(ix)
}

fn expect_event<E>(logs: &[String]) -> E
where
    E: AnchorDeserialize + Discriminator,
{
    let prefix = "Program data: ";
    for log in logs {
        if let Some(encoded) = log.strip_prefix(prefix) {
            if let Ok(bytes) = STANDARD.decode(encoded) {
                if bytes.starts_with(E::DISCRIMINATOR) {
                    let mut slice: &[u8] = &bytes[E::DISCRIMINATOR.len()..];
                    return E::deserialize(&mut slice).expect("event decode");
                }
            }
        }
    }
    panic!("event {} not found in logs", std::any::type_name::<E>());
}

#[test]
fn emits_character_management_events() {
    let mut ctx = TestContext::new();
    let mint_fixture = fixtures::create_mint_state(&mut ctx);
    let authority = ctx.payer().insecure_clone();
    let authority_pk = conversions::keypair_pubkey(&authority);

    let character = fixtures::create_character(
        &mut ctx,
        Civilization::Ard,
        1,
        &authority,
        Gender::Female,
        None,
    );

    // Give the character enough XP to unlock stats/attributes and class selection.
    let mut character_account: Character = ctx.get_anchor_account(&character);
    character_account.experience = 3_500; // Level 10
    ctx.store_anchor_account(character, arising::ID, &character_account, Character::LEN);

    // Allocate core stats.
    let core_allocation = CoreStats::new(1, 1, 0);
    let core_meta = ctx
        .execute_instruction(
            build_instruction(
                accounts::ModifyCharacterOwned {
                    character,
                    authority: authority_pk,
                },
                instruction::AllocateCoreStats {
                    civilization: Civilization::Ard,
                    character_id: 1,
                    allocation: core_allocation,
                },
            ),
            &[&authority],
        )
        .expect("core stats allocation");
    let core_event: arising::CoreStatsAllocatedEvent = expect_event(&core_meta.logs);
    let after_core: Character = ctx.get_anchor_account(&character);
    assert_core_eq(core_event.allocation, core_allocation);
    assert_core_eq(core_event.new_stats, after_core.stats);
    assert_eq!(
        core_event.available_core_points,
        after_core.available_core_points()
    );

    // Allocate attribute points.
    let attr_allocation = Attributes::new(1, 0, 0, 0, 0, 0);
    let attr_meta = ctx
        .execute_instruction(
            build_instruction(
                accounts::ModifyCharacterOwned {
                    character,
                    authority: authority_pk,
                },
                instruction::AllocateAttributes {
                    civilization: Civilization::Ard,
                    character_id: 1,
                    allocation: attr_allocation,
                },
            ),
            &[&authority],
        )
        .expect("attribute allocation");
    let attr_event: arising::AttributesAllocatedEvent = expect_event(&attr_meta.logs);
    let after_attr: Character = ctx.get_anchor_account(&character);
    assert_attrs_eq(attr_event.allocation, attr_allocation);
    assert_attrs_eq(attr_event.new_attributes, after_attr.attributes);
    assert_eq!(
        attr_event.available_attribute_points,
        after_attr.available_attribute_points()
    );

    // Choose a class.
    let class = CharacterClass::ArdKnight;
    let class_meta = ctx
        .execute_instruction(
            build_instruction(
                accounts::ModifyCharacterOwned {
                    character,
                    authority: authority_pk,
                },
                instruction::ChooseClass {
                    civilization: Civilization::Ard,
                    character_id: 1,
                    class,
                },
            ),
            &[&authority],
        )
        .expect("class choice");
    let class_event: arising::ClassChosenEvent = expect_event(&class_meta.logs);
    let after_class: Character = ctx.get_anchor_account(&character);
    assert_eq!(class_event.class, class);
    assert_core_eq(class_event.new_stats, after_class.stats);
    assert_attrs_eq(class_event.new_attributes, after_class.attributes);

    // Spend and then refill energy.
    let energy_amount = 3;
    let consume_meta = ctx
        .execute_instruction(
            build_instruction(
                accounts::ModifyCharacterOwned {
                    character,
                    authority: authority_pk,
                },
                instruction::ConsumeEnergy {
                    civilization: Civilization::Ard,
                    character_id: 1,
                    amount: energy_amount,
                },
            ),
            &[&authority],
        )
        .expect("consume energy");
    let consume_event: arising::EnergyConsumedEvent = expect_event(&consume_meta.logs);
    let after_consume: Character = ctx.get_anchor_account(&character);
    assert_eq!(consume_event.amount, energy_amount);
    assert_eq!(consume_event.remaining_energy, after_consume.energy);

    clock::warp_seconds(&mut ctx, ENERGY_REFILL_COOLDOWN_SECS + 1);
    let refill_meta = ctx
        .execute_instruction(
            build_instruction(
                accounts::ModifyCharacterOwned {
                    character,
                    authority: authority_pk,
                },
                instruction::RefillEnergy {
                    civilization: Civilization::Ard,
                    character_id: 1,
                },
            ),
            &[&authority],
        )
        .expect("refill energy");
    let refill_event: arising::EnergyRefilledEvent = expect_event(&refill_meta.logs);
    let after_refill: Character = ctx.get_anchor_account(&character);
    assert_eq!(refill_event.new_energy, after_refill.energy);
    assert!(after_refill.energy > consume_event.remaining_energy);

    // Update civilization cap and verify the log.
    let cap_meta = ctx
        .execute_instruction(
            build_instruction(
                accounts::UpdateCivilizationCap {
                    mint_state: mint_fixture.mint_state,
                    authority: authority_pk,
                },
                instruction::UpdateCivilizationCap {
                    civilization: Civilization::Ard,
                    new_cap: 1_500,
                },
            ),
            &[&authority],
        )
        .expect("update cap");
    let cap_event: arising::CivilizationCapUpdatedEvent = expect_event(&cap_meta.logs);
    let mint_state: CharacterMintState = ctx.get_anchor_account(&mint_fixture.mint_state);
    assert_eq!(cap_event.previous_cap, 1_000);
    assert_eq!(cap_event.new_cap, mint_state.track(Civilization::Ard).cap);
}

fn assert_core_eq(left: CoreStats, right: CoreStats) {
    assert_eq!(left.might, right.might);
    assert_eq!(left.speed, right.speed);
    assert_eq!(left.intellect, right.intellect);
}

fn assert_attrs_eq(left: Attributes, right: Attributes) {
    assert_eq!(left.atk, right.atk);
    assert_eq!(left.def, right.def);
    assert_eq!(left.mag_atk, right.mag_atk);
    assert_eq!(left.mag_def, right.mag_def);
    assert_eq!(left.range, right.range);
    assert_eq!(left.rate, right.rate);
}
