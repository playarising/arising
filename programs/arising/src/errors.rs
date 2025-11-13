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
    #[msg("Invalid collection token account provided")]
    InvalidCollectionTokenAccount,
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
    #[msg("Allocation must contain at least one positive value")]
    EmptyAllocation,
    #[msg("Allocation values must be non-negative")]
    NegativeAllocation,
    #[msg("Not enough core points available for this allocation")]
    InsufficientCorePoints,
    #[msg("Not enough attribute points available for this allocation")]
    InsufficientAttributePoints,
    #[msg("You must reach the required level before choosing a class")]
    ClassLevelRequirement,
    #[msg("Character class already chosen")]
    ClassAlreadyChosen,
    #[msg("Selected class does not match the civilization")]
    InvalidClassForCivilization,
    #[msg("Not enough energy available")]
    InsufficientEnergy,
    #[msg("Energy refill is still on cooldown")]
    EnergyRefillCooldown,
    #[msg("Arithmetic overflow encountered")]
    ArithmeticOverflow,
}
