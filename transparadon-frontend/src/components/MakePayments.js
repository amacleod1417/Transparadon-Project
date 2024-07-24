import React, { useState } from 'react';
import { makePayment } from '../services/stellarService';

const MakePayment = () => {
  const [destination, setDestination] = useState('');
  const [amount, setAmount] = useState('');

  const handleMakePayment = async () => {
    try {
      await makePayment(destination, amount);
      alert('Payment made successfully');
    } catch (error) {
      alert(error.toString());
    }
  };

  return (
    <div>
      <h2>Make Payment</h2>
      <input
        type="text"
        value={destination}
        onChange={(e) => setDestination(e.target.value)}
        placeholder="Destination address"
      />
      <input
        type="text"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
        placeholder="Amount"
      />
      <button onClick={handleMakePayment}>Make Payment</button>
    </div>
  );
};

export default MakePayment;
