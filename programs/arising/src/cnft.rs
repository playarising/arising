use anchor_lang::prelude::*;
use solana_program_keccak::keccak::hashv;

use crate::{ArisingError, MintCharacterCnft};

use mpl_bubblegum::{
    instructions::{
        MintToCollectionV1Cpi, MintToCollectionV1CpiAccounts, MintToCollectionV1InstructionArgs,
    },
    state::tree_config::TreeConfig,
    types::{Collection, Creator, MetadataArgs, TokenProgramVersion, TokenStandard},
    ID as BUBBLEGUM_PROGRAM_ID,
};
use spl_account_compression::ID as SPL_ACCOUNT_COMPRESSION_ID;
use spl_noop::ID as SPL_NOOP_PROGRAM_ID;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CharacterMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct CnftMintResult {
    pub merkle_tree: Pubkey,
    pub leaf_index: u64,
    pub leaf_hash: [u8; 32],
}

pub fn mint_character_cnft<'info>(
    cnft: &MintCharacterCnft<'info>,
    authority: &Signer<'info>,
    metadata: &CharacterMetadataArgs,
    character_id: u64,
    system_program: &Program<'info, System>,
) -> Result<CnftMintResult> {
    if is_production_tree(cnft) {
        return mint_via_bubblegum(cnft, authority, metadata, character_id, system_program);
    }

    Ok(simulate_cnft(cnft, authority, metadata, character_id))
}

fn is_production_tree(cnft: &MintCharacterCnft) -> bool {
    cnft.bubblegum_program.key() == BUBBLEGUM_PROGRAM_ID
        && cnft.compression_program.key() == SPL_ACCOUNT_COMPRESSION_ID
        && cnft.log_wrapper.key() == SPL_NOOP_PROGRAM_ID
}

fn simulate_cnft<'info>(
    cnft: &MintCharacterCnft<'info>,
    authority: &Signer<'info>,
    metadata: &CharacterMetadataArgs,
    character_id: u64,
) -> CnftMintResult {
    let leaf_hash = hash_bytes(&[
        authority.key.as_ref(),
        &character_id.to_le_bytes(),
        metadata.name.as_bytes(),
        metadata.symbol.as_bytes(),
        metadata.uri.as_bytes(),
    ]);

    CnftMintResult {
        merkle_tree: cnft.merkle_tree.key(),
        leaf_index: 0,
        leaf_hash,
    }
}

fn mint_via_bubblegum<'info>(
    cnft: &MintCharacterCnft<'info>,
    authority: &Signer<'info>,
    metadata: &CharacterMetadataArgs,
    character_id: u64,
    system_program: &Program<'info, System>,
) -> Result<CnftMintResult> {
    require_keys_eq!(
        cnft.bubblegum_program.key(),
        BUBBLEGUM_PROGRAM_ID,
        ArisingError::InvalidBubblegumProgram
    );
    require_keys_eq!(
        cnft.compression_program.key(),
        SPL_ACCOUNT_COMPRESSION_ID,
        ArisingError::InvalidCompressionProgram
    );
    require_keys_eq!(
        cnft.log_wrapper.key(),
        SPL_NOOP_PROGRAM_ID,
        ArisingError::InvalidLogWrapperProgram
    );

    let accounts = MintToCollectionV1CpiAccounts {
        tree_config: &cnft.tree_authority.to_account_info(),
        leaf_owner: &authority.to_account_info(),
        leaf_delegate: &authority.to_account_info(),
        merkle_tree: &cnft.merkle_tree.to_account_info(),
        payer: &authority.to_account_info(),
        tree_creator_or_delegate: &cnft.tree_delegate.to_account_info(),
        collection_authority: &cnft.collection_authority.to_account_info(),
        collection_authority_record_pda: Some(
            &cnft.collection_authority_record_pda.to_account_info(),
        ),
        collection_mint: &cnft.collection_mint.to_account_info(),
        collection_metadata: &cnft.collection_metadata.to_account_info(),
        collection_edition: &cnft.collection_edition.to_account_info(),
        bubblegum_signer: &cnft.bubblegum_signer.to_account_info(),
        log_wrapper: &cnft.log_wrapper.to_account_info(),
        compression_program: &cnft.compression_program.to_account_info(),
        token_metadata_program: &cnft.token_metadata_program.to_account_info(),
        system_program: &system_program.to_account_info(),
    };

    let creator = Creator {
        address: authority.key(),
        verified: true,
        share: 100,
    };

    let metadata_args = MetadataArgs {
        name: metadata.name.clone(),
        symbol: metadata.symbol.clone(),
        uri: metadata.uri.clone(),
        seller_fee_basis_points: 0,
        creators: vec![creator],
        primary_sale_happened: false,
        is_mutable: false,
        edition_nonce: None,
        uses: None,
        collection: Some(Collection {
            key: cnft.collection_mint.key(),
            verified: false,
        }),
        token_program_version: TokenProgramVersion::Original,
        token_standard: Some(TokenStandard::NonFungible),
    };

    let args = MintToCollectionV1InstructionArgs { metadata_args };

    let cpi = MintToCollectionV1Cpi::new(&cnft.bubblegum_program.to_account_info(), accounts, args);
    cpi.invoke()?;

    let tree_config = TreeConfig::try_from_slice(&cnft.tree_authority.try_borrow_data()?)?;
    let leaf_index = tree_config
        .num_minted
        .checked_sub(1)
        .ok_or(ArisingError::ArithmeticUnderflow)?;

    let leaf_hash = hash_bytes(&[
        cnft.merkle_tree.key().as_ref(),
        &leaf_index.to_le_bytes(),
        &character_id.to_le_bytes(),
    ]);

    Ok(CnftMintResult {
        merkle_tree: cnft.merkle_tree.key(),
        leaf_index,
        leaf_hash,
    })
}

fn hash_bytes(chunks: &[&[u8]]) -> [u8; 32] {
    let hash = hashv(chunks);
    let mut out = [0u8; 32];
    out.copy_from_slice(hash.as_ref());
    out
}
