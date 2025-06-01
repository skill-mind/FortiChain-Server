const sequelize = require('../config/db.config');

// Import all models
const Wallet = require('./wallet.model');
const Project = require('./project.model');
const User = require('./user.model');
const SupportTicket = require('./supportTicket.model');
const ValidatorRanking = require('./validatorRanking.model');
const Profile = require('./profile.model');
const HelpRequest = require('./helpRequest.model');
const tips = require('./tips.models');
const Report = require('./report.model');
const Notification = require('./notification.model');

// Initialize model relationships (associations)
const initModels = () => {
  if (Wallet.associate) Wallet.associate({ Project, User, SupportTicket, ValidatorRanking });
  if (Project.associate) Project.associate({ Wallet, User, SupportTicket, ValidatorRanking });
  if (User.associate) User.associate({ Wallet, Project, SupportTicket, ValidatorRanking });
  if (SupportTicket.associate) SupportTicket.associate({ Wallet, Project, User, ValidatorRanking });
  if (ValidatorRanking.associate) ValidatorRanking.associate({ User });
  if (Report.associate) Report.associate({ /* Add associated models here if any */ });
};

// Test database connection and sync models
const dbConnection = async () => {
  try {
    await sequelize.authenticate();
    console.log(`✅ Database connection established successfully in ${process.env.NODE_ENV} mode.`);

    // Initialize model relationships
    initModels();

    // Sync all models to the database
    if (process.env.NODE_ENV === 'test') {
      await sequelize.sync({ force: true }); // Clean database for testing
    } else if (process.env.NODE_ENV === 'development') {
      await sequelize.sync({ alter: true }); // Update schema in development
    } else {
      await sequelize.sync(); // Default sync for production
    }

    console.log(`✅ Database synchronized successfully in ${process.env.NODE_ENV} mode.`);
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
  tips,
  Report,
  Notification,
};