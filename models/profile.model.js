const { DataTypes } = require('sequelize');
const sequelize = require('../config/db.config');
const User = require('./user.model');

const Profile = sequelize.define('Profile', {
  id: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true,
  },
  userId: {
    type: DataTypes.UUID,
    allowNull: false,
    unique: true,
    references: {
      model: 'Users',
      key: 'id',
    },
  },
  personalInfo: {
    type: DataTypes.JSON,
    defaultValue: {},
  },
  professionalBackground: {
    type: DataTypes.JSON,
    defaultValue: {},
  },
  verificationHistory: {
    type: DataTypes.JSON,
    defaultValue: [],
  },
  userState: {
    type: DataTypes.ENUM('active', 'suspended'),
    defaultValue: 'active',
    allowNull: false,
  },
}, {
  tableName: 'Profiles',
  timestamps: true,
});

Profile.belongsTo(User, { foreignKey: 'userId', as: 'user' });

module.exports = Profile;