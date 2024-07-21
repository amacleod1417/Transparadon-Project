
import axios from 'axios';

import StellarSdk from 'stellar-sdk';

const API_URL = process.env.REACT_APP_API_URL;


export const getDonations = async () => {
  const response = await axios.get(`${API_URL}/donations`);
  return response.data;
};

const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');

export const makeDonation = async (charity, amount) => {
  const sourceKeys = StellarSdk.Keypair.fromSecret('your-secret-key');
  const charityAccount = await server.loadAccount(charity);

  const transaction = new StellarSdk.TransactionBuilder(sourceKeys, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: StellarSdk.Networks.TESTNET,
  })
    .addOperation(StellarSdk.Operation.payment({
      destination: charity,
      asset: StellarSdk.Asset.native(),
      amount: amount.toString(),
    }))
    .setTimeout(30)
    .build();

  transaction.sign(sourceKeys);
  await server.submitTransaction(transaction);

  return transaction;
};
