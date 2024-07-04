const StellarSdk = require('@stellar/sdk');
const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');

const Donation = require('../models/Donation');
const TransparadonContract = require('../../contracts/src/lib.rs')
const { Keypair } = require('stellar-sdk');

exports.donate = async (req, res) => {
    try {
    const { amount, user } = req.body;
   
    if (amount <= 0) {
        return res.status(400).json({ message: 'Invalid donation amount' });
    }

        const newDonation = new Donation({
            amount,
            user
        });

        await newDonation.save();

        const userAddress=Keypair.fromPublicKey(user).publicKey();
        const env = {};
        const result = await TransparadonContract.donate(env, amount, userAddress);

    return res.status(200).json({ message: 'Donation Recorded', userAddress, amount});
} catch (error) {
    return res.status(500).send('Server Error');
}
};

exports.addContributor = async (req, res) => {
    try {
        const { contributor } = req.body;
        const contributorAddress = Keypair.fromPublicKey(contributor).publicKey();
        const env = {};
        await TransparadonContract.add_contributor(env, contributorAddress);

        return res.status(200).json({ message: 'Contributor added'});
    } catch (error) {
        return res.status(500).send('Error adding contributor');
    }
};

exports.recordImpact = async (req, res) => {
    try {
        const { user, amount } = req.body;

        const userAddress = Keypair.fromPublicKey(user).publicKey();
        const env = {};
        await TransparadonContract.recordImpact(env, userAddress, amount);

        return res.status(500).json( { message: 'Impact recorded'});
    } catch (error) {
        return res.status(500).send('Error adding contributor');
    }
};

exports.getContributors = async (req, res) => {
    try {
        const env = {}; // Set up your environment as needed for your smart contract
        const contributors = await TransparadonContract.get_contributors(env);

        return res.status(200).json({ contributors });
    } catch (error) {
        console.error('Error getting contributors:', error);
        return res.status(500).json({ message: 'Failed to get contributors' });
    }
};

exports.getReport = async (req, res) => {
    try {
        const env = {}; // Set up your environment as needed for your smart contract
        const report = await TransparadonContract.get_report(env);

        return res.status(200).json({ report });
    } catch (error) {
        console.error('Error getting report:', error);
        return res.status(500).json({ message: 'Failed to get report' });
    }
};

exports.vote = async (req, res) => {
    try {
        const { user, charity, votes } = req.body;

        // Call vote function in your smart contract
        const userAddress = Keypair.fromPublicKey(user).publicKey();
        const charityAddress = Keypair.fromPublicKey(charity).publicKey();
        const env = {}; // Set up your environment as needed for your smart contract
        const result = await TransparadonContract.vote(env, userAddress, charityAddress, votes);

        return res.status(200).json({ message: 'Vote successful', result });
    } catch (error) {
        console.error('Error voting:', error);
        return res.status(500).json({ message: 'Failed to vote' });
    }
};

module.exports = {
    donate
};