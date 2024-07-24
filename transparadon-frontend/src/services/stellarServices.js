import StellarSdk from 'stellar-sdk';

//use the test network
const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');
StellarSdk.Networks.TESTNET;

const keypair = StellarSdk.Keypair.fromSecret('YOUR_SECRET_KEY');

const createAccount = async (destination) => {
  const account = await server.loadAccount(keypair.publicKey());

  const transaction = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: StellarSdk.Networks.TESTNET,
  })
    .addOperation(StellarSdk.Operation.createAccount({
      destination,
      startingBalance: '100', // Starting balance in Lumens
    }))
    .setTimeout(30)
    .build();

  transaction.sign(keypair);
  await server.submitTransaction(transaction);
};

const makePayment = async (destination, amount) => {
  const account = await server.loadAccount(keypair.publicKey());

  const transaction = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: StellarSdk.Networks.TESTNET,
  })
    .addOperation(StellarSdk.Operation.payment({
      destination,
      asset: StellarSdk.Asset.native(),
      amount,
    }))
    .setTimeout(30)
    .build();

  transaction.sign(keypair);
  await server.submitTransaction(transaction);
};

export { createAccount, makePayment };
