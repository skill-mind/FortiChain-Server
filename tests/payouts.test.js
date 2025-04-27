const request = require('supertest');
const app = require('../app'); 

let createdPayoutId;

describe('Payout Routes', () => {
  it('should create a new payout', async () => {
    const res = await request(app).post('/api/payouts').send({
      amount: 100,
      status: 'pending',
    });

    expect(res.statusCode).toEqual(201);
    expect(res.body).toHaveProperty('id');
    createdPayoutId = res.body.id; 
  });

  it('should update a payout', async () => {
    const res = await request(app)
      .put(`/api/payouts/${createdPayoutId}`)
      .send({ status: 'completed' });

    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('status', 'completed');
  });

  it('should delete a payout', async () => {
    const res = await request(app).delete(`/api/payouts/${createdPayoutId}`);
    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('message', 'Payout deleted successfully');
  });
});
