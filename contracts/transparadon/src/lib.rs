#![no_std]
use core::{f64::consts, ops::Add};

use soroban_sdk::{contract,contracttype, contractimpl, vec, Address, Env, Symbol, Vec, logs};

#[contract]
pub struct TransparadonContract;

pub struct Candidate {
    address: Address,
    votes: u64,
}

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
    VotingPower(Address)
}


#[contractimpl]
impl TransparadonContract {

    pub fn donate(env: Env, amount: u64, user: Address) ->Symbol {
        // logs::log(&format!("Donation of {} from {}", amount, env.caller()));
        if amount <= 0 {
            panic!("Nah")
        }
        // check if address is a valid contributor
        let  contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&"contributors")
            .unwrap_or(vec![&env]);
        let  is_contributor: bool = env.storage().instance().get(&DataKey::IsContributor((user.clone()))).unwrap_or(false);

        if is_contributor != true { // wait what
            panic!("waiiiiit")
        }

        // add their address to the contributors list
        let mut contributors: Vec<Address> = env
                .storage()
                .instance()
                .get(&"contributors")
                .unwrap_or(vec![&env, env.current_contract_address()]);
            contributors.push_back(user.clone());
            env.storage()
                .instance()
                .set(&"contributors", &contributors);
            Self::record_impact(user)
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
    
    pub fn vote(env: Env,user : Address) {
        // how much power does the current voter have?
        env.storage();
            .instance()
            .get(&candidatecandidate)
            .unwrap_or(map![&env, (env.)]);

        let mut contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Contributors)
            .unwrap_or(vec![&env]);

        let mut found = false;
        let  is_contributor: bool = env.storage().instance().get(&DataKey::IsContributor((user.clone()))).unwrap_or(false);

        

        // let mut voting_power = TransparadonContract::calculate_quadratic_voting_power(voting_power);
        // update contributor stats:
        // fella is tryna do nothing, or is not a contributor
        // if voting_power <= 0 || contributors.is_empty(){
        //     return;
        // }

        // add the voting power to the contributor
        let mut contributors: Vec<Address> = env
            .storage()
            .instance()
            .get(&"contributors")
            .unwrap_or(vec![&env]);
        // let mut new_contributors: Vec<Address> = vec![&env];
        // let mut voting_power: u64;
        
            contributors.push_back(user);
            env.storage().instance().set(&DataKey::Contributors, &contributors);
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
pub fn get_recipient_balance(env: Env, user: Address, token:Address ) -> u64 {
    token::Client::new(&env, &token ).balance(&user)
 }
    
// // Function to record the impact of a donation on Stellar
// fn record_impact_o_stellar(donation: Donation, source_keypair: &Keypair, server: &Server) -> Result<(), Box<dyn Error>> {
//     let transaction = TransactionBuilder::new(server.account(&source_keypair.public_key()).await?, Network::Test)
//         .add_operation(Operation::Payment(
//             stellar_sdk::PaymentOp {
//                 destination: "GAUVOIKWKB2E7T5RNZPQTWYXHMPN23A7YZR76Y5SCOGQOQUL3GKFPIRN".to_string(), //stellar wallet address
//                 asset: stellar_sdk::Asset::new_native(),
//                 amount: donation.amount.to_string().parse()?,
//             },
//         ))
//         .memo(stellar_sdk::Memo::Text(format!("Charity: {}; Impact: {}", donation.charity, donation.impact_description)))
//         .build();

//     let signed_transaction = transaction.sign(&[&source_keypair]);
//     server.submit_transaction(&signed_transaction).await?;

//     println!("Impact of donation recorded on the Stellar blockchain: {:?}", donation);
//     Ok(())
// }

fn record_impact( user: Address) -> Symbol {
    env.storge.instance().set(DataKey::IsContributor(user), &true);
    Symbol::new(env, "Thanks Chief")
}

}

mod test;
mod token;