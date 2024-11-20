# Decentralized Land Registry Platform

The Decentralized Land Registry Platform is a blockchain-based system that allows users to securely register, verify, and transfer land ownership. Built on the Internet Computer (ICP), the platform offers a transparent and tamper-proof mechanism for land management, reducing the risk of fraud and providing a reliable system for property transactions. The project comprises two canisters: one for the frontend interface and another for backend logic.

## Features

- **Land Registration**: Users can register land parcels, including property details such as location, size, and ownership information.
- **Ownership Transfer**: Registered property owners can securely transfer ownership to another user.
- **Verification**: Ensure the authenticity of land titles through blockchain technology.
- **Candid Interface for Backend**: Interact with the backend system using the Candid interface for administrative or complex tasks.
- **Decentralization**: The entire system operates on a decentralized platform, ensuring transparency and reducing the chances of corruption or tampering.

## Project Structure

The project consists of two main components:
1. **Frontend**: The user interface where land registration, ownership transfers, and verification actions are performed.
2. **Backend**: The smart contract logic that manages the land records and handles all operations related to land registry functions.

## Deployed Links

- **Frontend Canister (Browser)**:  
  [Land System Frontend](https://dydyq-6aaaa-aaaag-amkha-cai.icp0.io/)

- **Backend Canister (Candid Interface)**:  
  [Land System Backend](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=dratm-iiaaa-aaaag-amkgq-cai)

## Installation & Setup

### Prerequisites

- Install DFX (Dfinity SDK) on your local machine to interact with ICP canisters.
- Node.js for managing dependencies.
- Vite for frontend development.

### Steps

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/victordev123/Decentralized-Land-Registry-Management-System
   cd Decentralized-Land-Registry-Management-System
   ```

2. **Install Dependencies**:
   ```bash
   npm install
   ```

3. **Deploy Canisters**:
   You can deploy the frontend and backend canisters on the Internet Computer using DFX commands. Ensure you have the correct permissions and network access to deploy canisters.

4. **Run Frontend Locally**:
   ```bash
   npm run dev
   ```
   This will start the frontend on a local server, allowing you to interact with the system for testing.

5. **Access Backend**:  
   You can access the backend through the Candid interface provided above. This interface allows interaction with the deployed canister and its functions directly.

### Canister Interaction

#### Register Land
To register land, users input property details such as location, size, and ownership. The frontend sends this information to the backend canister, which stores it securely on the blockchain.

#### Transfer Ownership
The current owner can initiate a transfer of ownership through the platform by selecting the land parcel and the recipient's details. The backend verifies the transaction and updates the ownership on-chain.

#### Verify Land Title
Any user can verify the authenticity of a land title by entering the property details. The backend checks the blockchain records and returns a verification status.

## Technologies Used

- **DFX**: For deploying and interacting with ICP canisters.
- **Vite**: A fast development build tool for the frontend.
- **Candid Interface**: For backend interactions and direct smart contract calls.
- **ICP Blockchain**: Ensures decentralized and secure land registration and ownership management.

## Contributing

Contributions are welcome! Feel free to fork this project, open issues, and submit pull requests.