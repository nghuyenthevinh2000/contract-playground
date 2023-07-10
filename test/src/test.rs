use classic_test_tube::{TerraTestApp, Wasm, SigningAccount, Module, Account};
use cosmwasm_std::{Coin, Uint128, Uint64};
use derivative::Derivative;
use anyhow::{Result};
use u64key_migrate::msg::{InstantiateMsg, CountResponse, QueryMsg, MigrateMsg, ExecuteMsg, HelloResponse};

fn contract_old_u64key_migrate() -> Vec<u8> {
    std::fs::read("../artifacts/old_u64key_migrate.wasm").unwrap()
}

fn contract_u64key_migrate() -> Vec<u8> {
    std::fs::read("../artifacts/u64key_migrate.wasm").unwrap()
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Suite<'a> {
    /// Application mock
    app: &'a TerraTestApp,
    /// Wasm mock
    #[derivative(Debug = "ignore")]
    wasm: Wasm<'a, TerraTestApp>,
    /// Special account
    #[derivative(Debug = "ignore")]
    pub owner: SigningAccount,
    /// ID of stored code for old contract
    old_contract_id: u64,
    /// ID of stored code for new contract
    new_contract_id: u64,
}

impl<'a> Suite<'a> {

    pub fn init(app: &'a TerraTestApp) -> Result<Suite<'a>> {
        let wasm = Wasm::new(app);

        // Set balances
        let owner = app.init_account(
            &[
                Coin::new(9999999999999u128, "uusd"),
                Coin::new(9999999999999u128, "uluna"),
            ]
        ).unwrap();

        let old_contract_id = wasm.store_code(&contract_old_u64key_migrate(), None, &owner).unwrap().data.code_id;
        let new_contract_id = wasm.store_code(&contract_u64key_migrate(), None, &owner).unwrap().data.code_id;

        Ok(Suite { app, wasm, owner, old_contract_id, new_contract_id })
    }

}

#[test]
fn migrate_test() {
    let app = TerraTestApp::new();
    let suite = Suite::init(&app).unwrap();

    // Init old contract
    let res = suite.wasm.instantiate(
        suite.old_contract_id,
        &InstantiateMsg { count: Uint128::new(10) },
        Some(&suite.owner.address()),
        Some("contract"),
        &[],
        &suite.owner
    ).unwrap();
    let contract_address = res.data.address;

    // Query before migration
    let pre_res = suite.wasm.query::<_, CountResponse>(
        &contract_address, 
        &QueryMsg::GetCount {
            slot: Uint64::new(0)
        }
    ).unwrap();

    println!("pre_res = {}", pre_res.count);

    // Migrate
    let migrate_msg = MigrateMsg {};
    suite.wasm.migrate(
        &contract_address, 
        &migrate_msg, 
        suite.new_contract_id, 
        &suite.owner
    ).unwrap();

    // Hello after migration
    let hello = suite.wasm.query::<_, HelloResponse>(
        &contract_address, 
        &QueryMsg::Hello {  }
    ).unwrap();

    println!("hello = {}", hello.prompt);
    assert_eq!(hello.prompt, "hello world");

    // Query after migration
    let after_res = suite.wasm.query::<_, CountResponse>(
        &contract_address, 
        &QueryMsg::GetCount {
            slot: Uint64::new(0)
        }
    ).unwrap();

    println!("after_res = {}", after_res.count);
    assert_eq!(pre_res.count, after_res.count);

    // Increment
    suite.wasm.execute(
        &contract_address, 
        &ExecuteMsg::Increment { 
            slot: Uint64::new(0)
        },
        &[], 
        &suite.owner
    ).unwrap();

    let after_res = suite.wasm.query::<_, CountResponse>(
        &contract_address, 
        &QueryMsg::GetCount {
            slot: Uint64::new(0)
        }
    ).unwrap();

    println!("after_res = {}", after_res.count);
    assert_eq!(pre_res.count.wrapping_add(Uint128::one()), after_res.count);
}