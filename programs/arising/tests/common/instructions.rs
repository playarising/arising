//! Builders for invoking program instructions inside LiteSVM tests.

use crate::common::conversions;
use anchor_lang::solana_program::instruction::AccountMeta as AnchorAccountMeta;
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use anchor_lang::{InstructionData, ToAccountMetas};
use arising::accounts;
use arising::codex::{equipment_definition_index, Civilization, EquipmentSlot, ResourceKind};
use solana_instruction::Instruction;

/// Converts Anchor instructions to native Solana instructions.
fn convert_instruction(ix: anchor_lang::solana_program::instruction::Instruction) -> Instruction {
    conversions::convert_instruction(ix)
}

/// Builds a `start_quest` instruction for the provided character.
pub fn start_quest(
    civilization: Civilization,
    character_id: u16,
    quest_id: u16,
    character: AnchorPubkey,
    authority: AnchorPubkey,
) -> Instruction {
    let accounts = accounts::ModifyCharacterOwned {
        character,
        authority,
    };
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts: accounts.to_account_metas(None),
        data: arising::instruction::StartQuest {
            civilization,
            character_id,
            quest_id,
        }
        .data(),
    })
}

/// Builds a `claim_quest` instruction with the provided reward accounts.
pub fn claim_quest(
    civilization: Civilization,
    character_id: u16,
    quest_id: u16,
    character: AnchorPubkey,
    authority: AnchorPubkey,
    mint_state: AnchorPubkey,
    rewards: Vec<AnchorAccountMeta>,
) -> Instruction {
    let mut accounts = accounts::ClaimQuest {
        character,
        authority,
        mint_state,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    }
    .to_account_metas(None);
    accounts.extend(rewards);
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts,
        data: arising::instruction::ClaimQuest {
            civilization,
            character_id,
            quest_id,
        }
        .data(),
    })
}

/// Builds a `start_recipe` instruction including the remaining resource accounts.
pub fn start_recipe(
    civilization: Civilization,
    character_id: u16,
    recipe_id: u16,
    character: AnchorPubkey,
    authority: AnchorPubkey,
    mint_state: AnchorPubkey,
    remaining: Vec<AnchorAccountMeta>,
) -> Instruction {
    let mut accounts = arising::accounts::ProcessRecipe {
        character,
        authority,
        mint_state,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    }
    .to_account_metas(None);
    accounts.extend(remaining);
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts,
        data: arising::instruction::StartRecipe {
            civilization,
            character_id,
            recipe_id,
        }
        .data(),
    })
}

/// Builds a `claim_recipe` instruction wiring the reward accounts.
pub fn claim_recipe(
    civilization: Civilization,
    character_id: u16,
    recipe_id: u16,
    character: AnchorPubkey,
    authority: AnchorPubkey,
    mint_state: AnchorPubkey,
    remaining: Vec<AnchorAccountMeta>,
) -> Instruction {
    let mut accounts = arising::accounts::ProcessRecipe {
        character,
        authority,
        mint_state,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    }
    .to_account_metas(None);
    accounts.extend(remaining);
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts,
        data: arising::instruction::ClaimRecipe {
            civilization,
            character_id,
            recipe_id,
        }
        .data(),
    })
}

/// Builds an `equip_item` instruction for the given slot/resource.
pub fn equip_item(
    civilization: Civilization,
    character_id: u16,
    slot: EquipmentSlot,
    resource: ResourceKind,
    character: AnchorPubkey,
    authority: AnchorPubkey,
    mint_state: AnchorPubkey,
    resource_accounts: Vec<AnchorAccountMeta>,
) -> Instruction {
    let mut accounts = arising::accounts::ManageEquipment {
        character,
        authority,
        mint_state,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    }
    .to_account_metas(None);
    accounts.extend(resource_accounts);
    let definition_index = equipment_definition_index(resource).expect("equipment index") as u8;
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts,
        data: arising::instruction::EquipItem {
            civilization,
            character_id,
            slot,
            definition_index,
        }
        .data(),
    })
}

/// Builds an `unequip_item` instruction.
pub fn unequip_item(
    civilization: Civilization,
    character_id: u16,
    slot: EquipmentSlot,
    character: AnchorPubkey,
    authority: AnchorPubkey,
    mint_state: AnchorPubkey,
    resource_accounts: Vec<AnchorAccountMeta>,
) -> Instruction {
    let mut accounts = arising::accounts::ManageEquipment {
        character,
        authority,
        mint_state,
        token_program: anchor_spl::token::ID,
        associated_token_program: anchor_spl::associated_token::ID,
        system_program: anchor_lang::system_program::ID,
        rent: anchor_lang::solana_program::sysvar::rent::ID,
    }
    .to_account_metas(None);
    accounts.extend(resource_accounts);
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts,
        data: arising::instruction::UnequipItem {
            civilization,
            character_id,
            slot,
        }
        .data(),
    })
}

/// Builds an authority-only instruction to update a civilization cap.
pub fn update_civilization_cap(
    mint_state: AnchorPubkey,
    authority: AnchorPubkey,
    civilization: Civilization,
    cap: u16,
) -> Instruction {
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts: accounts::UpdateCivilizationCap {
            mint_state,
            authority,
        }
        .to_account_metas(None),
        data: arising::instruction::UpdateCivilizationCap {
            civilization,
            new_cap: cap,
        }
        .data(),
    })
}

pub fn buy_energy_pass(
    civilization: Civilization,
    character_id: u16,
    quantity: u8,
    character: AnchorPubkey,
    authority: AnchorPubkey,
    mint_state: AnchorPubkey,
    user_usdc_account: AnchorPubkey,
    treasury_usdc_account: AnchorPubkey,
) -> Instruction {
    let accounts = arising::accounts::BuyEnergyPass {
        character,
        authority,
        mint_state,
        user_usdc_account,
        treasury_usdc_account,
        token_program: anchor_spl::token::ID,
    };
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts: accounts.to_account_metas(None),
        data: arising::instruction::BuyEnergyPass {
            civilization,
            character_id,
            quantity,
        }
        .data(),
    })
}

pub fn redeem_energy_pass(
    civilization: Civilization,
    character_id: u16,
    character: AnchorPubkey,
    authority: AnchorPubkey,
) -> Instruction {
    let accounts = arising::accounts::RedeemEnergyPass {
        character,
        authority,
    };
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts: accounts.to_account_metas(None),
        data: arising::instruction::RedeemEnergyPass {
            civilization,
            character_id,
        }
        .data(),
    })
}

pub fn withdraw_usdc_treasury(
    mint_state: AnchorPubkey,
    mint_authority: AnchorPubkey,
    treasury: AnchorPubkey,
    destination: AnchorPubkey,
    amount: u64,
) -> Instruction {
    let accounts = arising::accounts::WithdrawUsdcTreasury {
        mint_state,
        mint_authority,
        treasury_usdc_account: treasury,
        destination,
        token_program: anchor_spl::token::ID,
    };
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts: accounts.to_account_metas(None),
        data: arising::instruction::WithdrawUsdcTreasury { amount }.data(),
    })
}
