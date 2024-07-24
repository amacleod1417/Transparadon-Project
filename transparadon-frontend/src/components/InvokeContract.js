import React, { useState } from 'react';
import axios from 'axios';

const InvokeContract = () => {
  const [contractId, setContractId] = useState('');
  const [functionName, setFunctionName] = useState('');
  const [args, setArgs] = useState('');

  const handleInvokeContract = async () => {
    try {
      const response = await axios.post('/api/soroban/invoke', {
        contractId,
        functionName,
        args: JSON.parse(args),
      });
      alert('Contract invoked successfully: ' + JSON.stringify(response.data));
    } catch (error) {
      alert(error.toString());
    }
  };

  return (
    <div>
      <h2>Invoke Contract</h2>
      <input
        type="text"
        value={contractId}
        onChange={(e) => setContractId(e.target.value)}
        placeholder="Contract ID"
      />
      <input
        type="text"
        value={functionName}
        onChange={(e) => setFunctionName(e.target.value)}
        placeholder="Function Name"
      />
      <input
        type="text"
        value={args}
        onChange={(e) => setArgs(e.target.value)}
        placeholder="Arguments (JSON format)"
      />
      <button onClick={handleInvokeContract}>Invoke Contract</button>
    </div>
  );
};

export default InvokeContract;
