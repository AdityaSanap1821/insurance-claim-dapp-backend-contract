use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define the structure of a medical insurance claim
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Claim {
    pub patient: Addr,
    pub medical_record: String,
    pub is_approved: bool,
}

// Define the claim storage as an Item
static CLAIM: Item<Claim> = Item::new("claim");

// Save a claim in the contract's state
pub fn save_claim(storage: &mut dyn Storage, claim: &Claim) -> cosmwasm_std::StdResult<()> {
    CLAIM.save(storage, claim)
}

// Read the claim from the contract's state
pub fn load_claim(storage: &dyn Storage) -> cosmwasm_std::StdResult<Claim> {
    CLAIM.load(storage)
}
