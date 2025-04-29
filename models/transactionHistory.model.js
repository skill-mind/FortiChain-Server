const { DataTypes } = require('sequelize');
const sequelize = require('../config/db.config');

const TransactionHistory = sequelize.define('TransactionHistory', {
  id: {
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4,
    primaryKey: true,
  },
  transactionId: {
    type: DataTypes.UUID,
    allowNull: false,
    unique: true,
    defaultValue: DataTypes.UUIDV4,
  },
  userId: {
    type: DataTypes.UUID,
    allowNull: false,
  },
  type: {
    type: DataTypes.ENUM(
      'Deposit',
      'Withdrawal',
      'Bounty Payment',
      'Transfer to Withdrawable',
      'Payout'
    ),
    allowNull: false,
  },
  amount: {
    type: DataTypes.DOUBLE,
    allowNull: false,
  },
  currency: {
    type: DataTypes.STRING,
    allowNull: false,
    defaultValue: 'USD',
  },
  status: {
    type: DataTypes.ENUM('Pending', 'Completed', 'Failed'),
    allowNull: false,
  },
  fee: {
    type: DataTypes.DOUBLE,
    allowNull: false,
    defaultValue: 0,
  },
  date: {
    type: DataTypes.DATE,
    allowNull: false,
    defaultValue: DataTypes.NOW,
  },
  metadata: {
    type: DataTypes.JSON,
    allowNull: true,
  },
}, {
  tableName: 'transaction_histories',
  timestamps: true,
});

module.exports = TransactionHistory;
