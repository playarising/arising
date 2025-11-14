use crate::common::conversions;
use anchor_lang::solana_program::instruction::AccountMeta as AnchorAccountMeta;
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use anchor_lang::{InstructionData, ToAccountMetas};
use arising::accounts;
use arising::codex::{equipment_definition_index, Civilization, EquipmentSlot, ResourceKind};
use solana_instruction::Instruction;

fn convert_instruction(ix: anchor_lang::solana_program::instruction::Instruction) -> Instruction {
    conversions::convert_instruction(ix)
}

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
    convert_instruction(anchor_lang::solana_program::instruction::Instruction {
        program_id: arising::ID,
        accounts,
        data: arising::instruction::EquipItem {
            civilization,
            character_id,
            slot,
            definition_index: equipment_definition_index(resource).expect("equipment index") as u8,
        }
        .data(),
    })
}

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
