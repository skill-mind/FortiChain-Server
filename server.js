// server.js

const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const bodyParser = require('body-parser');
const dotenv = require('dotenv');
const morgan = require('morgan');
const fs = require('fs');
const path = require('path');




// Load environment variables
dotenv.config();

// Logger
const logger = require('./utils/logger');

// Database connection
const { dbConnection } = require('./models/index');

// Route modules
const walletRoutes = require('./routes/wallet.routes');
const projectRoutes = require('./routes/project.routes');
const userRoutes = require('./routes/user.routes');
const supportRoutes = require('./routes/support.routes');
const validatorRankingRoutes = require('./routes/validatorRanking.routes');
const transactionRoutes = require('./routes/transactionRoutes');
const payoutRoutes = require('./routes/payoutRoutes');
const profileRoutes = require('./routes/profile.routes');

// Initialize Express app
const app = express();

// Ensure logs directory exists
const logDir = path.join(__dirname, 'logs');
if (!fs.existsSync(logDir)) {
  fs.mkdirSync(logDir);
}

// Security & parsing middleware
app.use(helmet());
app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));
app.use(morgan('combined', { stream: logger.stream }));

// Request logging
app.use((req, res, next) => {
  logger.debug(`Request received: ${req.method} ${req.originalUrl}`);
  next();
});

// Mount feature routes
app.use('/api/wallets', walletRoutes);
app.use('/projects', projectRoutes);
app.use('/users', userRoutes);
app.use('/support', supportRoutes);
app.use('/api/validator-rankings', validatorRankingRoutes);
app.use('/api/transactions', transactionRoutes);
app.use('/api/payouts', payoutRoutes);
app.use('/api', profileRoutes);

// Root endpoint
app.get('/', (req, res) => {
  logger.info('Root endpoint accessed');
  res.send('FortiChain API is running');
});

// Error handling
app.use((err, req, res, next) => {
  logger.error(`Error: ${err.message}`, { stack: err.stack });
  res.status(500).json({
    message: 'An internal server error occurred',
    error: process.env.NODE_ENV === 'production'
      ? {}
      : { message: err.message, stack: err.stack },
  });
});

// Export app for tests
module.exports = app;

// Start server if run directly
if (require.main === module) {
  const PORT = process.env.PORT || 3000;
  (async () => {
    try {
      await dbConnection();
      logger.info('Database connection established successfully');
      app.listen(PORT, () => {
        logger.info(`Server is running on port ${PORT}`);
      });
    } catch (error) {
      logger.error('Failed to start server:', error);
      process.exit(1);
    }
  })();
}
