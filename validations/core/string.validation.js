const Joi = require('joi');

const string = {
  // Basic string validations
  required: Joi.string().required(),
  optional: Joi.string().allow('', null),

  // Email validations
  email: Joi.string().email().required(),
  optionalEmail: Joi.string().email().allow('', null),

  // URL validations
  url: Joi.string().uri().required(),
  optionalUrl: Joi.string().uri().allow('', null),

  // UUID validation
  uuid: Joi.string().guid({ version: 'uuidv4' }).required().messages({
    'string.guid': 'Invalid UUID format'
  }),
  optionalUuid: Joi.string().guid({ version: 'uuidv4' }).allow('', null),

  // Text length validations
  shortText: (min = 1, max = 50) => Joi.string().min(min).max(max).required(),
  optionalShortText: (min = 1, max = 50) => Joi.string().min(min).max(max).allow('', null),
  mediumText: (min = 1, max = 200) => Joi.string().min(min).max(max).required(),
  optionalMediumText: (min = 1, max = 200) => Joi.string().min(min).max(max).allow('', null),
  longText: (min = 1, max = 1000) => Joi.string().min(min).max(max).required(),
  optionalLongText: (min = 1, max = 1000) => Joi.string().min(min).max(max).allow('', null),

  // Enum validation
  enum: (values) => Joi.string().valid(...values).required().messages({
    'any.only': 'Invalid value. Must be one of: {{#valids}}'
  }),
  optionalEnum: (values) => Joi.string().valid(...values).allow('', null),

  // File type validation
  fileType: (types) => Joi.string().valid(...types).required().messages({
    'any.only': 'Invalid file type. Must be one of: {{#valids}}'
  }),
  optionalFileType: (types) => Joi.string().valid(...types).allow('', null),

  // Blockchain address validations
  ethereumAddress: Joi.string().pattern(/^0x[a-fA-F0-9]{40}$/).required().messages({
    'string.pattern.base': 'Invalid Ethereum address format'
  }),
  optionalEthereumAddress: Joi.string().pattern(/^0x[a-fA-F0-9]{40}$/).allow('', null),

  starknetAddress: Joi.string().pattern(/^0x[a-fA-F0-9]{64}$/).required().messages({
    'string.pattern.base': 'Invalid Starknet address format'
  }),
  optionalStarknetAddress: Joi.string().pattern(/^0x[a-fA-F0-9]{64}$/).allow('', null),

  stellarAddress: Joi.string().pattern(/^G[a-zA-Z0-9]{55}$/).required().messages({
    'string.pattern.base': 'Invalid Stellar address format'
  }),
  optionalStellarAddress: Joi.string().pattern(/^G[a-zA-Z0-9]{55}$/).allow('', null)
};

module.exports = string; 