const mongoose = require('mongoose');

const CharitySchema = new mongoose.Schema({
    name: {
        type: String,
        required: true,
        unique: true,
    },
    certificationId: {
        type: String,
        required: true,
        unique: true,
    },
    verified: {
        type: Boolean,
        default: false,
    },
});

const Charity = mongoose.model('Charity', CharitySchema);
module.exports = Charity;
