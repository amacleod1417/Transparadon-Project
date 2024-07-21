const StellarSdk = require('stellar-sdk');

// Use the test network
const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');
StellarSdk.Networks.TESTNET;

const keypair = StellarSdk.Keypair.fromSecret('ILoveTransPeople');

module.exports = { server, keypair, StellarSdk };
