const request = require('supertest');
const app = require('../../src/app');
const sequelize = require('../../src/config/db.config');

describe('Transaction History API', () => {
  let userId;
  let txId;

  beforeAll(async () => {
    process.env.NODE_ENV = 'test';
    await sequelize.sync({ force: true });

    // Create a test user
    const userRes = await request(app)
      .post('/api/users')
      .send({
        username: 'txhistoryuser',
        email: 'txhistory@example.com',
        wallet_address: '0xabcdefabcdefabcdefabcdefabcdefabcdefabcd',
      });
    userId = userRes.body.data.id;
  });

  afterAll(async () => {
    await sequelize.close();
  });

  test('POST /api/transactions/history → create a transaction record', async () => {
    const res = await request(app)
      .post('/api/transactions/history')
      .send({
        userId,
        type: 'Deposit',
        amount: 200,
        currency: 'USD',
        status: 'Completed',
        fee: 0.5,
        metadata: { source: 'Unit Test' },
      });

    expect(res.status).toBe(201);
    expect(res.body.success).toBe(true);
    expect(res.body.data).toMatchObject({
      userId,
      type: 'Deposit',
      amount: 200,
      currency: 'USD',
      status: 'Completed',
      fee: 0.5,
    });
    // capture internal UUID
    txId = res.body.data.transactionId;
  });

  test('GET /api/transactions/history?userId=… → list transactions', async () => {
    const res = await request(app)
      .get('/api/transactions/history')
      .query({ userId });
    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);
    expect(Array.isArray(res.body.data)).toBe(true);
    expect(res.body.data[0].userId).toBe(userId);
  });

  test('GET /api/transactions/history/:id → fetch by PK', async () => {
    // first get the internal id from the list
    const list = await request(app).get('/api/transactions/history').query({ userId });
    const record = list.body.data.find(r => r.transactionId === txId);
    const res = await request(app).get(`/api/transactions/history/${record.id}`);
    expect(res.status).toBe(200);
    expect(res.body.data.transactionId).toBe(txId);
  });

  test('PATCH /api/transactions/history/:transactionId → update status', async () => {
    const res = await request(app)
      .patch(`/api/transactions/history/${txId}`)
      .send({ status: 'Failed' });

    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.data.status).toBe('Failed');
  });

  test('DELETE /api/transactions/history/:transactionId → delete record', async () => {
    const res = await request(app).delete(`/api/transactions/history/${txId}`);
    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);

    // confirm deletion
    const list = await request(app).get('/api/transactions/history').query({ userId });
    expect(list.body.data.find(r => r.transactionId === txId)).toBeUndefined();
  });
});
