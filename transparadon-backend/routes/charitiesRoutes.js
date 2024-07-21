const express = require('express');
const Charity = require('../models/Charity');
const router = express.Router();

// Add Charity
router.post('/add', async (req, res) => {
    const { name, certificationId } = req.body;
    try {
        const charity = new Charity({ name, certificationId });
        await charity.save();
        res.status(201).send('Charity added');
    } catch (error) {
        res.status(500).send('Error adding charity');
    }
});

// Verify Charity
router.put('/verify/:id', async (req, res) => {
    try {
        const charity = await Charity.findById(req.params.id);
        if (!charity) {
            return res.status(404).send('Charity not found');
        }
        charity.verified = true;
        await charity.save();
        res.status(200).send('Charity verified');
    } catch (error) {
        res.status(500).send('Error verifying charity');
    }
});

// Fetch All Charities
router.get('/', async (req, res) => {
    try {
        const charities = await Charity.find();
        res.json(charities);
    } catch (error) {
        res.status(500).send('Error fetching charities');
    }
});

module.exports = router;


