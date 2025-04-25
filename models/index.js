const sequelize = require('../config/db.config');
const Wallet = require('./wallet.model');

// Test database connection
const dbConnection = async () => {
  try {
    await sequelize.authenticate();
    console.log('Database connection established successfully.');

    // Sync all defined models with the database
    await sequelize.sync({ alter: true });
    console.log('Database synchronized successfully.');

    return true;
  } catch (error) {
    console.error('Unable to connect to the database:', error);
    return false;
  }
};

module.exports = {
  sequelize,
  dbConnection,
  Wallet
};
