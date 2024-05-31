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
    pub fn addContributor(env: Env, address: Address) {
        // save the address of the user who deposited in a vec
        const key: &str = "depositors";
        let mut dep: Vec<Address> = env.storage().instance().get(&key).unwrap_or(vec![&env]);
        let addition = vec![&env, address];
        dep.append(&addition);

        // Save the updated list of depositors back to storage
        env.storage().instance().set(&key, &dep);
    }

}

mod test;
