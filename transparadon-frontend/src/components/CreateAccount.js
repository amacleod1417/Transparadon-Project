import React, { useState } from 'react';
import { createAccount } from '../services/stellarService';

const CreateAccount = () => {
  const [destination, setDestination] = useState('');

  const handleCreateAccount = async () => {
    try {
      await createAccount(destination);
      alert('Account created successfully');
    } catch (error) {
      alert(error.toString());
    }
  };

  return (
    <div>
      <h2>Create Account</h2>
      <input
        type="text"
        value={destination}
        onChange={(e) => setDestination(e.target.value)}
        placeholder="Destination address"
      />
      <button onClick={handleCreateAccount}>Create Account</button>
    </div>
  );
};

export default CreateAccount;
