use stellar_sdk::{Keypair, Network, Operation, TransactionBuilder, Server};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::cmp::Ordering;
use tokio::time::{delay_for, Duration};

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

fn calculate_quadratic_voting_power(donation_amount: &BigUint) -> BigUint {
    if donation_amount.is_zero() {
        return BigUint::zero();
    }

    // Initial guess for the square root
    let mut guess = donation_amount.clone() / BigUint::from(2u32) + BigUint::one();

    // Babylonian method for square root calculation
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
async fn vote(server: &Server, source_keypair: &Keypair, voting_power: &BigUint) -> Result<(), stellar_sdk::Error> {
    // Build transaction
    let transaction = TransactionBuilder::new(server.account(&source_keypair.public_key()).await?, Network::Test)
        .add_operation(Operation::Payment(
            stellar_sdk::PaymentOp {
                destination: "GBGWWAKCWIRABCTBTP2OLUMM34JGTQ2G5N5ZZ5MED4XERD3Q7CA5INYQ".to_string(),
                asset: stellar_sdk::Asset::new_native(),
                amount: voting_power.to_string().parse().unwrap(), // Use voting power directly as the amount
            },
        ))
        .build();

    // Sign transaction
    let signed_transaction = transaction.sign(&[&source_keypair]);

    // Submit transaction to the Stellar network
    server.submit_transaction(&signed_transaction).await?;
    Ok(())
}

async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Stellar server (Testnet in this example)
    let server = Server::horizon_testnet();

    // Secret key of the source account
    let source_secret = "GBVK3AJD4GXJ22ZB3JFC27LBUOQWGSHOTN7WHRR2OH42HXJWQFTHL26X"; // alice or source account key
    let source_keypair = Keypair::from_secret(source_secret)?;

    // Check source account balance
    let balance_threshold: u64 = 100; // Minimum balance required
    let source_balance = get_account_balance(&server, &source_keypair).await?;

    // Ensure source account has enough balance
    if source_balance < balance_threshold {
        println!("Source account does not have sufficient balance to perform the transaction.");
        return Ok(());
    }

    // Example donation amount (this should be dynamically obtained in a real application)
    let donation_amount = BigUint::from(123456u64);
    let voting_power = calculate_quadratic_voting_power(&donation_amount);
    println!("Voting power: {}", voting_power);

    // Counter for "yes" votes
    let yes_votes = Arc::new(Mutex::new(BigUint::zero()));

    // Asynchronously vote
        match vote(&server, &source_keypair, &voting_power).await {
            Ok(_) => {
                println!("Vote transaction successful.");

                // Increment the yes vote counter
                let mut votes = yes_votes.lock().unwrap();
                *votes += voting_power.clone();
                println!("Current 'yes' votes: {}", votes);

                // Check if the number of "yes" votes has reached 100
                if *votes >= BigUint::from(100u64) {
                    println!("100 'yes' votes reached! Proceeding with allowing the charity");
                   
                    break;
                }
            },
            Err(e) => eprintln!("Vote transaction failed: {}", e),
        }
        sleep(Duration::from_secs(10)).await;


    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_async().await {
        eprintln!("Error: {}", e);
    }
}
