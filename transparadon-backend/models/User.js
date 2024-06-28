//user schema

const mongoose = require('mongoose');

const UserSchema = new mongoose.Schema({
    address: {type: String, required: true },
    votingPower: { type: Number, default: 0 }
});

module.exports = mongoose.model('User', UserSchema);