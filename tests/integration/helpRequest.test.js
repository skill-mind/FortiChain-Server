const request = require('supertest');
const { sequelize } = require('../../models');
const app = require('../../server');
const HelpRequest = require('../../models/helpRequest.model'); // Make sure to import the model correctly

describe('Help Request API', () => {
  beforeAll(async () => {
    // Initialize database connection and sync models
    await sequelize.authenticate();
    await sequelize.sync({ force: true });
  });

  afterAll(async () => {
    await sequelize.close();
  });

  describe('POST /api/help-requests', () => {
    it('should create a new help request', async () => {
      const helpRequestData = {
        email: 'test@example.com',
        subject: 'Test Subject',
        message: 'This is a test message with sufficient length.'
      };

      const response = await request(app)
        .post('/api/help-requests')
        .send(helpRequestData);

      expect(response.status).toBe(201);
      expect(response.body).toHaveProperty('id');
    });

    it('should reject invalid email', async () => {
      const helpRequestData = {
        email: 'invalid-email',
        subject: 'Test Subject',
        message: 'This is a test message with sufficient length.'
      };

      const response = await request(app)
        .post('/api/help-requests')
        .send(helpRequestData);

      expect(response.status).toBe(400);
    });
  });

  describe('GET /api/help-requests', () => {
    it('should retrieve all help requests', async () => {
      // Create test data first using the actual model
      await HelpRequest.create({
        email: 'test2@example.com',
        subject: 'Test Subject 2',
        message: 'Another test message'
      });

      const response = await request(app)
        .get('/api/help-requests');

      expect(response.status).toBe(200);
      expect(Array.isArray(response.body)).toBe(true);
      expect(response.body.length).toBeGreaterThan(0);
    });
  });
});