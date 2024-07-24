const express = require('express');
const { invokeContract } = require('../services/sorobanService');

const router = express.Router();

// Invoke smart contract
router.post('/invoke', async (req, res) => {
  const { contractId, functionName, args } = req.body;
  try {
    const result = await invokeContract(contractId, functionName, args);
    res.status(200).send(result);
  } catch (error) {
    res.status(500).send(error.toString());
  }
});

module.exports = router;
