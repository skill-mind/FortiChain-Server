const { AsyncLocalStorage } = require('async_hooks');
const { v4: uuidv4 } = require('uuid');


const asyncLocalStorage = new AsyncLocalStorage();


const requestContextMiddleware = (req, res, next) => {
  const requestId = req.headers['x-request-id'] || uuidv4();
  const startTime = Date.now();


  const context = {
    requestId,
    startTime,
    method: req.method,
    path: req.originalUrl,
    ip: req.ip,
    userAgent: req.get('user-agent'),
  };


  asyncLocalStorage.run(context, () => {
 
    res.setHeader('X-Request-ID', requestId);
    next();
  });
};
const getRequestContext = () => {
  return asyncLocalStorage.getStore();
};

module.exports = {
  requestContextMiddleware,
  getRequestContext,
}; 