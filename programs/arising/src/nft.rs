use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_program, sysvar},
};
use anchor_spl::token;
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs, SetAndVerifySizedCollectionItem,
    },
    types::{Collection, Creator, DataV2},
    ID as TOKEN_METADATA_PROGRAM_ID,
};

use crate::{errors::ArisingError, MintCharacter};

pub const METADATA_SEED: &[u8] = b"metadata";
pub const MASTER_EDITION_SEED: &[u8] = b"edition";

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CharacterMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn metadata_pda(mint: &Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[METADATA_SEED, program_id.as_ref(), mint.as_ref()],
        program_id,
    )
    .0
}

pub fn master_edition_pda(mint: &Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            METADATA_SEED,
            program_id.as_ref(),
            mint.as_ref(),
            MASTER_EDITION_SEED,
        ],
        program_id,
    )
    .0
}

pub fn mint_character_nft<'info>(
    accounts: &MintCharacter<'info>,
    metadata_args: &CharacterMetadataArgs,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let metadata_program = Pubkey::new_from_array(TOKEN_METADATA_PROGRAM_ID.to_bytes());
    require_keys_eq!(
        accounts.token_metadata_program.key(),
        metadata_program,
        ArisingError::InvalidTokenMetadataProgram
    );

    let expected_metadata = metadata_pda(&accounts.character_mint.key(), &metadata_program);
    require_keys_eq!(
        accounts.metadata.key(),
        expected_metadata,
        ArisingError::InvalidMetadataAccount
    );

    let expected_master = master_edition_pda(&accounts.character_mint.key(), &metadata_program);
    require_keys_eq!(
        accounts.master_edition.key(),
        expected_master,
        ArisingError::InvalidMasterEditionAccount
    );

    require_keys_eq!(
        accounts.collection_mint.key(),
        accounts.mint_state.collection_mint,
        ArisingError::InvalidCollectionMint
    );
    require_keys_eq!(
        accounts.collection_metadata.key(),
        accounts.mint_state.collection_metadata,
        ArisingError::InvalidCollectionMetadata
    );
    require_keys_eq!(
        accounts.collection_master_edition.key(),
        accounts.mint_state.collection_master_edition,
        ArisingError::InvalidCollectionMasterEdition
    );

    let creator = Creator {
        address: accounts.authority.key(),
        verified: true,
        share: 100,
    };

    let metadata_data = DataV2 {
        name: metadata_args.name.clone(),
        symbol: metadata_args.symbol.clone(),
        uri: metadata_args.uri.clone(),
        seller_fee_basis_points: 0,
        creators: Some(vec![creator]),
        collection: Some(Collection {
            key: accounts.collection_mint.key(),
            verified: false,
        }),
        uses: None,
    };

    let metadata_key = accounts.metadata.key();
    let mint_key = accounts.character_mint.key();
    let mint_authority_key = accounts.mint_state.key();
    let authority_key = accounts.authority.key();
    let token_program_key = accounts.token_program.key();

    let metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_key,
        mint: mint_key,
        mint_authority: mint_authority_key,
        payer: authority_key,
        update_authority: (authority_key, true),
        system_program: system_program::ID,
        rent: Some(sysvar::rent::id()),
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: false,
        collection_details: None,
    });

    let metadata_infos = [
        accounts.metadata.to_account_info(),
        accounts.character_mint.to_account_info(),
        accounts.mint_state.to_account_info(),
        accounts.authority.to_account_info(),
        accounts.authority.to_account_info(),
        accounts.system_program.to_account_info(),
        accounts.rent.to_account_info(),
        accounts.token_metadata_program.to_account_info(),
    ];
    invoke_signed(&metadata_ix, &metadata_infos, signer_seeds)?;

    let master_ix = CreateMasterEditionV3 {
        edition: accounts.master_edition.key(),
        mint: mint_key,
        update_authority: authority_key,
        mint_authority: mint_authority_key,
        payer: authority_key,
        metadata: metadata_key,
        token_program: token_program_key,
        system_program: system_program::ID,
        rent: Some(sysvar::rent::id()),
    }
    .instruction(CreateMasterEditionV3InstructionArgs {
        max_supply: Some(0),
    });

    let master_infos = [
        accounts.master_edition.to_account_info(),
        accounts.character_mint.to_account_info(),
        accounts.authority.to_account_info(),
        accounts.mint_state.to_account_info(),
        accounts.authority.to_account_info(),
        accounts.metadata.to_account_info(),
        accounts.token_program.to_account_info(),
        accounts.system_program.to_account_info(),
        accounts.rent.to_account_info(),
        accounts.token_metadata_program.to_account_info(),
    ];
    invoke_signed(&master_ix, &master_infos, signer_seeds)?;

    token::mint_to(
        CpiContext::new_with_signer(
            accounts.token_program.to_account_info(),
            token::MintTo {
                mint: accounts.character_mint.to_account_info(),
                to: accounts.character_token_account.to_account_info(),
                authority: accounts.mint_state.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    let verify_ix = SetAndVerifySizedCollectionItem {
        metadata: metadata_key,
        collection_authority: mint_authority_key,
        payer: authority_key,
        update_authority: authority_key,
        collection_mint: accounts.collection_mint.key(),
        collection: accounts.collection_metadata.key(),
        collection_master_edition_account: accounts.collection_master_edition.key(),
        collection_authority_record: None,
    }
    .instruction();

    let verify_infos = vec![
        accounts.metadata.to_account_info(),
        accounts.mint_state.to_account_info(),
        accounts.authority.to_account_info(),
        accounts.authority.to_account_info(),
        accounts.collection_mint.to_account_info(),
        accounts.collection_metadata.to_account_info(),
        accounts.collection_master_edition.to_account_info(),
        accounts.system_program.to_account_info(),
        accounts.token_metadata_program.to_account_info(),
    ];
    invoke_signed(&verify_ix, &verify_infos, signer_seeds)?;

    Ok(())
}
