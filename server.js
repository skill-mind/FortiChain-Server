const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const bodyParser = require('body-parser');
const dotenv = require('dotenv');
const morgan = require('morgan');
const fs = require('fs');
const path = require('path');

dotenv.config();

const logger = require('./utils/logger');
const { requestContextMiddleware } = require('./middlewares/requestContext');
const requestLoggerMiddleware = require('./middlewares/requestLogger');
const { errorHandler } = require('./middlewares/errorHandler');
const { dbConnection } = require('./models/index');

// Import routes
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

const app = express();

// Ensure logs directory exists
const logDir = path.join(__dirname, 'logs');
if (!fs.existsSync(logDir)) {
  fs.mkdirSync(logDir);
}

// Security middleware
app.use(helmet());
app.use(cors());

// Body parsing middleware
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// Request context and logging middleware
app.use(requestContextMiddleware);
app.use(requestLoggerMiddleware);
app.use(morgan('combined', { stream: logger.stream }));

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

app.use(errorHandler);

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
