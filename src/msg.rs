use cosmwasm_schema::{cw_serde, QueryResponses};

// Message for instantiating the contract (empty in our case)
#[cw_serde]
pub struct InstantiateMsg {}

// Message for submitting and approving claims
#[cw_serde]
pub enum ExecuteMsg {
    SubmitClaim {
        patient: String,
        medical_record: String,
    },
    ApproveClaim {
        admin: String,
    },
}

// Queries can be defined later if needed (currently unused)
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
