use cosmwasm_std::{Uint128};
use cw_storage_plus::{Map, U64Key};

pub const TEST_STORAGE: Map<u64, Uint128> = Map::new("test_storage");
