//! Helpers to bridge Anchor and native Solana instruction types.

use anchor_lang::solana_program::instruction::{
    AccountMeta as AnchorAccountMeta, Instruction as AnchorInstruction,
};
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_signer::Signer;

/// Converts an Anchor `Pubkey` into a core Solana `Pubkey`.
pub fn to_solana_pubkey(pk: AnchorPubkey) -> Pubkey {
    Pubkey::from(pk.to_bytes())
}

/// Converts a Solana `Pubkey` back into the Anchor type.
pub fn to_anchor_pubkey(pk: Pubkey) -> AnchorPubkey {
    AnchorPubkey::new_from_array(pk.to_bytes())
}

/// Returns the Anchor-form public key for a given keypair.
pub fn keypair_pubkey(keypair: &solana_keypair::Keypair) -> AnchorPubkey {
    AnchorPubkey::new_from_array(keypair.pubkey().to_bytes())
}

/// Converts an Anchor instruction into a native Solana instruction for LiteSVM.
pub fn convert_instruction(ix: AnchorInstruction) -> Instruction {
    Instruction {
        program_id: to_solana_pubkey(ix.program_id),
        accounts: ix.accounts.into_iter().map(convert_account_meta).collect(),
        data: ix.data,
    }
}

/// Converts an account meta entry between Anchor and native Solana types.
pub fn convert_account_meta(meta: AnchorAccountMeta) -> AccountMeta {
    AccountMeta {
        pubkey: to_solana_pubkey(meta.pubkey),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }
}
