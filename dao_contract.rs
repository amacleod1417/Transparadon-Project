use stellar_sdk::{Keypair, Network, Operation, TransactionBuilder, Server};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;
use tokio::time::Duration;

// Function to get account balance from the Stellar network
async fn get_account_balance(server: &Server, keypair: &Keypair) -> Result<u64, stellar_sdk::Error> {
    let account = server.account(&keypair.public_key()).await?;
    for balance in account.balances.iter() {
        if balance.asset_type == "native" {
            return Ok(balance.balance.parse().unwrap());
        }
    }
    Err(stellar_sdk::Error::Custom("Native balance not found".to_string()))
}

// Function to calculate quadratic voting power based on donation amount
fn calculate_quadratic_voting_power(donation_amount: &BigUint) -> BigUint {
    if donation_amount.is_zero() {
        return BigUint::zero();
    }

    let mut guess = donation_amount.clone() / BigUint::from(2u32) + BigUint::one();
    let mut sqrt_num = guess.clone();
    let mut sqrt_num1 = BigUint::zero();
    loop {
        sqrt_num1 = sqrt_num.clone();
        sqrt_num = (&sqrt_num + &(&donation_amount / &sqrt_num)) / BigUint::from(2u32);

        if sqrt_num.cmp(&sqrt_num1) == Ordering::Equal {
            break;
        }
    }

    sqrt_num
}

// Function to asynchronously vote
async fn vote(server: &Server, source_keypair: &Keypair, charity_votes: Arc<Mutex<HashMap<String, BigUint>>>, charity: &str, voting_power: &BigUint) -> Result<(), stellar_sdk::Error> {
    let transaction = TransactionBuilder::new(server.account(&source_keypair.public_key()).await?, Network::Test)
        .add_operation(Operation::Payment(
            stellar_sdk::PaymentOp {
                destination: charity.to_string(),
                asset: stellar_sdk::Asset::new_native(),
                amount: voting_power.to_string().parse().unwrap(),
            },
        ))
        .build();

    let signed_transaction = transaction.sign(&[&source_keypair]);
    server.submit_transaction(&signed_transaction).await?;

    let mut votes = charity_votes.lock().unwrap();
    *votes.entry(charity.to_string()).or_insert(BigUint::zero()) += voting_power.clone();

    Ok(())
}

// Function to allocate the funds based on the votes
fn allocate_funds(charity_votes: &HashMap<String, BigUint>, total_funds: BigUint) {
    let total_votes: BigUint = charity_votes.values().sum();
    for (charity, votes) in charity_votes.iter() {
        let proportion = votes * &total_funds / &total_votes;
        println!("Allocating {} to charity {}", proportion, charity);
        // Actual transaction logic to transfer proportion amount to charity 
    }
}

async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::horizon_testnet();
    let source_secret =   "GBGWWAKCWIRABCTBTP2OLUMM34JGTQ2G5N5ZZ5MED4XERD3Q7CA5INYQ"; // alice/receivers key
    let source_keypair = Keypair::from_secret(source_secret)?;

    let balance_threshold: u64 = 100;
    let source_balance = get_account_balance(&server, &source_keypair).await?;

    if source_balance < balance_threshold {
        println!("Source account does not have sufficient balance to perform the transaction.");
        return Ok(());
    }

    let donation_amount = BigUint::from(123456u64);
    let voting_power = calculate_quadratic_voting_power(&donation_amount);
    println!("Voting power: {}", voting_power);

    let charity_votes = Arc::new(Mutex::new(HashMap::new()));
    let charities = vec![
        "GBGWWAKCWIRABCTBTP2OLUMM34JGTQ2G5N5ZZ5MED4XERD3Q7CA5INYQ",
        "GCGGWAKCWIRABCTBTP2OLUMM34JGTQ2G5N5ZZ5MED4XERD3Q7CA5INYR"
    ];

    let mut charity_votes_map = HashMap::new();
    for charity in &charities {
        charity_votes_map.insert(charity.to_string(), BigUint::from(0u64));
    }
    let charity_votes = Arc::new(Mutex::new(charity_votes_map));

    // Vote for each charity once based on user's allocation of votes
    for charity in &charities {
        match vote(&server, &source_keypair, Arc::clone(&charity_votes), charity, &voting_power).await {
            Ok(_) => println!("Vote transaction for charity {} successful.", charity),
            Err(e) => eprintln!("Vote transaction for charity {} failed: {}", charity, e),
        }
    }

    // Check if the number of "yes" votes has reached 100
    let votes = charity_votes.lock().unwrap();
    let total_votes: BigUint = votes.values().sum();
    if total_votes >= BigUint::from(100u64) {
        println!("100 'yes' votes reached! Proceeding with allowing the charity...");
        allocate_funds(&votes, BigUint::from(1000u64)); // Example total funds to allocate
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_async().await {
        eprintln!("Error: {}", e);
    }
}
