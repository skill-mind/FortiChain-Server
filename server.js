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

// Import logger
const logger = require('./utils/logger');

// Import database connection
const { dbConnection } = require('./models/index');

// Import existing routes
const walletRoutes = require('./routes/wallet.routes');
const projectRoutes = require('./routes/project.routes');
const userRoutes = require('./routes/user.routes');
const supportRoutes = require('./routes/support.routes');
const validatorRankingRoutes = require('./routes/validatorRanking.routes');
const profileRoutes = require('./routes/profile.routes');
const helpRequestRoutes = require('./routes/helpRequest.routes');

const tipsRoutes = require('./routes/tips.routes');
const payoutRoutes = require('./routes/payout.routes');
const transactionHistoryRoutes = require('./routes/transactionHistory.routes');
const reportRoutes = require('./routes/report.routes');
const notificationRoutes = require('./routes/notification.routes');
const authRoutes = require("./routes/auth.routes");

// Initialize express app
const app = express();

// Ensure logs directory exists
const logDir = path.join(__dirname, 'logs');
if (!fs.existsSync(logDir)) {
  fs.mkdirSync(logDir);
}

// Middleware
app.use(helmet());
app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));
app.use(morgan('combined', { stream: logger.stream }));

// Request logging middleware
app.use((req, res, next) => {
  logger.debug(`Request received: ${req.method} ${req.originalUrl}`);
  next();
});

// Mount routes
app.use('/api/wallets', walletRoutes);
app.use('/projects', projectRoutes);
app.use('/users', userRoutes);
app.use('/support', supportRoutes);
app.use('/api/validator-rankings', validatorRankingRoutes);
app.use('/api', profileRoutes);
app.use('/api/help-requests', helpRequestRoutes);

app.use('/api/tips', tipsRoutes);
app.use('/api/payouts', payoutRoutes);
app.use('/api/transaction-histories', transactionHistoryRoutes);
app.use('/api/notifications', notificationRoutes);

app.use('/api/reports', reportRoutes);
app.use("/api/auth", authRoutes);


// Default root route
app.get('/', (req, res) => {
  logger.info('Root endpoint accessed');
  res.send('FortiChain API is running');
});

// Error handling middleware
app.use((err, req, res, next) => {
  logger.error(`Error: ${err.message}`, { stack: err.stack });
  res.status(500).json({
    message: 'An internal server error occurred',
    error: process.env.NODE_ENV === 'production'
      ? {}
      : { message: err.message, stack: err.stack },
  });
});

module.exports = app;

if (require.main === module) {
  const PORT = process.env.PORT || 3000;
  const startServer = async () => {
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
  };

  startServer();
}
