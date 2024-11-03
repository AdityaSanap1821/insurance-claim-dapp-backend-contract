use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Claim already approved")]
    ClaimAlreadyApproved {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Claim already exists")]
    ClaimAlreadyExists {},

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(u128),

    #[error("Standard error: {0}")]
    Std(#[from] StdError), // Catch standard errors directly
}
