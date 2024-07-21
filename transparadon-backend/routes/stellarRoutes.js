const express = require('express');
const { createAccount, makePayment } = require('../services/stellarService');

const router = express.Router();

// Create a new account
router.post('/create-account', async (req, res) => {
  const { destination } = req.body;
  try {
    await createAccount(destination);
    res.status(200).send('Account created successfully');
  } catch (error) {
    res.status(500).send(error.toString());
  }
});

// Make a payment
router.post('/payment', async (req, res) => {
  const { destination, amount } = req.body;
  try {
    await makePayment(destination, amount);
    res.status(200).send('Payment made successfully');
  } catch (error) {
    res.status(500).send(error.toString());
  }
});

module.exports = router;
