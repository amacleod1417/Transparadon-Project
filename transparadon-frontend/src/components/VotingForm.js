import React, { useState } from 'react';

function DonationForm() {
  const [amount, setAmount] = useState('');
  const [user, setUser] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();
    // Call the donate function from the backend
    console.log(`Donating ${amount} from ${user}`);
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>
        Donation Amount:
        <input type="number" value={amount} onChange={(e) => setAmount(e.target.value)} />
      </label>
      <label>
        User Address:
        <input type="text" value={user} onChange={(e) => setUser(e.target.value)} />
      </label>
      <button type="submit">Donate</button>
    </form>
  );
}

export default DonationForm;
