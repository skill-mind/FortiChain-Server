const request = require('supertest');
const app = require('../../server');
const { Notification, User, sequelize } = require('../../models');

describe('Notification API Integration', () => {
  let notificationId;
  let userId;

  beforeAll(async () => {
    await sequelize.sync({ force: true }); // Reset DB for tests

    // Create a test user (add walletAddress as required by your model)
    const user = await User.create({
      username: 'testuser',
      email: 'test@example.com',
      password: 'password123',
      walletAddress: '0x1234567890abcdef1234567890abcdef12345678'
    });
    userId = user.id;
  });

  afterAll(async () => {
    await sequelize.close();
  });

  it('should create a notification', async () => {
    const res = await request(app)
      .post('/api/notifications')
      .send({
        userId,
        type: 'urgent',
        title: 'Test Notification',
        message: 'This is a test notification.',
      });
    expect(res.statusCode).toBe(201);
    expect(res.body).toHaveProperty('id');
    notificationId = res.body.id;
  });

  it('should get all notifications', async () => {
    const res = await request(app).get('/api/notifications');
    expect(res.statusCode).toBe(200);
    expect(Array.isArray(res.body)).toBe(true);
  });

  it('should get a notification by id', async () => {
    const res = await request(app).get(`/api/notifications/${notificationId}`);
    expect(res.statusCode).toBe(200);
    expect(res.body.id).toBe(notificationId);
  });

  it('should update a notification', async () => {
    const res = await request(app)
      .put(`/api/notifications/${notificationId}`)
      .send({ isRead: true });
    expect(res.statusCode).toBe(200);
    expect(res.body.isRead).toBe(true);
  });

  it('should delete a notification', async () => {
    const res = await request(app).delete(`/api/notifications/${notificationId}`);
    expect(res.statusCode).toBe(204);
  });
});