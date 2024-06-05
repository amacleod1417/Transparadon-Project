
Transparadon is a decentralized charity donation platform designed to bring transparency, accountability, and efficiency to the world of charitable giving. Built on the Soroban smart contract platform on the Stellar blockchain, Transparadon ensures that all donations are tracked and verifiable, providing donors with confidence that their contributions are making a real impact.

![TransparaDon](https://github.com/onkr0d/transparadon/assets/155204136/2aed2768-4064-486b-a42e-f1a7ee2cbc5a)

Features
Decentralized Donations: Enable secure and transparent donation transactions using blockchain technology.
Smart Contracts: Automate the distribution of funds to ensure they reach the intended recipients.
Real-Time Tracking: Allow donors to track their donations in real-time.
Low Transaction Fees: Minimize costs associated with traditional payment methods.
Transparency Reports: Generate detailed reports on the use and impact of donations.
Anonymity Options: Allow donors to choose between anonymous and public donations.
Global Accessibility: Support donations from anywhere in the world.
Community Pool Donations: Donate to a community pool where quadratic voting determines the allocation of funds.

Technology Stack
Frontend: React.js
Backend: Node.js, Express.js
Blockchain: Stellar, Soroban for smart contracts
Database: MongoDB
Authentication: JWT, Stellar wallet integration
APIs: Stellar SDK, Soroban SDK for blockchain interactions

Installation
To set up Transparadon locally, follow these steps:

Clone the repository:

sh
Copy code
git clone https://github.com/yourusername/transparadon.git
cd transparadon
Install dependencies:

sh
Copy code
npm install
Set up environment variables:
Create a .env file in the root directory and add the following:

makefile
Copy code
NODE_ENV=development
PORT=3000
MONGODB_URI=your_mongodb_connection_string
JWT_SECRET=your_jwt_secret
STELLAR_NETWORK=testnet (or public)
Run the development server:

sh
Copy code
npm run dev
Deploy the smart contracts:
Follow the Soroban CLI instructions to deploy your smart contracts to the Stellar testnet or public network.

Usage
Once the platform is set up, you can start using Transparadon to make and manage donations.

Register/Login:

Use a Stellar wallet to connect your account.
Register an account or log in using your wallet.
Make a Donation:

Browse the list of verified charities.
Select a charity and enter the donation amount.
Confirm the transaction using your Stellar wallet.
Track Donations:

Visit the 'My Donations' section to see the status and impact of your donations.
Access detailed reports for transparency.

License
Transparadon is licensed under the MIT License. See the LICENSE file for more information.

Thank you for contributing to a more transparent and effective charitable giving ecosystem with Transparadon!
