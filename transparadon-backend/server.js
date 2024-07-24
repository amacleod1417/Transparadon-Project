const express = require('express');
const mongoose = require('mongoose');
const bodyParser = require('body-parser');
const cors = require('cors');
const userRoutes = require('./routes/users');
const charityRoutes = require('./routes/charities'); // Add this line
const { createAccount, makePayment } = require('./services/stellarService');

const app = express();
const port = process.env.PORT || 5000;

app.use(bodyParser.json());
app.use(cors());
app.use('/api/soroban', require('./routes/sorobanRoutes'));
app.use('/api/users', userRoutes);
app.use('/api/charities', charityRoutes); 

app.post('/api/stellar/createAccount', async (req, res) => {
    const { destination } = req.body;
    try {
        await createAccount(destination);
        res.status(200).send('Account created');
    } catch (error) {
        res.status(500).send('Error creating account');
    }
});

app.post('/api/stellar/makePayment', async (req, res) => {
    const { destination, amount } = req.body;
    try {
        await makePayment(destination, amount);
        res.status(200).send('Payment made');
    } catch (error) {
        res.status(500).send('Error making payment');
    }
});

mongoose.connect('mongodb://localhost:27017/transparadon', {
    useNewUrlParser: true,
    useUnifiedTopology: true,
}).then(() => console.log('MongoDB connected'))
    .catch(err => console.log(err));

app.listen(port, () => console.log(`Server running on port ${port}`));
