const request = require('supertest');
const express = require('express');
const { requestContextMiddleware } = require('../../middlewares/requestContext');
const requestLoggerMiddleware = require('../../middlewares/requestLogger');
const { errorHandler, AppError } = require('../../middlewares/errorHandler');
const logger = require('../../utils/logger');

// Mock logger
jest.mock('../../utils/logger', () => ({
  info: jest.fn(),
  error: jest.fn(),
  stream: {
    write: jest.fn()
  }
}));

describe('Logging System', () => {
  let app;

  beforeEach(() => {
    app = express();
    app.use(express.json()); // Needed for POST body parsing
    app.use(requestContextMiddleware);
    app.use(requestLoggerMiddleware);

    // Test routes
    app.get('/test', (req, res) => {
      res.status(200).json({ message: 'success' });
    });
    app.post('/test', (req, res) => {
      res.status(200).json({ ok: true });
    });
    app.get('/error', (req, res, next) => {
      next(new AppError('Test error', 500));
    });

    app.use(errorHandler); // Error handler must be last

    // Clear mock calls
    jest.clearAllMocks();
  });

  it('should log successful request with request ID', async () => {
    const response = await request(app)
      .get('/test')
      .expect(200);

    // Check request ID header
    expect(response.headers['x-request-id']).toBeDefined();

    // Check logs
    expect(logger.info).toHaveBeenCalledWith('Request started', expect.any(Object));
    expect(logger.info).toHaveBeenCalledWith('Request completed', expect.objectContaining({
      statusCode: 200,
      duration: expect.any(Number)
    }));
  });

  it('should log error with request ID', async () => {
    const response = await request(app)
      .get('/error')
      .expect(500);

    // Check request ID header
    expect(response.headers['x-request-id']).toBeDefined();

    // Check error logs
    expect(logger.error).toHaveBeenCalledWith('Request failed', expect.objectContaining({
      error: expect.objectContaining({
        message: expect.any(String),
        stack: expect.any(String),
        statusCode: expect.any(Number),
        status: expect.any(String),
        isOperational: expect.any(Boolean)
      })
    }));
  });

  it('should redact sensitive information', async () => {
    const sensitiveData = {
      password: 'secret123',
      token: 'jwt-token',
      authorization: 'Bearer token'
    };

    await request(app)
      .post('/test')
      .send(sensitiveData)
      .expect(200);

    // Check that sensitive data was redacted
    const logCall = logger.info.mock.calls.find(call => call[0] === 'Request started');
    const logBody = JSON.parse(logCall[1].body);
    
    expect(logBody.password).toBe('[REDACTED]');
    expect(logBody.token).toBe('[REDACTED]');
    expect(logBody.authorization).toBe('[REDACTED]');
    expect(logBody.password).not.toBe('secret123');
    expect(logBody.token).not.toBe('jwt-token');
  });
}); 