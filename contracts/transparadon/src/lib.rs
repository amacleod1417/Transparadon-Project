#![no_std]
use core::f64::consts;

use soroban_sdk::{contract, contractimpl, vec, Address, Env, Symbol, Vec, logs};

#[contract]
pub struct TransparadonContract;

pub struct Candidate {
    address: Address,
    votes: u64,
}

pub struct DonationStage {
    current_generation: u64,
}

#[contractimpl]
impl TransparadonContract {

    pub fn donate(env: Env, amount: u64) {
        // logs::log(&format!("Donation of {} from {}", amount, env.caller()));
        if amount <= 0 {
            return;
        }
        // check if address is a valid contributor
        let mut contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&"contributors")
            .unwrap_or(vec![&env]);
        let mut is_contributor = false;
        for c in contributors.iter() {
            if c == env.caller() {
                is_contributor = true;
                break;
            }
        }

        if !is_contributor { // wait what
            return;
        }

        // add their address to the contributors list
        let mut contributors: Vec<Address> = env
                .storage()
                .instance()
                .get(&"contributors")
                .unwrap_or(vec![&env, env.caller().clone()]);
            contributors.push_back(env.caller().clone());
            env.storage()
                .instance()
                .set(&"contributors", &contributors);
    }

    // try with 1000001 lol
    pub fn calculate_quadratic_voting_power(value: u64) -> u64 {
        if value == 0 {
            return 0;
        }

        let mut x = value;
        let mut last_x = 0;

        while x != last_x {
            last_x = x;
            x = (x + value / x) / 2;
        }

        x
    }

    // Voting 🔥
    pub fn vote(env: Env, candidate: Addressu64) {
        // how much power does the current voter have?
        env.storage()
            .instance()
            .get(&"candidate")
            .unwrap_or(map![&env, (env.)]);

        env.current_contract_address();
        let mut contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&"contributors")
            .unwrap_or(vec![&env]);

        let mut found = false;
        for c in contributors.iter() {
            if c == candidate {
                found = true;
                break;
            }
        }
        if !found {
            return;
        }

        let mut voting_power = TransparadonContract::calculate_quadratic_voting_power(voting_power);
        // update contributor stats:
        // fella is tryna do nothing, or is not a contributor
        if voting_power <= 0 || contributors.is_empty(){
            return;
        }

        // add the voting power to the contributor
        let mut contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&"contributors")
            .unwrap_or(vec![&env]);
        let mut new_contributors: Vec<Address> = vec![&env];
        // create or update the contributor

        for c in contributors.iter() {
            if c == env.current_contract_address() {
                new_contributors.push_back(Contributor {
                    address:
                    voting_power: voting_power,
                });
            } else {
                new_contributors.push_back(c.clone());
            }
        }

    }

    // Contribute to the fund, and store the address of the user who contributed
    pub fn add_contributor(env: Env, contributor: Address) {
        const KEY: &str = "contributors";
        let mut contributors = env
                    .storage()
                    .instance()
                    .get(&KEY)
                    .unwrap_or(vec![&env, contributor.clone()]);
                contributors.push_back(contributor);
                env.storage()
                    .instance()
                    .set(&KEY, &contributors);
    }

    pub fn increment_generation(env: Env) {
        // Increment the generation number
        const GEN_KEY: &str = "generation";
        let mut generation: u64 = env
            .storage()
            .instance()
            .get(&GEN_KEY)
            .unwrap_or(0);
        generation += 1;
        env.storage()
            .instance()
            .set(&GEN_KEY, &generation);
        // Reset the contributors
        const CONT_KEY: &str = "contributors";
        let empty: Vec<Address> = vec![&env];
        env
            .storage()
            .instance()
            .set(&CONT_KEY, &empty);
    }

}

mod test;
