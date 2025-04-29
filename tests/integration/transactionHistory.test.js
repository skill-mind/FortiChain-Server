// tests/integration/transactionHistory.test.js

const request = require('supertest');
const { v4: uuidv4 } = require('uuid');
const app = require('../../server');
const { dbConnection, sequelize } = require('../../models/index');

describe('Transaction History API Integration Tests', () => {
  let txPrimaryId, txTransactionId;
  const userId = uuidv4();

  beforeAll(async () => {
    await dbConnection();
    await sequelize.sync({ force: true });
  });

  afterAll(async () => {
    await sequelize.close();
  });

  test('POST /api/transaction-histories → create a new transaction', async () => {
    const res = await request(app)
      .post('/api/transaction-histories')
      .send({
        userId,
        type: 'Deposit',
        amount: 50.00,
        status: 'Pending',
      });

    expect(res.statusCode).toBe(201);
    expect(res.body.success).toBe(true);
    expect(res.body.data).toHaveProperty('id');
    expect(res.body.data).toHaveProperty('transactionId');
    txPrimaryId = res.body.data.id;
    txTransactionId = res.body.data.transactionId;
  });

  test('GET /api/transaction-histories/:id → fetch by primary key', async () => {
    const res = await request(app).get(`/api/transaction-histories/${txPrimaryId}`);

    expect(res.statusCode).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.data.id).toBe(txPrimaryId);
  });

  test('GET /api/transaction-histories → list all transactions', async () => {
    const res = await request(app).get('/api/transaction-histories');

    expect(res.statusCode).toBe(200);
    expect(res.body.success).toBe(true);
    expect(Array.isArray(res.body.data)).toBe(true);
    expect(res.body.data.length).toBeGreaterThanOrEqual(1);
  });

  test('PATCH /api/transaction-histories/:transactionId → update status & metadata', async () => {
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

  test('DELETE /api/transaction-histories/:transactionId → delete the transaction', async () => {
    const res = await request(app).delete(
      `/api/transaction-histories/${txTransactionId}`
    );

    expect(res.statusCode).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.message).toBe('Transaction deleted');
  });
});
