#![no_std]
extern crate alloc;

use soroban_sdk::{contract, contracttype, contractimpl, vec, Address, Env, Symbol, Vec};
use alloc::string::String;
use alloc::format;

#[contract]
pub struct TransparadonContract;

#[allow(dead_code)]
pub struct Candidate {
    address: Address,
    votes: u64,
}

#[allow(dead_code)]
pub struct DonationStage {
    current_generation: u64,
}

// The DataKey enum is used to represent state variables stored in the contract's storage.
// This allows for structured access to data within the contract's storage.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Contributions(Address),
    Contributors,
    Token,
    ShareToken,
    IsActive,
    Admin,
    Initialized,
    IsContributor(Address),
    VotingPower(Address),
    CharityVotes(Address),
    DistributionSchedule,
    Reports,
    DonationImpact,
}


#[contractimpl]
impl TransparadonContract {

    pub fn donate(env: Env, amount: u64, user: Address) ->Symbol {
        // logs::log(&format!("Donation of {} from {}", amount, env.caller()));
        if amount <= 0 {
            panic!("Invalid donation amount")
        }

        let is_contributor: bool = env.storage().instance().get(&DataKey::IsContributor(user.clone())).unwrap_or(false);

        if is_contributor != true { // wait what
            panic!("User is not a valid contributor")
        }

        // add their address to the contributors list
        let mut contributors: Vec<Address> = env
                .storage()
                .instance()
                .get(&DataKey::Contributors)
                .unwrap_or(vec![&env, env.current_contract_address()]);
            contributors.push_back(user.clone());
            env.storage()
                .instance()
                .set(&DataKey::Contributors, &contributors);
            Self::record_impact(env, user, amount)
    } 

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
    
    pub fn vote(env: Env, user : Address, charity: Address, votes: u64) {
        // how much power does the current voter have?       

        let voting_power = env.storage().instance().get(&DataKey::VotingPower(user.clone())).unwrap_or(0);
        if voting_power == 0 || votes > voting_power {
            panic!("User has insufficient voting power");
        }

        let is_contributor: bool = env.storage().instance().get(&DataKey::IsContributor(user.clone())).unwrap_or(false);
        if !is_contributor {
            panic!("User is not a valid contributor")
        }

        // add the voting power to the contributor
        let _contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Contributors)
            .unwrap_or(vec![&env]);
        env.storage().instance().set(&DataKey::CharityVotes(user.clone()), &1);

        let charity_votes: u64 =
            env
            .storage()
            .instance()
            .get(&DataKey::CharityVotes(charity.clone()))
            .unwrap_or(0);
        env
            .storage()
            .instance()
            .set(&DataKey::CharityVotes(charity.clone()), &(charity_votes+votes));
        env
            .storage()
            .instance()
            .set(&DataKey::VotingPower(user.clone()), &(voting_power - votes));

            env.events().publish(("vote", &user, votes, &charity), None::<()>);
    }

    // Contribute to the fund, and store the address of the user who contributed
    pub fn add_contributor(env: Env, contributor: Address) {
       // const KEY: &str = "contributors";
        let mut contributors = env
                    .storage()
                    .instance()
                    .get(&DataKey::Contributors)
                    .unwrap_or(vec![&env]);
                contributors.push_back(contributor.clone());
                env.storage()
                    .instance()
                    .set(&DataKey::Contributors, &contributors);
                env.storage()
                    .instance()
                    .set(&DataKey::IsContributor(contributor), &true);
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

pub fn get_recipient_balance(env: Env, user: Address, token:Address ) -> i128 {
    token::Client::new(&env, &token ).balance(&user)
 }

 pub fn set_distribution_schedule(env: Env, schedule: Vec<(Address, u64)>) {
    env.storage()
        .instance()
        .set(&DataKey::DistributionSchedule, &schedule);
 }
 
 pub fn distribute_funds(env: Env, total_funds: u64) {
    let charities: Vec<Address> =
        env
        .storage()
        .instance()
        .get(&DataKey::Contributors)
        .unwrap_or(vec![&env]);
    let mut total_votes: u64 = 0;

    for charity in charities.iter() {
        let charity_votes: u64 =
        env
        .storage()
        .instance()
        .get(&DataKey::CharityVotes(charity.clone()))
        .unwrap_or(0);
        total_votes += charity_votes;
    }
    for charity in charities.iter() {
        let charity_votes: u64 =
        env
        .storage()
        .instance()
        .get(&DataKey::CharityVotes(charity.clone()))
        .unwrap_or(0);
    if total_votes > 0 {
        let share: i128 = ((charity_votes as i128 * total_funds as i128) / total_votes as i128) as i128;
        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::Client::new(&env, &token_address).transfer(&env.current_contract_address(), &charity, &share);
        //env.logs().debug(&format!("Distributed {} to {}", share, charity));
    }
    }
 }

 pub fn generate_report(env: Env, generation: u64) -> Symbol {
    let contributors: Vec<Address> =
        env.storage().instance().get(&DataKey::Contributors).unwrap_or(vec![&env]);
    let total_contributors: u64 = contributors.iter().map(|c| {
        env.storage().instance().get(&DataKey::Contributions(c.clone())).unwrap_or(0)
    }).sum();

    let report = format!("Generation {}: Total Contributions: {}", generation, total_contributors);
    env
        .storage()
        .instance()
        .set(&DataKey::Reports, &report);
    Symbol::new(&env, "Report Generated")
}

pub fn get_report(env: Env) -> Symbol {
    let report: String = 
    env
    .storage()
    .instance()
    .get(&DataKey::Reports)
    .unwrap_or(String::new());
Symbol::new(&env, &report)
 }

 fn record_impact(env: Env, user: Address, amount: u64) -> Symbol {
    env.storage().instance().set(&DataKey::IsContributor(user.clone()), &true);

    let current_contribution: u64 = env.storage().instance().get(&DataKey::Contributions(user.clone())).unwrap_or(0);
    env.storage().instance().set(&DataKey::Contributions(user), &(current_contribution + amount));
    Symbol::new(&env, "Thanks Chief")
}

}

mod test;
mod token;