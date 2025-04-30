const { DataTypes } = require('sequelize');
const sequelize = require('../config/db.config');
const User = require('./user.model');

const Notification = sequelize.define('Notification', {
  id: {
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4,
    primaryKey: true,
  },
  userId: {
    type: DataTypes.UUID,
    allowNull: false,
    references: {
      model: User,
      key: 'id',
    },
    onDelete: 'CASCADE',
  },
  type: {
    type: DataTypes.ENUM('urgent', 'fund_release', 'bounty_paid', 'new_submission'),
    allowNull: false,
  },
  title: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  message: {
    type: DataTypes.TEXT,
    allowNull: false,
  },
  isRead: {
    type: DataTypes.BOOLEAN,
    defaultValue: false,
  },
  meta: {
    type: DataTypes.JSONB,
    allowNull: true,
  },
}, {
  timestamps: true,
  tableName: 'notifications',
});

Notification.associate = (models) => {
  Notification.belongsTo(models.User, { foreignKey: 'userId', as: 'user' });
};

module.exports = Notification;