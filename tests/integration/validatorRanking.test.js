const db = require('../../models');
const request = require('supertest');
const app = require('../../server');
const sequelize = db.sequelize;
const ValidatorRanking = db.ValidatorRanking;
const User = db.User;

describe('Validator Ranking Endpoints', () => {
  let validatorId;
  let rankingId;

  beforeAll(async () => {
    await db.dbConnection();
    await sequelize.sync({ force: true });
    const user = await User.create({ walletAddress: '0xTestValidator1234567890' });
    validatorId = user.id;
  });

  afterAll(async () => {
    await sequelize.close();
  });

  it('should create a validator ranking', async () => {
    const res = await request(app)
      .post('/api/validator-rankings')
      .send({ validatorId, rank: 1, score: 99.5, notes: 'Top validator' });
    expect(res.statusCode).toBe(201);
    expect(res.body).toHaveProperty('id');
    expect(res.body.validatorId).toBe(validatorId);
    rankingId = res.body.id;
  });

  it('should fetch all validator rankings', async () => {
    const res = await request(app).get('/api/validator-rankings');
    expect(res.statusCode).toBe(200);
    expect(res.body).toHaveProperty('count');
    expect(Array.isArray(res.body.rows)).toBe(true);
    expect(res.body.rows.length).toBeGreaterThan(0);
  });

  it('should fetch a validator ranking by ID', async () => {
    const res = await request(app).get(`/api/validator-rankings/${rankingId}`);
    expect(res.statusCode).toBe(200);
    expect(res.body).toHaveProperty('id', rankingId);
  });

  it('should update a validator ranking', async () => {
    const res = await request(app)
      .put(`/api/validator-rankings/${rankingId}`)
      .send({ rank: 2, score: 88.8, notes: 'Updated notes' });
    expect(res.statusCode).toBe(200);
    expect(res.body.rank).toBe(2);
    expect(res.body.score).toBe(88.8);
    expect(res.body.notes).toBe('Updated notes');
  });

  it('should delete a validator ranking', async () => {
    const res = await request(app).delete(`/api/validator-rankings/${rankingId}`);
    expect(res.statusCode).toBe(200);
    expect(res.body).toHaveProperty('message', 'Ranking deleted successfully.');
  });

  it('should return 404 for non-existent ranking', async () => {
    const res = await request(app).get(`/api/validator-rankings/${rankingId}`);
    expect(res.statusCode).toBe(404);
    expect(res.body).toHaveProperty('error');
  });

  it('should return 404 for creating ranking with invalid validatorId', async () => {
    const res = await request(app)
      .post('/api/validator-rankings')
      .send({ validatorId: 'invalid-id', rank: 1, score: 99.5 });
    expect(res.statusCode).toBe(404);
    expect(res.body).toHaveProperty('error');
  });

  it('should return 404 for updating non-existent ranking', async () => {
    const res = await request(app)
      .put(`/api/validator-rankings/${rankingId}`)
      .send({ rank: 3 });
    expect(res.statusCode).toBe(404);
    expect(res.body).toHaveProperty('error');
  });

  it('should return 404 for deleting non-existent ranking', async () => {
    const res = await request(app).delete(`/api/validator-rankings/${rankingId}`);
    expect(res.statusCode).toBe(404);
    expect(res.body).toHaveProperty('error');
  });
}); 