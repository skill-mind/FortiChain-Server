const sequelize = require('../config/db.config');


const Wallet = require('./wallet.model');
const Project = require('./project.model');
const User = require('./user.model');
const SupportTicket = require('./supportTicket.model');
const Profile = require('./profile.model');

const initModels = () => {
  if (Wallet.associate) Wallet.associate({ Project, User, SupportTicket });
  if (Project.associate) Project.associate({ Wallet, User, SupportTicket });
  if (User.associate) User.associate({ Wallet, Project, SupportTicket });
  if (SupportTicket.associate) SupportTicket.associate({ Wallet, Project, User });
};

const dbConnection = async () => {
  try {
    await sequelize.authenticate();
    console.log('✅ Database connection established successfully.');

    initModels();

    await sequelize.sync({ alter: process.env.NODE_ENV !== 'production' });
    console.log('✅ Database synchronized successfully.');

    return true;
  } catch (error) {
    console.error('❌ Unable to connect to the database:', error);
    return false;
  }
};

module.exports = {
  sequelize,
  dbConnection,
  Wallet,
  Project,
  User,
  SupportTicket,
  Profile
};



