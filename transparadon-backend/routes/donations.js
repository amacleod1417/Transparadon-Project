const express = require('express');
const router = express.Router();
const { donate } = require('../controllers/donationsController.js');

router.post('/donate', donate);

module.exports = router;