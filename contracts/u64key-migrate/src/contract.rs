use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint64};
use cw2::{set_contract_version, get_contract_version};
use cw_storage_plus::U64Key;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg, HelloResponse};
use crate::state::{TEST_STORAGE};

// version info for migration info
const CONTRACT_NAME: &str = "u64key-migrate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    TEST_STORAGE.save(deps.storage, 0u64, &msg.count)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment { slot } => try_increment(deps, slot),
    }
}

pub fn try_increment(deps: DepsMut, slot: Uint64) -> Result<Response, ContractError> {
    TEST_STORAGE.update(deps.storage, slot.u64(), |state| -> Result<_, ContractError> {
        let new_state = state.unwrap().checked_add(Uint128::one()).unwrap();
        Ok(new_state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount { slot } => to_binary(&query_count(deps, slot)?),
        QueryMsg::Hello {  } => to_binary(&hello()?),
    }
}

fn query_count(deps: Deps, slot: Uint64) -> StdResult<CountResponse> {
    let count = TEST_STORAGE.load(deps.storage, slot.u64())?;
    Ok(CountResponse { count: count })
}

fn hello() -> StdResult<HelloResponse> {
    Ok(HelloResponse { prompt: String::from_str("hello world").unwrap() })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps:DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("previous_contract_name", &contract_version.contract)
        .add_attribute("previous_contract_version", &contract_version.version)
        .add_attribute("new_contract_name", CONTRACT_NAME)
        .add_attribute("new_contract_version", CONTRACT_VERSION))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: Uint128::new(17) };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount { slot: Uint64::new(0) }).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count.u128());
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: Uint128::new(17) };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment { slot: Uint64::new(0) };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount { slot: Uint64::new(0) }).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count.u128());
    }
}
