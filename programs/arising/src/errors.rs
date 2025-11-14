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
    #[msg("Quest is not recognized")]
    InvalidQuest,
    #[msg("Character is already on a quest")]
    QuestAlreadyInProgress,
    #[msg("Character is not on a quest")]
    QuestNotInProgress,
    #[msg("Quest has not finished yet")]
    QuestNotReady,
    #[msg("Quest in progress does not match the requested quest id")]
    QuestMismatch,
    #[msg("Character does not meet the requirements for this quest")]
    QuestRequirementsNotMet,
    #[msg("Invalid quest recipe identifier")]
    InvalidRecipe,
    #[msg("Character is already processing a recipe")]
    RecipeAlreadyInProgress,
    #[msg("Character is not processing a recipe")]
    RecipeNotInProgress,
    #[msg("Recipe has not finished yet")]
    RecipeNotReady,
    #[msg("Recipe in progress does not match the requested recipe id")]
    RecipeMismatch,
    #[msg("Character does not meet the requirements for this recipe")]
    RecipeRequirementsNotMet,
    #[msg("Invalid resource mint provided")]
    InvalidResourceMint,
    #[msg("Invalid resource token account provided")]
    InvalidResourceAccount,
    #[msg("Missing accounts for resource instructions")]
    MissingRewardAccounts,
    #[msg("Missing accounts for recipe processing")]
    MissingRecipeAccounts,
    #[msg("Arithmetic overflow encountered")]
    ArithmeticOverflow,
}
