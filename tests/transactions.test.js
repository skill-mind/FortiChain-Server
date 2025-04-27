const request = require('supertest');
const app = require('../app'); 

let createdTransactionId;

describe('Transaction Routes', () => {
  it('should create a new transaction', async () => {
    const res = await request(app).post('/api/transactions').send({
      amount: 500,
      type: 'deposit',
    
    });

    expect(res.statusCode).toEqual(201);
    expect(res.body).toHaveProperty('id');
    createdTransactionId = res.body.id;
  });
  
  it('should get all transactions', async () => {
    const res = await request(app).get('/api/transactions');
    expect(res.statusCode).toEqual(200);
    expect(Array.isArray(res.body.data)).toBe(true);
  });

  it('should get a single transaction', async () => {
    const res = await request(app).get(`/api/transactions/${createdTransactionId}`);
    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('id', createdTransactionId);
  });

  it('should update a transaction', async () => {
    const res = await request(app)
      .put(`/api/transactions/${createdTransactionId}`)
      .send({ amount: 600 });

    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('amount', 600);
  });

  
  it('should delete a transaction', async () => {
    const res = await request(app).delete(`/api/transactions/${createdTransactionId}`);
    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('message', 'Transaction deleted successfully');
  });
});
