const StellarSdk = require('@stellar/sdk');
const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');



const donate = async (req, res) => {
    try {
    const { userAddress, amount } = req.body;
    //logic to handle donation
    res.status(200).json({ message: 'Donation Recorded', userAddress, amount});
} catch (error) {
    res.status(500).send('Server Error');
}
};

module.exports = {
    donate
};