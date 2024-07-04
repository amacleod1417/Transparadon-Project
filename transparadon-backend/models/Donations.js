const mongoose = require('mongoose');
const Schema = mongoose.Schema;

const donationSchema = new Schema({
    amount: {
        type: Number,
        required: true
    },
    user: {
        type: String, 
        required: true
    },
    date: {
        type: Date,
        default: Date.now
    }
});

module.exports = mongoose.model('Donation', donationSchema);