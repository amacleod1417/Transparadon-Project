import { Router } from 'express';
const router = Router();

import donationRoutes from './donations';

router.use('/donations', donationRoutes);

router.get('/', (req, res) => {
    res.send('API is working');
});

export default router;