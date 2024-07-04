const express = require('express');
const router = express.Router();
const { donate } = require('../controllers/donationsController.js');

router.post('/donate', donate);
router.post('/vote', donationsController.vote);


module.exports = router;