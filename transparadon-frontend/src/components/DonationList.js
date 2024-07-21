// src/components/DonationList.js
import React, { useEffect, useState } from 'react';
import { getDonations } from '../services/api';

const DonationList = () => {
  const [donations, setDonations] = useState([]);

  useEffect(() => {
    const fetchDonations = async () => {
      const donations = await getDonations();
      setDonations(donations);
    };

    fetchDonations();
  }, []);

  return (
    <div>
      <h2>My Donations</h2>
      <ul>
        {donations.map(donation => (
          <li key={donation.id}>{donation.charity}: {donation.amount}</li>
        ))}
      </ul>
    </div>
  );
};

export default DonationList;
