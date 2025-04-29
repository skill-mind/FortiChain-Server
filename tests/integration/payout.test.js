const request = require('supertest');
const app = require('../../src/app');
const sequelize = require('../../src/config/db.config');

describe('Payout API', () => {
  let userId;
  let payoutId;

  beforeAll(async () => {
    process.env.NODE_ENV = 'test';
    await sequelize.sync({ force: true });

    // Create a test user
    const userRes = await request(app)
      .post('/api/users')
      .send({
        username: 'payoutuser',
        email: 'payout@example.com',
        wallet_address: '0x1234567890abcdef1234567890abcdef12345678',
      });
    userId = userRes.body.data.id;
  });

  afterAll(async () => {
    await sequelize.close();
  });

  test('POST /api/payouts → create a payout', async () => {
    const res = await request(app)
      .post('/api/payouts')
      .send({
        userId,
        amount: 150.5,
        currency: 'USD',
        fee: 1.5,
        metadata: { reason: 'Test payout' },
      });

    expect(res.status).toBe(201);
    expect(res.body.success).toBe(true);
    expect(res.body.data).toMatchObject({
      userId,
      amount: 150.5,
      currency: 'USD',
      fee: 1.5,
      status: 'Pending',
    });
    payoutId = res.body.data.id;
  });

  test('GET /api/payouts/:id → fetch by ID', async () => {
    const res = await request(app).get(`/api/payouts/${payoutId}`);
    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.data.id).toBe(payoutId);
  });

  test('GET /api/payouts → list all (filter by userId)', async () => {
    const res = await request(app).get('/api/payouts').query({ userId });
    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);
    expect(Array.isArray(res.body.data)).toBe(true);
    expect(res.body.data[0].userId).toBe(userId);
  });

  test('PATCH /api/payouts/:id → update status & processedAt', async () => {
    const now = new Date().toISOString();
    const res = await request(app)
      .patch(`/api/payouts/${payoutId}`)
      .send({ status: 'Completed', processedAt: now });

    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.data.status).toBe('Completed');
    // processedAt comes back as a string
    expect(new Date(res.body.data.processedAt).toISOString()).toBe(now);
  });

  test('DELETE /api/payouts/:id → delete payout', async () => {
    const res = await request(app).delete(`/api/payouts/${payoutId}`);
    expect(res.status).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.message).toMatch(/deleted/i);

    // confirm it's gone
    const res2 = await request(app).get(`/api/payouts/${payoutId}`);
    expect(res2.status).toBe(404);
  });
});
