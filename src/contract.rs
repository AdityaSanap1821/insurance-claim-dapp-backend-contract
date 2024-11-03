use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{load_claim, save_claim, Claim};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    testing::{message_info, mock_dependencies, mock_env},
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
};

// Instantiate contract (initial setup)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Placeholder for any setup logic if needed later
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Handle contract execution (submit/approve claims)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SubmitClaim {
            patient,
            medical_record,
        } => execute_submit_claim(deps, info, patient, medical_record),
        ExecuteMsg::ApproveClaim { admin } => execute_approve_claim(deps, info, admin),
    }
}

// Submit a new claim
pub fn execute_submit_claim(
    deps: DepsMut,
    info: MessageInfo,
    patient: String,
    medical_record: String,
) -> Result<Response, ContractError> {
    let patient_addr = deps.api.addr_validate(&patient)?;

    // Check if a claim already exists to avoid overwriting
    if load_claim(deps.storage).is_ok() {
        return Err(ContractError::ClaimAlreadyExists {});
    }

    let claim = Claim {
        patient: patient_addr,
        medical_record,
        is_approved: false,
    };

    save_claim(deps.storage, &claim)?;

    Ok(Response::new().add_attribute("method", "submit_claim"))
}

// Approve an existing claim
pub fn execute_approve_claim(
    deps: DepsMut,
    info: MessageInfo,
    admin: String,
) -> Result<Response, ContractError> {
    // Ensure the caller is the correct admin
    let admin_addr = deps.api.addr_validate(&admin)?;
    let sender = info.sender.clone();

    if sender != admin_addr {
        return Err(ContractError::Unauthorized {});
    }

    let mut claim = load_claim(deps.storage)?;

    if claim.is_approved {
        return Err(ContractError::ClaimAlreadyApproved {});
    }

    claim.is_approved = true;
    save_claim(deps.storage, &claim)?;

    Ok(Response::new().add_attribute("method", "approve_claim"))
}

// Query logic (empty for now, can be expanded later)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    unimplemented!()
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{attr, Addr, StdError};

    // Test instantiation of the contract
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = message_info(&Addr::unchecked("cosmos1creator"), &[]); // Valid Bech32 address
        let env = mock_env(); // Simulate environment

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(res.attributes, vec![attr("method", "instantiate")]);
    }

    // Test submitting a new claim
    #[test]
    fn test_submit_claim() {
        let mut deps = mock_dependencies();

        // Instantiate the contract first
        let msg = InstantiateMsg {};
        let info = message_info(&Addr::unchecked("cosmos1creator"), &[]); // Valid Bech32 address
        let env = mock_env();

        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Submit a new claim
        let patient = "cosmos1patient"; // Use valid Bech32 address
        let medical_record = String::from("medical_record_data");
        let msg = ExecuteMsg::SubmitClaim {
            patient: patient.to_string(),
            medical_record: medical_record.clone(),
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes, vec![attr("method", "submit_claim")]);

        // Check if the claim was saved correctly
        let claim = load_claim(&deps.storage).unwrap();
        assert_eq!(claim.patient, Addr::unchecked(patient)); // Check for correct Addr
        assert_eq!(claim.medical_record, medical_record);
        assert_eq!(claim.is_approved, false);
    }

    // Test failure when trying to submit a duplicate claim
    #[test]
    fn test_submit_duplicate_claim() {
        let mut deps = mock_dependencies();

        // Instantiate the contract
        let msg = InstantiateMsg {};
        let info = message_info(&Addr::unchecked("cosmos1creator"), &[]); // Valid Bech32 address
        let env = mock_env();
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Submit a claim
        let patient = "cosmos1patient"; // Use valid Bech32 address
        let medical_record = String::from("medical_record_data");
        let submit_msg = ExecuteMsg::SubmitClaim {
            patient: patient.to_string(),
            medical_record: medical_record.clone(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), submit_msg).unwrap();

        // Try to submit the same claim again
        let duplicate_claim_msg = ExecuteMsg::SubmitClaim {
            patient: patient.to_string(),
            medical_record: medical_record.clone(),
        };
        let err = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            duplicate_claim_msg,
        )
        .unwrap_err();
        assert_eq!(err, ContractError::ClaimAlreadyExists {});
    }

    // Test approving a claim
    #[test]
    fn test_approve_claim() {
        let mut deps = mock_dependencies();

        // Instantiate and submit a claim first
        let msg = InstantiateMsg {};
        let info = message_info(&Addr::unchecked("cosmos1creator"), &[]); // Valid Bech32 address
        let env = mock_env();
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let patient = "cosmos1patient"; // Use valid Bech32 address
        let medical_record = String::from("medical_record_data");
        let submit_msg = ExecuteMsg::SubmitClaim {
            patient: patient.to_string(),
            medical_record: medical_record.clone(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), submit_msg).unwrap();

        // Approve the claim
        let approve_msg = ExecuteMsg::ApproveClaim {
            admin: String::from("cosmos1creator"), // Valid Bech32 address for admin
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), approve_msg).unwrap();
        assert_eq!(res.attributes, vec![attr("method", "approve_claim")]);

        // Check if the claim's approval status changed
        let claim = load_claim(&deps.storage).unwrap();
        assert!(claim.is_approved);
    }

    // Test failure when trying to approve an already approved claim
    #[test]
    fn test_double_approval_error() {
        let mut deps = mock_dependencies();

        // Instantiate and submit a claim first
        let msg = InstantiateMsg {};
        let info = message_info(&Addr::unchecked("cosmos1creator"), &[]); // Valid Bech32 address
        let env = mock_env();
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let patient = "cosmos1patient"; // Use valid Bech32 address
        let medical_record = String::from("medical_record_data");
        let submit_msg = ExecuteMsg::SubmitClaim {
            patient: patient.to_string(),
            medical_record: medical_record.clone(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), submit_msg).unwrap();

        // Approve the claim
        let approve_msg = ExecuteMsg::ApproveClaim {
            admin: String::from("cosmos1creator"), // Valid Bech32 address for admin
        };
        execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            approve_msg.clone(),
        )
        .unwrap();

        // Try to approve the same claim again
        let err = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            approve_msg.clone(),
        )
        .unwrap_err();
        assert_eq!(err, ContractError::ClaimAlreadyApproved {});
    }

    // Test unauthorized approval attempt
    #[test]
    fn test_unauthorized_approve() {
        let mut deps = mock_dependencies();

        // Instantiate and submit a claim first
        let msg = InstantiateMsg {};
        let info = message_info(&Addr::unchecked("cosmos1creator"), &[]); // Valid Bech32 address
        let env = mock_env();
        instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let patient = "cosmos1patient"; // Use valid Bech32 address
        let medical_record = String::from("medical_record_data");
        let submit_msg = ExecuteMsg::SubmitClaim {
            patient: patient.to_string(),
            medical_record: medical_record.clone(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), submit_msg).unwrap();

        // Try to approve as a different unauthorized user
        let approve_msg = ExecuteMsg::ApproveClaim {
            admin: String::from("cosmos1unauthorized"), // Use valid Bech32 address
        };

        // Correctly assign info before using it in execute
        let info = message_info(&Addr::unchecked("cosmos1unauthorized"), &[]); // Use valid Bech32 address

        // Execute the approve action and assert for the expected error
        let err = execute(
            deps.as_mut(),
            env.clone(),
            info, // Use the info variable here
            approve_msg,
        )
        .unwrap_err();

        assert_eq!(err, ContractError::Unauthorized {});
    }
}