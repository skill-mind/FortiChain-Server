const winston = require('winston');
const { format, transports, createLogger } = winston;
const path = require('path');
const { getRequestContext } = require('../middlewares/requestContext');

const logFormat = format.combine(
  format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  format.errors({ stack: true }),
  format.splat(),
  format((info) => {
    const context = getRequestContext();
    if (context) {
      info.requestId = context.requestId;
      info.method = context.method;
      info.path = context.path;
      if (info.level === 'info' && info.message.includes('Request completed')) {
        info.duration = Date.now() - context.startTime;
      }
    }
    return info;
  })(),
  format.json()
);

const logger = createLogger({
  level: process.env.LOG_LEVEL || 'info',
  format: logFormat,
  defaultMeta: { service: 'fortichain-api' },
  transports: [
 
    new transports.Console({
      format: format.combine(
        format.colorize(),
        format.printf(
          info => {
            const requestInfo = info.requestId ? `[${info.requestId}] ` : '';
            const duration = info.duration ? ` (${info.duration}ms)` : '';
            return `${info.timestamp} ${requestInfo}${info.level}: ${info.message}${duration}${info.stack ? '\n' + info.stack : ''}`;
          }
        )
      )
    }),
    new transports.File({
      filename: path.join('logs', 'error.log'),
      level: 'error',
      maxsize: 10485760, // 10MB
      maxFiles: 5
    }),
  
    new transports.File({
      filename: path.join('logs', 'combined.log'),
      maxsize: 10485760, // 10MB
      maxFiles: 5
    })
  ]
});


logger.stream = {
  write: (message) => logger.info(message.trim())
};

module.exports = logger;
