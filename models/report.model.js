const { DataTypes } = require('sequelize');
const sequelize = require('../config/db.config');

const Report = sequelize.define('Report', {
  id: {
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4,
    primaryKey: true,
  },
  projectName: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  projectStatus: {
    type: DataTypes.ENUM('Ongoing', 'Closed'),
    allowNull: false,
  },
  submittedBy: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  reviewedBy: {
    type: DataTypes.STRING,
    allowNull: true,
  },
  vulnerabilityTitle: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  affectedUrl: {
    type: DataTypes.STRING,
    allowNull: true,
  },
  parameterAffected: {
    type: DataTypes.STRING,
    allowNull: true,
  },
  description: {
    type: DataTypes.TEXT,
    allowNull: false,
  },
  impact: {
    type: DataTypes.TEXT,
    allowNull: true,
  },
  stepsToReproduce: {
    type: DataTypes.TEXT,
    allowNull: true,
  },
  severity: {
    type: DataTypes.ENUM('Critical', 'High', 'Medium', 'Low'),
    allowNull: false,
  },
  cvssScore: {
    type: DataTypes.DECIMAL(3, 1),
    allowNull: true,
  },
  proofOfConcept: {
    type: DataTypes.JSON, // Use JSON instead of ARRAY for compatibility with SQLite
    allowNull: true,
  },
  mitigation: {
    type: DataTypes.TEXT,
    allowNull: true,
  },
  bounty: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: true,
  },
  reviewerReward: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: true,
  },
  vulnerabilitiesFound: {
    type: DataTypes.INTEGER,
    allowNull: true,
  },
  totalBountyPaid: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: true,
  },
  bountyAllocated: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: true,
  },
  expiryDate: {
    type: DataTypes.DATE,
    allowNull: true,
  },
  status: {
    type: DataTypes.ENUM('pending', 'approved', 'rejected'),
    defaultValue: 'pending',
  },
}, {
  timestamps: true,
});

module.exports = Report;
