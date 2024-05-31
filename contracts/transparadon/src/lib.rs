#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec};

#[contract]
pub struct TransparadonContract;

#[contractimpl]
impl TransparadonContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
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

}

mod test;
