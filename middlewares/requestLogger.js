const logger = require('../utils/logger');

const safeStringify = (obj) => {
  try {
    const sanitized = { ...obj };
    ['password', 'token', 'secret', 'authorization'].forEach(field => {
      if (sanitized[field]) {
        sanitized[field] = '[REDACTED]';
      }
    });
    return JSON.stringify(sanitized);
  } catch (error) {
    return '[Unable to stringify]';
  }
};

const requestLoggerMiddleware = (req, res, next) => {
  const startTime = Date.now();

  logger.info('Request started', {
    body: safeStringify(req.body),
    query: req.query,
    params: req.params,
    headers: {
      ...req.headers,
      authorization: req.headers.authorization ? '[REDACTED]' : undefined
    }
  });

  const originalSend = res.send;
  res.send = function (body) {
    const duration = Date.now() - startTime;

    logger.info('Request completed', {
      statusCode: res.statusCode,
      duration,
      responseBody: typeof body === 'string' ? body : safeStringify(body)
    });
    
    res.send = originalSend;
    return res.send(body);
  };

  next();
};

module.exports = requestLoggerMiddleware; 