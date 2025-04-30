// test/integration/payoutAndTransaction.test.js

const request = require('supertest');
const { v4: uuidv4 } = require('uuid');
const app = require('../../server');              // adjust path if needed
const { dbConnection, sequelize } = require('../../models/index');

describe('Financial API Integration Tests', () => {
  // Connect & sync before any tests run
  beforeAll(async () => {
    await dbConnection();
    // force:true drops & recreates tables so each test run is isolated
    await sequelize.sync({ force: true });
  });

  // Close DB connection when tests complete
  afterAll(async () => {
    await sequelize.close();
  });

  describe('Payouts Endpoints', () => {
    let payoutId;
    const userId = uuidv4();

    it('POST /api/payouts → should create a new payout', async () => {
      const res = await request(app)
        .post('/api/payouts')
        .send({
          userId,
          amount: 150.75,       // required
          // currency, fee, metadata are optional
        });

      expect(res.statusCode).toBe(201);
      expect(res.body.success).toBe(true);
      expect(res.body.data).toHaveProperty('id');
      expect(res.body.data.userId).toBe(userId);
      payoutId = res.body.data.id;
    });

    it('GET /api/payouts/:id → should retrieve the payout by ID', async () => {
      const res = await request(app).get(`/api/payouts/${payoutId}`);

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(res.body.data.id).toBe(payoutId);
    });

    it('GET /api/payouts → should list all payouts', async () => {
      const res = await request(app).get('/api/payouts');

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(Array.isArray(res.body.data)).toBe(true);
      expect(res.body.data.length).toBeGreaterThanOrEqual(1);
    });

    it('PATCH /api/payouts/:id → should update payout status & fee', async () => {
      const res = await request(app)
        .patch(`/api/payouts/${payoutId}`)
        .send({
          status: 'Completed',
          fee: 2.50,
          metadata: { note: 'Test update' },
        });

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(res.body.data.status).toBe('Completed');
      expect(res.body.data.fee).toBeCloseTo(2.50);
      expect(res.body.data.metadata).toEqual({ note: 'Test update' });
    });

    it('DELETE /api/payouts/:id → should delete the payout', async () => {
      const res = await request(app).delete(`/api/payouts/${payoutId}`);

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(res.body.message).toBe('Payout deleted');
    });
  });

  describe('Transaction History Endpoints', () => {
    let txPrimaryId, txTransactionId;
    const userId = uuidv4();

    it('POST /api/transaction-histories → should create a new transaction', async () => {
      const res = await request(app)
        .post('/api/transaction-histories')
        .send({
          userId,
          type: 'Deposit',     // must match ENUM in model
          amount: 50.00,
          status: 'Pending',    // must match ENUM
          // currency, fee, metadata are optional
        });

      expect(res.statusCode).toBe(201);
      expect(res.body.success).toBe(true);
      expect(res.body.data).toHaveProperty('id');
      expect(res.body.data).toHaveProperty('transactionId');
      txPrimaryId = res.body.data.id;
      txTransactionId = res.body.data.transactionId;
    });

    it('GET /api/transaction-histories/:id → should fetch by primary key (id)', async () => {
      const res = await request(app).get(`/api/transaction-histories/${txPrimaryId}`);

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(res.body.data.id).toBe(txPrimaryId);
    });

    it('GET /api/transaction-histories → should list all transactions', async () => {
      const res = await request(app).get('/api/transaction-histories');

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(Array.isArray(res.body.data)).toBe(true);
      expect(res.body.data.length).toBeGreaterThanOrEqual(1);
    });

    it('PATCH /api/transaction-histories/:transactionId → should update status & metadata', async () => {
      const res = await request(app)
        .patch(`/api/transaction-histories/${txTransactionId}`)
        .send({
          status: 'Completed',
          metadata: { processed: true },
        });

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(res.body.data.status).toBe('Completed');
      expect(res.body.data.metadata).toEqual({ processed: true });
    });

    it('DELETE /api/transaction-histories/:transactionId → should delete the transaction', async () => {
      const res = await request(app).delete(
        `/api/transaction-histories/${txTransactionId}`
      );

      expect(res.statusCode).toBe(200);
      expect(res.body.success).toBe(true);
      expect(res.body.message).toBe('Transaction deleted');
    });
  });
});
