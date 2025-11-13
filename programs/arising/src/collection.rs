use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program_pack::Pack, system_instruction, sysvar},
};
use anchor_spl::token::{self, spl_token};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{CollectionDetails, Creator, DataV2},
    ID as TOKEN_METADATA_PROGRAM_ID,
};

use crate::{
    character::{CHARACTER_MINT_STATE_SEED, COLLECTION_MINT_PDA_SEED, COLLECTION_TOKEN_PDA_SEED},
    errors::ArisingError,
    nft::{master_edition_pda, metadata_pda},
    Initialize,
};

pub fn initialize_collection(
    accounts: &mut Initialize<'_>,
    program_id: &Pubkey,
    mint_state_bump: u8,
    collection_mint_bump: u8,
    collection_token_bump: u8,
    collection_name: String,
    collection_symbol: String,
    collection_uri: String,
) -> Result<()> {
    validate_collection_pdas(accounts, program_id)?;

    let mint_state_signer: &[&[&[u8]]] = &[&[CHARACTER_MINT_STATE_SEED, &[mint_state_bump]]];

    ensure_collection_mint(accounts, collection_mint_bump)?;
    ensure_collection_token_account(accounts, collection_token_bump, mint_state_signer)?;
    ensure_collection_metadata(
        accounts,
        mint_state_signer,
        collection_name,
        collection_symbol,
        collection_uri,
    )?;

    accounts.mint_state.initialize(
        mint_state_bump,
        accounts.authority.key(),
        accounts.collection_mint.key(),
        accounts.collection_metadata.key(),
        accounts.collection_master_edition.key(),
    );

    Ok(())
}

fn validate_collection_pdas(accounts: &Initialize<'_>, program_id: &Pubkey) -> Result<()> {
    let (expected_collection_mint, _) =
        Pubkey::find_program_address(&[COLLECTION_MINT_PDA_SEED], program_id);
    let (expected_collection_token, _) =
        Pubkey::find_program_address(&[COLLECTION_TOKEN_PDA_SEED], program_id);

    require_keys_eq!(
        accounts.collection_mint.key(),
        expected_collection_mint,
        ArisingError::InvalidCollectionMint
    );
    require_keys_eq!(
        accounts.collection_token_account.key(),
        expected_collection_token,
        ArisingError::InvalidCollectionTokenAccount
    );

    let metadata_program = Pubkey::new_from_array(TOKEN_METADATA_PROGRAM_ID.to_bytes());
    require_keys_eq!(
        accounts.token_metadata_program.key(),
        metadata_program,
        ArisingError::InvalidTokenMetadataProgram
    );

    let expected_metadata = metadata_pda(&accounts.collection_mint.key(), &metadata_program);
    let expected_master = master_edition_pda(&accounts.collection_mint.key(), &metadata_program);

    require_keys_eq!(
        accounts.collection_metadata.key(),
        expected_metadata,
        ArisingError::InvalidCollectionMetadata
    );
    require_keys_eq!(
        accounts.collection_master_edition.key(),
        expected_master,
        ArisingError::InvalidCollectionMasterEdition
    );

    Ok(())
}

fn ensure_collection_mint(accounts: &Initialize<'_>, collection_mint_bump: u8) -> Result<()> {
    if accounts.collection_mint.to_account_info().data_is_empty() {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(spl_token::state::Mint::LEN);

        let create_ix = system_instruction::create_account(
            &accounts.authority.key(),
            &accounts.collection_mint.key(),
            lamports,
            spl_token::state::Mint::LEN as u64,
            &accounts.token_program.key(),
        );
        let collection_mint_signer: &[&[u8]] = &[COLLECTION_MINT_PDA_SEED, &[collection_mint_bump]];
        invoke_signed(
            &create_ix,
            &[
                accounts.authority.to_account_info(),
                accounts.collection_mint.to_account_info(),
                accounts.system_program.to_account_info(),
            ],
            &[collection_mint_signer],
        )?;

        token::initialize_mint(
            CpiContext::new(
                accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: accounts.collection_mint.to_account_info(),
                    rent: accounts.rent.to_account_info(),
                },
            ),
            0,
            &accounts.mint_state.key(),
            Some(&accounts.mint_state.key()),
        )?;
    }

    Ok(())
}

