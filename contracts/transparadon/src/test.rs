#![cfg(test)]
extern crate std;

use super::{token, CharityContract, CharityContractClient};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn install_token_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "./token/soroban_token_contract.wasm");
    env.deployer().upload_contract_wasm(WASM)
}

#[test]
fn initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_reward_token(), deposit_token.address);
    assert!(client.check_campaign_status());
}

#[test]
fn contribute() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contributor = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    deposit_token.mint(&contributor, &1000);
    assert_eq!(deposit_token.balance(&contributor), 1000);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);
    client.contribute(&contributor, &deposit_token.address, &100);

    assert_eq!(client.get_contribution(&contributor), 100);
    assert_eq!(client.get_total_contributions(), 100);
}

#[test]
fn withdraw() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contributor = Address::generate(&env);
    let recipient = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    deposit_token.mint(&contributor, &1000);
    assert_eq!(deposit_token.balance(&contributor), 1000);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);
    client.contribute(&contributor, &deposit_token.address, &100);
    client.withdraw(&contributor, &recipient, &deposit_token.address);

    assert_eq!(client.get_contribution(&contributor), 0);
    assert_eq!(client.get_total_contributions(), 0);
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

#[test]
fn clear_contributor() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contributor = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);
    client.contribute(&contributor, &deposit_token.address, &100);
    client.clear_contributor(&contributor);

    assert_eq!(client.get_contribution(&contributor), 0);
}

#[test]
fn get_contributors() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contributor1 = Address::generate(&env);
    let contributor2 = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);
    client.contribute(&contributor1, &deposit_token.address, &100);
    client.contribute(&contributor2, &deposit_token.address, &200);

    let contributors = client.get_contributors();
    assert_eq!(contributors.len(), 2);
    assert!(contributors.contains(&contributor1));
    assert!(contributors.contains(&contributor2));
}

#[test]
fn get_total_contributions() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contributor1 = Address::generate(&env);
    let contributor2 = Address::generate(&env);
    let token_wasm_hash = install_token_wasm(&env);
    let deposit_token = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, CharityContract);
    let client = CharityContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_wasm_hash, &deposit_token.address);
    client.contribute(&contributor1, &deposit_token.address, &100);
    client.contribute(&contributor2, &deposit_token.address, &200);

    assert_eq!(client.get_total_contributions(), 300);
}
