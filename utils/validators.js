const Joi = require('joi');

// Wallet validation schema
const walletSchema = Joi.object({
  address: Joi.string()
    .required()
    .trim()
    .custom((value, helpers) => {
      // Ethereum address: 0x followed by 40 hex characters
      const ethereumPattern = /^0x[a-fA-F0-9]{40}$/;
      // Starknet address: 0x followed by 64 hex characters
      const starknetPattern = /^0x[a-fA-F0-9]{64}$/;
      // Stellar address: G followed by 55 characters (A-Z, 2-7)
      const stellarPattern = /^G[A-Z2-7]{55}$/;

      if (
        ethereumPattern.test(value) ||
        starknetPattern.test(value) ||
        stellarPattern.test(value)
      ) {
        return value;
      }

      // Custom error message that explains the expected formats
      return helpers.message('Invalid wallet address format. Please provide a valid Ethereum address (0x followed by 40 hex characters), Starknet address (0x followed by 64 hex characters), or Stellar address (G followed by 55 characters)');
    }, 'wallet address validation'),
  role: Joi.string()
    .required()
    .valid('project_owner', 'researcher', 'validator', 'admin')
});

// Project validation schema
const projectSchema = Joi.object({
  name: Joi.string().required().min(3).max(100),
  description: Joi.string().required().min(10).max(2000),
  category: Joi.string().required(),
  smartContractAddress: Joi.string().required(),
  contactInfo: Joi.string().email().required(),
  repositoryHost: Joi.string().required(),
  repositoryName: Joi.string().required(),
  repositoryLink: Joi.string().uri().required(),
  token: Joi.string().required(),
  bountyCurrency: Joi.string().required(),
  dateOfExpiry: Joi.date().iso().min('now').required(),
  allocatedBounty: Joi.number().min(0).required(),
  autoTopUp: Joi.boolean().default(false)
});

// Help request validation schema
const helpRequestSchema = Joi.object({
  email: Joi.string().email().required(),
  subject: Joi.string().default('Vulnerability Report'),
  message: Joi.string().min(10).max(256).required(),
});

// Document validation schema
const documentSchema = Joi.object({
  document: Joi.object({
    mimetype: Joi.string().valid('application/pdf').required(),
    size: Joi.number().max(5 * 1024 * 1024).required(), // 5MB max
  }).unknown(true),
});

// Validator ranking validation schema
const validatorRankingSchema = Joi.object({
  validatorId: Joi.string().uuid().required(),
  rank: Joi.number().integer().min(1).required(),
  score: Joi.number().min(0).max(100).required(),
  notes: Joi.string().max(500).allow('', null)
});

// Tip validation schema
const tipSchema = Joi.object({
  title: Joi.string().min(3).max(100).required(),
  content: Joi.string().min(10).max(5000).required(),
  category: Joi.string().required(),
  isBestModel: Joi.boolean().default(false)
});

// Report validation schema
const reportSchema = Joi.object({
  projectName: Joi.string().required(),
  projectStatus: Joi.string().valid('Ongoing', 'Closed').required(),
  submittedBy: Joi.string().required(),
  vulnerabilityTitle: Joi.string().required(),
  description: Joi.string().required(),
  severity: Joi.string().valid('Critical', 'High', 'Medium', 'Low').required(),
  proofOfConcept: Joi.array().items(Joi.string().uri()).max(5),
  status: Joi.string().valid('pending', 'approved', 'rejected').default('pending'),
  cvssScore: Joi.number().min(0).max(10).precision(1),
  bounty: Joi.number().min(0).precision(2),
  reviewerReward: Joi.number().min(0).precision(2),
  mitigation: Joi.string().allow('', null)
});

// Payout validation schema
const payoutSchema = Joi.object({
  userId: Joi.string().uuid().required(),
  amount: Joi.number().min(0.01).required(),
  currency: Joi.string().default('USD'),
  fee: Joi.number().min(0).default(0),
  metadata: Joi.object().allow(null)
});

// Support ticket reply validation schema
const supportReplySchema = Joi.object({
  message: Joi.string().required().min(1).max(1000)
});

// Validation functions
const validateWallet = (data) => {
  return walletSchema.validate(data, { abortEarly: false });
};

const validateProject = (data) => {
  return projectSchema.validate(data, { abortEarly: false });
};

const validateHelpRequest = (data) => {
  return helpRequestSchema.validate(data, { abortEarly: false });
};

const validateDocument = (data) => {
  return documentSchema.validate(data, { abortEarly: false });
};

const validateValidatorRanking = (data) => {
  return validatorRankingSchema.validate(data, { abortEarly: false });
};

const validateTip = (data) => {
  return tipSchema.validate(data, { abortEarly: false });
};

const validateReport = (data) => {
  return reportSchema.validate(data, { abortEarly: false });
};

const validatePayout = (data) => {
  return payoutSchema.validate(data, { abortEarly: false });
};

const validateSupportReply = (data) => {
  return supportReplySchema.validate(data, { abortEarly: false });
};

module.exports = {
  validateWallet,
  validateProject,
  validateHelpRequest,
  validateDocument,
  validateValidatorRanking,
  validateTip,
  validateReport,
  validatePayout,
  validateSupportReply,
  // Export schemas for direct use if needed
  walletSchema,
  projectSchema,
  helpRequestSchema,
  documentSchema,
  validatorRankingSchema,
  tipSchema,
  reportSchema,
  payoutSchema,
  supportReplySchema
};
