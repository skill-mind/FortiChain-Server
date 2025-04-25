// Set test environment
process.env.NODE_ENV = 'test';

const request = require('supertest');
const express = require('express');
const bodyParser = require('body-parser');
const Wallet = require('../../models/wallet.model');
const walletRoutes = require('../../routes/wallet.routes');
const sequelize = require('../../config/db.config');

// Create a test Express app
const app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));
app.use('/api/wallets', walletRoutes);

describe('Wallet Controller Integration Tests', () => {
  // Test data for different wallet types
  const ethereumWallet = {
    address: '0xabcdef1234567890abcdef1234567890abcdef12',
    role: 'researcher'
  };

  const starknetWallet = {
    address: '0x057d35a858fc7a5238b9339d640648bb2363cddd729deb357d035d6f27c2d476',
    role: 'project_owner'
  };

  const stellarWallet = {
    address: 'GAKONCKYJ7PRRKBZSWVPG3MURUNX4TLNWCAVTPXXGQDKB6BZD5RFA6AN',
    role: 'validator'
  };

  const updatedWalletData = {
    address: '0xabcdef1234567890abcdef1234567890abcdef12',
    role: 'validator'
  };

  let walletId;

  // Setup before tests run
  beforeAll(async () => {
    // Clear test database and sync models
    await sequelize.sync({ force: true });
  });

  // Clean up after tests
  afterAll(async () => {
    await sequelize.close();
  });

  // Test suite for POST /api/wallets with different address formats
  describe('POST /api/wallets with different address formats', () => {
    beforeEach(async () => {
      // Clear wallets before each test
      await Wallet.destroy({ where: {}, force: true });
    });

    it('should create a wallet with Ethereum address format', async () => {
      const response = await request(app)
        .post('/api/wallets')
        .send(ethereumWallet);

      expect(response.status).toBe(201);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toHaveProperty('address', ethereumWallet.address);
    });

    it('should create a wallet with Starknet address format', async () => {
      const response = await request(app)
        .post('/api/wallets')
        .send(starknetWallet);

      expect(response.status).toBe(201);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toHaveProperty('address', starknetWallet.address);
    });

    it('should create a wallet with Stellar address format', async () => {
      const response = await request(app)
        .post('/api/wallets')
        .send(stellarWallet);

      expect(response.status).toBe(201);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toHaveProperty('address', stellarWallet.address);
    });

    it('should reject an invalid wallet address format', async () => {
      const response = await request(app)
        .post('/api/wallets')
        .send({
          address: 'invalid-address-format',
          role: 'researcher'
        });

      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('success', false);
    });
  });

  // The rest of your existing tests
  // Test suite for GET /api/wallets (getAllWallets)
  describe('GET /api/wallets', () => {
    beforeEach(async () => {
      // Clear and create test wallets
      await Wallet.destroy({ where: {}, force: true });
      await Wallet.create(ethereumWallet);
      await Wallet.create(starknetWallet);
      await Wallet.create(stellarWallet);
    });

    it('should get all wallets successfully', async () => {
      const response = await request(app)
        .get('/api/wallets')
        .send();

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toBeInstanceOf(Array);
      expect(response.body.data.length).toBe(3);
      expect(response.body.data[0]).toHaveProperty('address');
      expect(response.body.data[1]).toHaveProperty('address');
      expect(response.body.data[2]).toHaveProperty('address');
    });
  });

  // Test suite for GET /api/wallets/id/:id (getWalletById)
  describe('GET /api/wallets/id/:id', () => {
    beforeEach(async () => {
      // Clear and create test wallet
      await Wallet.destroy({ where: {}, force: true });
      const wallet = await Wallet.create(ethereumWallet);
      walletId = wallet.id;
    });

    it('should get wallet by ID successfully', async () => {
      const response = await request(app)
        .get(`/api/wallets/id/${walletId}`)
        .send();

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toHaveProperty('id', walletId);
      expect(response.body.data).toHaveProperty('address', ethereumWallet.address);
    });

    it('should return 404 if wallet ID does not exist', async () => {
      const nonExistentId = '11111111-1111-1111-1111-111111111111';

      const response = await request(app)
        .get(`/api/wallets/id/${nonExistentId}`)
        .send();

      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('success', false);
      expect(response.body).toHaveProperty('message', 'Wallet not found');
    });
  });

  // Test suite for GET /api/wallets/address/:address (getWalletByAddress)
  describe('GET /api/wallets/address/:address', () => {
    beforeEach(async () => {
      // Clear and create test wallet
      await Wallet.destroy({ where: {}, force: true });
      await Wallet.create(ethereumWallet);
    });

    it('should get wallet by address successfully', async () => {
      const response = await request(app)
        .get(`/api/wallets/address/${ethereumWallet.address}`)
        .send();

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toHaveProperty('address', ethereumWallet.address);
    });

    it('should return 404 if wallet address does not exist', async () => {
      const nonExistentAddress = '0x1111111111111111111111111111111111111111';

      const response = await request(app)
        .get(`/api/wallets/address/${nonExistentAddress}`)
        .send();

      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('success', false);
      expect(response.body).toHaveProperty('message', 'Wallet not found');
    });
  });

  // Test suite for PUT /api/wallets/:id (updateWallet)
  describe('PUT /api/wallets/:id', () => {
    beforeEach(async () => {
      // Clear and create test wallet
      await Wallet.destroy({ where: {}, force: true });
      const wallet = await Wallet.create(ethereumWallet);
      walletId = wallet.id;
    });

    it('should update wallet successfully', async () => {
      const response = await request(app)
        .put(`/api/wallets/${walletId}`)
        .send(updatedWalletData);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body.data).toHaveProperty('role', updatedWalletData.role);

      // Verify wallet is actually updated in database
      const updatedWallet = await Wallet.findByPk(walletId);
      expect(updatedWallet.role).toBe(updatedWalletData.role);
    });

    it('should return 404 if wallet does not exist', async () => {
      const nonExistentId = '11111111-1111-1111-1111-111111111111';

      const response = await request(app)
        .put(`/api/wallets/${nonExistentId}`)
        .send(updatedWalletData);

      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('success', false);
      expect(response.body).toHaveProperty('message', 'Wallet not found');
    });

    it('should return 400 if role is invalid', async () => {
      const response = await request(app)
        .put(`/api/wallets/${walletId}`)
        .send({
          address: ethereumWallet.address,
          role: 'invalid_role'
        });

      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('success', false);
    });
  });

  // Test suite for DELETE /api/wallets/:id (deleteWallet)
  describe('DELETE /api/wallets/:id', () => {
    beforeEach(async () => {
      // Create a test wallet in the database
      await Wallet.destroy({ where: {}, force: true });
      const wallet = await Wallet.create(ethereumWallet);
      walletId = wallet.id;
    });

    it('should delete a wallet successfully', async () => {
      const response = await request(app)
        .delete(`/api/wallets/${walletId}`)
        .send();

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('success', true);
      expect(response.body).toHaveProperty('message', 'Wallet deleted successfully');

      // Verify wallet is actually deleted from database
      const deletedWallet = await Wallet.findByPk(walletId);
      expect(deletedWallet).toBeNull();
    });

    it('should return 404 if wallet does not exist', async () => {
      const nonExistentId = '11111111-1111-1111-1111-111111111111';

      const response = await request(app)
        .delete(`/api/wallets/${nonExistentId}`)
        .send();

      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('success', false);
      expect(response.body).toHaveProperty('message', 'Wallet not found');
    });

    it('should handle server errors properly', async () => {
      // Mock a server error by temporarily replacing the findByPk method
      const originalFindByPk = Wallet.findByPk;
      Wallet.findByPk = jest.fn().mockImplementation(() => {
        throw new Error('Database error');
      });

      // Send request that will generate error
      const response = await request(app)
        .delete(`/api/wallets/${walletId}`)
        .send();

      // Assert the error response
      expect(response.status).toBe(500);
      expect(response.body).toHaveProperty('success', false);
      expect(response.body).toHaveProperty('message', 'Internal server error');
      expect(response.body).toHaveProperty('error');

      // Restore the original method
      Wallet.findByPk = originalFindByPk;
    });
  });
});
