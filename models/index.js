const sequelize = require('../config/db.config');
const Wallet = require('./wallet.model');

// Test database connection
const dbConnection = async () => {
  try {
    await sequelize.authenticate();
    console.log('✅ Database connection established successfully.');

    // Initialize model relationships
    initModels();

    // Sync all models to the database
    await sequelize.sync({ alter: process.env.NODE_ENV !== 'production' });
    console.log('✅ Database synchronized successfully.');

    return true;
  } catch (error) {
    console.error('❌ Unable to connect to the database:', error);
    return false;
  }
};

// Export everything
module.exports = {
  sequelize,
  dbConnection,
  Wallet,
  HelpRequest,
  Project,
  User,
  SupportTicket,
  ValidatorRanking
};
