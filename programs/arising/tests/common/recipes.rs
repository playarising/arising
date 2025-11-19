//! Helpers for deriving recipe resource metadata used in tests.

use crate::common::tokens;
use anchor_lang::solana_program::{
    instruction::AccountMeta as AnchorAccountMeta, pubkey::Pubkey as AnchorPubkey,
};
use arising::codex::{recipe_by_id, RecipeInput, RecipeOutput, ResourceKind};

/// Returns the ordered list of resource inputs required for a recipe.
pub fn recipe_input_resources(recipe_id: u16) -> Vec<ResourceKind> {
    let recipe = recipe_by_id(recipe_id).expect("invalid recipe id");
    match recipe.input {
        RecipeInput::Forge { raw_material, .. } => vec![raw_material],
        RecipeInput::Craft {
            materials,
            gold_amount,
        } => {
            let mut resources = materials.iter().map(|m| m.kind).collect::<Vec<_>>();
            if gold_amount > 0 {
                resources.push(ResourceKind::Gold);
            }
            resources
        }
    }
}

/// Returns the resource outputs produced by a recipe.
pub fn recipe_output_resources(recipe_id: u16) -> Vec<ResourceKind> {
    let recipe = recipe_by_id(recipe_id).expect("invalid recipe id");
    match recipe.output {
        RecipeOutput::Resource { resource, .. } => vec![resource],
    }
}

/// Builds mint/ATA metas for recipe inputs owned by `authority`.
pub fn recipe_input_accounts(authority: &AnchorPubkey, recipe_id: u16) -> Vec<AnchorAccountMeta> {
    let resources = recipe_input_resources(recipe_id);
    let accounts = tokens::reward_accounts(authority, &resources);
    assert_account_alignment(&accounts, &resources);
    accounts
}

/// Builds mint/ATA metas for recipe outputs owned by `authority`.
pub fn recipe_output_accounts(authority: &AnchorPubkey, recipe_id: u16) -> Vec<AnchorAccountMeta> {
    let resources = recipe_output_resources(recipe_id);
    let accounts = tokens::reward_accounts(authority, &resources);
    assert_account_alignment(&accounts, &resources);
    accounts
}

fn assert_account_alignment(accounts: &[AnchorAccountMeta], resources: &[ResourceKind]) {
    assert_eq!(
        accounts.len(),
        resources.len() * 3,
        "each resource needs mint+metadata+ata"
    );
    for (chunk, resource) in accounts.chunks(3).zip(resources.iter()) {
        assert_eq!(chunk.len(), 3, "missing account triple for recipe resource");
        assert_eq!(
            chunk[0].pubkey,
            tokens::resource_mint(*resource),
            "mint mismatch for recipe resource",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forge_input_returns_raw_material() {
        let resources = recipe_input_resources(1);
        assert_eq!(resources, vec![ResourceKind::Wood]);
    }
}
