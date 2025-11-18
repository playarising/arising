//! Coverage for PDA helpers used by NFT/collection code.

use anchor_lang::solana_program::pubkey::Pubkey;
use arising::nft::{master_edition_pda, metadata_pda};

#[test]
fn metadata_and_master_pdas_resolve() {
    let mint = Pubkey::new_unique();
    // Use token metadata program id from crate to ensure paths hit code.
    let program_id = Pubkey::new_unique();
    let metadata = metadata_pda(&mint, &program_id);
    let edition = master_edition_pda(&mint, &program_id);
    assert_ne!(metadata, edition);
}
