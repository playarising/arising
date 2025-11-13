use anchor_lang::prelude::*;

#[error_code]
pub enum ArisingError {
    #[msg("Invalid token metadata program provided")]
    InvalidTokenMetadataProgram,
    #[msg("Invalid metadata PDA provided")]
    InvalidMetadataAccount,
    #[msg("Invalid master edition PDA provided")]
    InvalidMasterEditionAccount,
    #[msg("Invalid collection mint provided")]
    InvalidCollectionMint,
    #[msg("Invalid collection metadata account provided")]
    InvalidCollectionMetadata,
    #[msg("Invalid collection master edition account provided")]
    InvalidCollectionMasterEdition,
    #[msg("Character id must match the next sequential id for the civilization")]
    InvalidCharacterIdSequence,
    #[msg("Civilization cap has been reached")]
    CivilizationCapReached,
    #[msg("Cap cannot be reduced below characters already minted")]
    CapBelowSupply,
    #[msg("Only the contract authority may perform this action")]
    Unauthorized,
    #[msg("Provided character account does not match the requested civilization/id pair")]
    InvalidCharacterAccount,
    #[msg("Arithmetic overflow encountered")]
    ArithmeticOverflow,
}
