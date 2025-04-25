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

// Import routes
const walletRoutes = require('./routes/wallet.routes');

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

// Routes
app.use('/api/wallets', walletRoutes);

// Default route
app.get('/', (req, res) => {
  logger.info('Root endpoint accessed');
  res.send('FortiChain API is running');
});

// Error handling middleware
app.use((err, req, res, next) => {
  logger.error(`Error: ${err.message}`, { stack: err.stack });
  res.status(500).json({
    message: 'An internal server error occurred',
    error: process.env.NODE_ENV === 'production' ? {} : { message: err.message, stack: err.stack }
  });
});

// Start server and connect to database
const PORT = process.env.PORT || 3000;
const startServer = async () => {
  try {
    // Connect to database
    await dbConnection();
    logger.info('Database connection established successfully');

    // Start server
    app.listen(PORT, () => {
      logger.info(`Server is running on port ${PORT}`);
    });
  } catch (error) {
    logger.error('Failed to start server:', error);
    process.exit(1);
  }
};

startServer();
