// tests/integration/payout.test.js

const request = require('supertest');
const { v4: uuidv4 } = require('uuid');
const app = require('../../server');            
const { dbConnection, sequelize } = require('../../models/index');

describe('Payouts API Integration Tests', () => {
  let payoutId;
  const userId = uuidv4();

  beforeAll(async () => {
    await dbConnection();
    await sequelize.sync({ force: true });
  });

  afterAll(async () => {
    await sequelize.close();
  });

  test('POST /api/payouts → create a new payout', async () => {
    const res = await request(app)
      .post('/api/payouts')
      .send({ userId, amount: 150.75 });

    expect(res.statusCode).toBe(201);
    expect(res.body.success).toBe(true);
    expect(res.body.data).toHaveProperty('id');
    expect(res.body.data.userId).toBe(userId);
    payoutId = res.body.data.id;
  });

  test('GET /api/payouts/:id → retrieve the payout by ID', async () => {
    const res = await request(app).get(`/api/payouts/${payoutId}`);

    expect(res.statusCode).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.data.id).toBe(payoutId);
  });

  test('GET /api/payouts → list all payouts', async () => {
    const res = await request(app).get('/api/payouts');

    expect(res.statusCode).toBe(200);
    expect(res.body.success).toBe(true);
    expect(Array.isArray(res.body.data)).toBe(true);
    expect(res.body.data.length).toBeGreaterThanOrEqual(1);
  });

  test('PATCH /api/payouts/:id → update payout status & fee', async () => {
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

  test('DELETE /api/payouts/:id → delete the payout', async () => {
    const res = await request(app).delete(`/api/payouts/${payoutId}`);

    expect(res.statusCode).toBe(200);
    expect(res.body.success).toBe(true);
    expect(res.body.message).toBe('Payout deleted');
  });
});
