const sequelize = require('../config/db.config');

// Import all models
const Wallet = require('./wallet.model');
const Project = require('./project.model');
const User = require('./user.model');
const SupportTicket = require('./supportTicket.model');
const ValidatorRanking = require('./validatorRanking.model');
const Profile = require('./profile.model');
const HelpRequest = require('./helpRequest.model');


// Optionally define associations (if any)
const initModels = () => {
  if (Wallet.associate) Wallet.associate({ Project, User, SupportTicket, ValidatorRanking });
  if (Project.associate) Project.associate({ Wallet, User, SupportTicket, ValidatorRanking });
  if (User.associate) User.associate({ Wallet, Project, SupportTicket, ValidatorRanking });
  if (SupportTicket.associate) SupportTicket.associate({ Wallet, Project, User, ValidatorRanking });
  if (ValidatorRanking.associate) ValidatorRanking.associate({ User });
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
  SupportTicket,
  ValidatorRanking,
  Profile,
  HelpRequest,
};



