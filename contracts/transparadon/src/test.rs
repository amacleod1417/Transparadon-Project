#![cfg(test)]
extern crate std;

use crate::{token, CharityContract, CharityContractClient};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn install_token_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "./token/soroban_token_contract.wasm");
    env.deployer().upload_contract_wasm(WASM)
}


#[test]
fn add_new_admin() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);
    client.add_new_admin(&new_admin);

    assert_eq!(client.get_admin(), new_admin);
}





