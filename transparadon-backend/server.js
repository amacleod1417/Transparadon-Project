const express = require('express');
const connectDB = require('./config/db');

const app = express();

connectDB();

//Init Middleware
app.use(express.json({ extended: false }));
app.use('/api/users', require('./routes/authRoutes'));
app.use('/api/stellar', require('./routes/stellarRoutes')); // add stellar routes

// Start the server
const PORT = process.env.PORT || 5000;
app.listen(PORT, () => console.log(`Server started on port ${PORT}`));
