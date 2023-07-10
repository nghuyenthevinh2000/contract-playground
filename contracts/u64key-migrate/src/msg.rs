use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Uint64};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {
        slot: Uint64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(CountResponse)]
    GetCount {
        slot: Uint64,
    },

    #[returns(HelloResponse)]
    Hello {}
}

// We define a custom struct for each query response
#[cw_serde]
pub struct CountResponse {
    pub count: Uint128,
}

#[cw_serde]
pub struct HelloResponse {
    pub prompt: String,
}

#[cw_serde]
pub struct MigrateMsg {
}