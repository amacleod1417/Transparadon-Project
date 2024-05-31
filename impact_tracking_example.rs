
use stellar_sdk::{Keypair, Network, Operation, TransactionBuilder, Server};
use std::error::Error;

// Struct to represent a donation
#[derive(Debug)]
struct Donation {
    amount: u64,
    charity: String,
    impact_description: String,
}

// Function to record the impact of a donation on Stellar
 fn record_impact_o_stellar(donation: Donation, source_keypair: &Keypair, server: &Server) -> Result<(), Box<dyn Error>> {
    let transaction = TransactionBuilder::new(server.account(&source_keypair.public_key()).await?, Network::Test)
        .add_operation(Operation::Payment(
            stellar_sdk::PaymentOp {
                destination: "GAUVOIKWKB2E7T5RNZPQTWYXHMPN23A7YZR76Y5SCOGQOQUL3GKFPIRN".to_string(), //stellar wallet address
                asset: stellar_sdk::Asset::new_native(),
                amount: donation.amount.to_string().parse()?,
            },
        ))
        .memo(stellar_sdk::Memo::Text(format!("Charity: {}; Impact: {}", donation.charity, donation.impact_description)))
        .build();

    let signed_transaction = transaction.sign(&[&source_keypair]);
    server.submit_transaction(&signed_transaction).await?;

    println!("Impact of donation recorded on the Stellar blockchain: {:?}", donation);
    Ok(())
}

#[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::horizon_testnet();
    let source_secret = "GBGWWAKCWIRABCTBTP2OLUMM34JGTQ2G5N5ZZ5MED4XERD3Q7CA5INYQ";
    let source_keypair = Keypair::from_secret(source_secret)?;

    let donation = Donation {
        amount: 100,
        charity: "Charity A".to_string(),
        impact_description: "Provided food to 50 families".to_string(),
    };

    record_impact_on_stellar(donation, &source_keypair, &server).await?;

    Ok(())
}