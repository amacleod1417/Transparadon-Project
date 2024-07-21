
import React, { useState } from 'react';
import { makeDonation } from '../services/api';

const DonationForm = () => {
  const [amount, setAmount] = useState('');
  const [charity, setCharity] = useState('');

  const handleDonate = async () => {
    await makeDonation(charity, amount);
  };

  return (
    <div>
      <h2>Make a Donation</h2>
      <input type="text" value={charity} onChange={(e) => setCharity(e.target.value)} placeholder="Charity" />
      <input type="number" value={amount} onChange={(e) => setAmount(e.target.value)} placeholder="Amount" />
      <button onClick={handleDonate}>Donate</button>
    </div>
  );
};

export default DonationForm;
