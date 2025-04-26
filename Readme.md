# Contributing to Fortichain: Enhancing the security of blockchain-based projects.

Thank you for your interest in contributing to Fortichain!

FortiChain is a decentralized platform designed to enhance the security of blockchain-based projects by streamlining vulnerability disclosure and bug bounty processes. By leveraging blockchain's inherent transparency and trustlessness, FortiChain allows developers, security researchers, and validators to engage in a secure and automated ecosystem for identifying and rewarding the resolution of smart contract vulnerabilities.

## Project Overview

**FortiChain** Enhancing the security of blockchain-based projects:

- **Smart Contract Developers/Project Owners**: Looking for an unbiased mechanism to audit and secure their code.
- **Security Researchers/Bug Hunters**: Seeking transparent and reliable platforms to report vulnerabilities and earn rewards.
- **Community Validators/Reviewers**: Interested in contributing to security oversight and earning reputation or token-based incentives.
- **Blockchain Ecosystem Participants**: Including organizations and individuals who value transparency and security in decentralized applications.

## Key Features & Functional Requirements

### 1 Project Registration & Management

**Feature:** On-chain registration for smart contracts.

**Requirements:**
- Ability for project owners to submit smart contract details (e.g., code repository links, deployment addresses).
- Versioning and audit trail for any changes to the registered contract.
- Verification process to validate project ownership (e.g., cryptographic signatures).

---

### 2 Vulnerability Submission & Reporting

**Feature:** Secure submission of vulnerability reports.

**Requirements:**
- Interface for security researchers to confidentially submit vulnerability details.
- Option to attach proof-of-concept files, screenshots, or documentation.
- Timestamping and immutable record creation on-chain.
- Option for researchers to choose public disclosure or private reporting (with potential delayed public reveal upon resolution).

---

### 3 Decentralized Review & Verification

**Feature:** Community-driven validation of submitted vulnerabilities.

**Requirements:**
- Voting mechanism where pre-approved validators/reviewers can assess the validity of a vulnerability.
- Reputation system to score reviewers based on historical accuracy and participation.
- Smart contract–based arbitration that triggers once consensus is reached.
- Incentives (tokens) for accurate reviews and penalties for fraudulent claims.

---

### 4 Automated Bounty Management

**Feature:** Smart contract automation for bounty escrow and payout.

**Requirements:**
- Escrow system where project owners deposit bounty funds in tokens.
- Smart contract logic to release funds automatically upon successful vulnerability validation.
- Provisions for partial payouts in cases of multi-stage vulnerabilities.
- Audit trail for every transaction ensuring transparency.

---

### 5 Token & Reputation Incentives

**Feature:** Native token integration for rewards and reputation tracking.

**Requirements:**
- Creation of a utility token used for bounties, voting rewards, and reputation stakes.
- Smart contracts to manage token issuance, distribution, and burning.
- A reputation system that influences voting power and eligibility for higher-value bounties.
- Transparent metrics for performance and participation.

---

## Validator Ranking Endpoints

### Create Validator Ranking
- **POST** `/api/validator-rankings`
- **Body:** `{ validatorId, rank, score, notes }`
- **Response:** Created ranking object

### Get All Validator Rankings
- **GET** `/api/validator-rankings?page=1&limit=20`
- **Response:** `{ count, rows: [ ...rankingObjects ] }`

### Get Validator Ranking by ID
- **GET** `/api/validator-rankings/:id`
- **Response:** Ranking object

### Update Validator Ranking
- **PUT** `/api/validator-rankings/:id`
- **Body:** `{ rank, score, notes }`
- **Response:** Updated ranking object

### Delete Validator Ranking
- **DELETE** `/api/validator-rankings/:id`
- **Response:** `{ message: 'Ranking deleted successfully.' }`

## How to Contribute

We welcome contributions in various forms, including bug fixes, feature implementations, and documentation improvements.

### 1. Fork the Repository
1. Navigate to the
(https://github.com/skill-mind/FortiChain-Server.git).
2. Click the **Fork** button to create your copy of the repository.

### 2. Clone the Repository
- Clone your forked repository to your local machine:
```bash
git clone https://github.com/<your-username>/skill-mind/FortiChain-Server.git

cd FortiChain-Server
```

### 3. Set Up the Project

#### Prerequisites:

- Node.js (v16.x or above)
- npm
- git
- Docker and Docker Compose (for local development)

#### Install Dependencies:

```bash
npm install
```

#### Configure Environment Variables:
Create a `.env` file in the root directory with the following content:

```
PORT=3000
NODE_ENV=development
LOG_LEVEL=info
```

### 4. Database Setup Using Docker

The project uses PostgreSQL running in Docker. To set up the database:

```bash
# Start the PostgreSQL container
docker compose up -d
```

This will:
- Create a PostgreSQL instance on port 5432
- Setup the database with the credentials specified in the docker-compose.yml file
- Create a persistent volume for data storage

### 5. Running the Application

```bash
# Start the server in development mode
npm run dev

# Start the server in production mode
npm start
```

The API will be available at `http://localhost:3000`. You can use the test-api.http file to test the existing endpoints.

### 6. Logging

The application uses Winston for logging:
- Logs are stored in the `logs` directory
- Error logs are in `logs/error.log`
- All logs are in `logs/combined.log`
- Console logs are also available during development

You can adjust the log level in the `.env` file by changing the `LOG_LEVEL` value.

### 7. Testing

The project uses Jest for testing.

#### Running Tests

```bash
# Run all tests
npm test

# Run tests with coverage report
npm run test:coverage

# Run tests in watch mode during development
npm run test:watch
```

#### Test Structure

- Integration tests are located in `tests/integration/` directory
- Unit tests are located in `tests/unit/` directory (if applicable)

#### Writing Tests

When contributing new features, please add corresponding tests:

1. For API endpoints, add integration tests that verify:
   - Successful operations
   - Error handling
   - Edge cases
   - Input validation

2. For utility functions or models, add unit tests that verify:
   - Function behavior with various inputs
   - Edge cases
   - Error handling

Example test structure (from wallet tests):
```javascript
describe('Feature or Component Name', () => {
  // Setup (if needed)
  beforeEach(async () => {
    // Setup code
  });

  // Individual test cases
  it('should do something specific', async () => {
    // Test code
    expect(result).toBe(expectedValue);
  });
});
```

### 8. Create a New Branch

**Create a branch for your feature or bug fix:**
```bash
  git checkout -b feature/<Issue title>
```

### 9. Make Changes and Commit

- Implement your changes.
- Test your changes thoroughly.
- Commit your work with a descriptive message:

```bash
   git add .
   git commit -m "Issue Title"
```

### 10. Push Changes
 - Push your branch to your forked repository:

```bash
   git push origin <Issue Title>
```

### 11. Create a Pull Request (PR)

- Click on Pull Requests and select New Pull Request.
- Provide a clear and concise title and description for your PR.
- Link any relevant issues.

**Code of Conduct**

- Please adhere to our Code of Conduct to maintain a respectful and inclusive community.

### Contribution Guidelines
- Write clean and modular code following the repository's coding standards.
- Ensure all changes are tested before submission.
- Document new features and updates thoroughly.