fn ensure_collection_token_account(
    accounts: &Initialize<'_>,
    collection_token_bump: u8,
    mint_state_signer: &[&[&[u8]]],
) -> Result<()> {
    if accounts
        .collection_token_account
        .to_account_info()
        .data_is_empty()
    {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(spl_token::state::Account::LEN);

        let create_ix = system_instruction::create_account(
            &accounts.authority.key(),
            &accounts.collection_token_account.key(),
            lamports,
            spl_token::state::Account::LEN as u64,
            &accounts.token_program.key(),
        );
        let collection_token_signer: &[&[u8]] =
            &[COLLECTION_TOKEN_PDA_SEED, &[collection_token_bump]];
        invoke_signed(
            &create_ix,
            &[
                accounts.authority.to_account_info(),
                accounts.collection_token_account.to_account_info(),
                accounts.system_program.to_account_info(),
            ],
            &[collection_token_signer],
        )?;

        token::initialize_account(CpiContext::new(
            accounts.token_program.to_account_info(),
            token::InitializeAccount {
                account: accounts.collection_token_account.to_account_info(),
                mint: accounts.collection_mint.to_account_info(),
                authority: accounts.authority.to_account_info(),
                rent: accounts.rent.to_account_info(),
            },
        ))?;

        token::mint_to(
            CpiContext::new_with_signer(
                accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: accounts.collection_mint.to_account_info(),
                    to: accounts.collection_token_account.to_account_info(),
                    authority: accounts.mint_state.to_account_info(),
                },
                mint_state_signer,
            ),
            1,
        )?;
    }

    Ok(())
}

fn ensure_collection_metadata(
    accounts: &Initialize<'_>,
    mint_state_signer: &[&[&[u8]]],
    collection_name: String,
    collection_symbol: String,
    collection_uri: String,
) -> Result<()> {
    let metadata_data = DataV2 {
        name: collection_name,
        symbol: collection_symbol,
        uri: collection_uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
            address: accounts.mint_state.key(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    if accounts.collection_metadata.data_is_empty() {
        let metadata_ix = CreateMetadataAccountV3 {
            metadata: accounts.collection_metadata.key(),
            mint: accounts.collection_mint.key(),
            mint_authority: accounts.mint_state.key(),
            payer: accounts.authority.key(),
            update_authority: (accounts.mint_state.key(), true),
            system_program: anchor_lang::solana_program::system_program::ID,
            rent: Some(sysvar::rent::id()),
        }
        .instruction(CreateMetadataAccountV3InstructionArgs {
            data: metadata_data,
            is_mutable: false,
            collection_details: Some(CollectionDetails::V1 { size: 0 }),
        });

        let metadata_infos = [
            accounts.collection_metadata.to_account_info(),
            accounts.collection_mint.to_account_info(),
            accounts.mint_state.to_account_info(),
            accounts.authority.to_account_info(),
            accounts.mint_state.to_account_info(),
            accounts.system_program.to_account_info(),
            accounts.rent.to_account_info(),
            accounts.token_metadata_program.to_account_info(),
        ];
        invoke_signed(&metadata_ix, &metadata_infos, mint_state_signer)?;
    }

    if accounts.collection_master_edition.data_is_empty() {
        let master_ix = CreateMasterEditionV3 {
            edition: accounts.collection_master_edition.key(),
            mint: accounts.collection_mint.key(),
            update_authority: accounts.mint_state.key(),
            mint_authority: accounts.mint_state.key(),
            payer: accounts.authority.key(),
            metadata: accounts.collection_metadata.key(),
            token_program: accounts.token_program.key(),
            system_program: anchor_lang::solana_program::system_program::ID,
            rent: Some(sysvar::rent::id()),
        }
        .instruction(CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        });

        let master_infos = [
            accounts.collection_master_edition.to_account_info(),
            accounts.collection_mint.to_account_info(),
            accounts.mint_state.to_account_info(),
            accounts.mint_state.to_account_info(),
            accounts.authority.to_account_info(),
            accounts.collection_metadata.to_account_info(),
            accounts.token_program.to_account_info(),
            accounts.system_program.to_account_info(),
            accounts.rent.to_account_info(),
            accounts.token_metadata_program.to_account_info(),
        ];
        invoke_signed(&master_ix, &master_infos, mint_state_signer)?;
    }

    Ok(())
}
