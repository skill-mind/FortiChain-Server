const { DataTypes } = require('sequelize');
const sequelize = require('../config/db.config'); 

const Project = sequelize.define('Project', {
  // Stage 1: Project Details
  name: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  description: {
    type: DataTypes.TEXT,
    allowNull: false,
  },
  category: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  smartContractAddress: {
    type: DataTypes.STRING,
    allowNull: false,
    validate: {
      notEmpty: true, // Ensures the address is not empty
    },
  },
  contactInfo: {
    type: DataTypes.STRING,
    allowNull: false,
    validate: {
      isEmail: true, // Ensures valid email format
    },
  },
  supportingDocument: {
    type: DataTypes.STRING, // Path to the uploaded document
    allowNull: true,
  },
  projectLogo: {
    type: DataTypes.STRING, // Path to the uploaded logo
    allowNull: true,
  },

  // Stage 2: Ownership Verification
  repositoryHost: {
    type: DataTypes.STRING, // e.g., GitHub, GitLab, etc.
    allowNull: false,
  },
  repositoryName: {
    type: DataTypes.STRING, // Name of the repository
    allowNull: false,
  },
  repositoryLink: {
    type: DataTypes.STRING, // URL of the public repository
    allowNull: false,
    validate: {
      isUrl: true, // Ensures valid URL format
    },
  },

  // Stage 3: Bounty Allocation
  token: {
    type: DataTypes.STRING, // Token type, e.g., STRK
    allowNull: false,
  },
  escrowBalance: {
    type: DataTypes.FLOAT, // Escrow balance available for allocation
    defaultValue: 0,
  },
  walletBalance: {
    type: DataTypes.FLOAT, // Wallet balance available for use
    defaultValue: 0,
  },
  allocatedBounty: {
    type: DataTypes.FLOAT, // Amount allocated for the bounty
    allowNull: false,
  },
  bountyCurrency: {
    type: DataTypes.STRING, // Currency for the bounty, e.g., USD
    allowNull: false,
  },
  dateOfExpiry: {
    type: DataTypes.DATE, // Expiry date for the bounty allocation
    allowNull: false,
    validate: {
      isAfter: {
        args: new Date().toISOString().split('T')[0], // Ensures the date is in the future
        msg: "Date of expiry must be a future date",
      },
    },
  },
  autoTopUp: {
    type: DataTypes.BOOLEAN, // Auto-top-up flag
    defaultValue: false,
  },
}, {
  timestamps: true,
});

module.exports = Project;

