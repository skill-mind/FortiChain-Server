const request = require('supertest');
const db = require('../../models/index');

describe('Help Request API', () => {
  let server;
  let app;

  beforeAll(async () => {
    const express = require('express');
    const bodyParser = require('express').json;
    
    app = express();
    app.use(bodyParser());
    
    // Import your routes
    const helpRequestRoutes = require('../../routes/helpRequest.routes');
    app.use('/api/help-requests', helpRequestRoutes);
    
    // Initialize the database connection
    await db.sequelize.sync({ force: true });
  });

  afterAll(async () => {
    // Close the database connection
    await db.sequelize.close();
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

      expect(response.statusCode).toBe(201);
      expect(response.body).toHaveProperty('id');
      expect(response.body.email).toBe(helpRequestData.email);
      expect(response.body.subject).toBe(helpRequestData.subject);
      expect(response.body.message).toBe(helpRequestData.message);
      expect(response.body.status).toBe('open');
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

      expect(response.statusCode).toBe(400);
      expect(response.body).toHaveProperty('error');
    });
  });

  describe('GET /api/help-requests', () => {
    it('should retrieve all help requests', async () => {
      const response = await request(app)
        .get('/api/help-requests');

      expect(response.statusCode).toBe(200);
      expect(Array.isArray(response.body)).toBe(true);
    });
  });
});