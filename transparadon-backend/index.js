import express, { json } from 'express';
import { connect } from 'mongoose';
import { config } from 'dotenv';
import routes from './routes/index';

config();

const app = express();
app.use(json());
app.use('/api', routes);

const PORT = process.env.PORT || 3000;
const MONGO_URI = process.env.MONGO_URI;

connect(MONGODB_URI, { useNewUrlParser: true, useUnifiedTopology: true }).then(() => console.log('MongoDB connected')).catch(err => console.log(err));

app.get('/', (req, res)=> {
    res.send('Welcome to Transparadon Backend');
});

app.listen(PORT, () => {
    console.log('Server running on port ${PORT}');
});