const sequelize = require('../config/db.config');

// Import all models
const Wallet = require('./wallet.model');
const Project = require('./project.model');
const User = require('./user.model');
const SupportTicket = require('./supportTicket.model');

// Optionally define associations (if any)
const initModels = () => {
  if (Wallet.associate) Wallet.associate({ Project, User, SupportTicket });
  if (Project.associate) Project.associate({ Wallet, User, SupportTicket });
  if (User.associate) User.associate({ Wallet, Project, SupportTicket });
  if (SupportTicket.associate) SupportTicket.associate({ Wallet, Project, User });
};

// Test database connection and sync models
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
  Project,
  User,
  SupportTicket
};
