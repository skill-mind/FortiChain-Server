const Joi = require('joi');

module.exports = {
  // Email validation
  email: Joi.string().email().required(),
  optionalEmail: Joi.string().email(),

  // URL validation
  url: Joi.string().uri().required(),
  optionalUrl: Joi.string().uri(),

  // UUID validation
  uuid: Joi.string().uuid().required().messages({
    'string.guid': 'Invalid UUID format'
  }),
  optionalUuid: Joi.string().uuid(),

  // Blockchain address validation
  ethereumAddress: Joi.string().pattern(/^0x[a-fA-F0-9]{40}$/).required(),
  starknetAddress: Joi.string().pattern(/^0x[a-fA-F0-9]{64}$/).required(),
  stellarAddress: Joi.string().pattern(/^G[A-Z2-7]{55}$/).required(),

  // Common string validations with length constraints
  shortText: (min = 3, max = 100) => Joi.string().min(min).max(max).required(),
  mediumText: (min = 10, max = 500) => Joi.string().min(min).max(max).required(),
  longText: (min = 10, max = 2000) => Joi.string().min(min).max(max).required(),

  // Optional text validations with length constraints
  optionalShortText: (min = 3, max = 100) => Joi.string().min(min).max(max),
  optionalMediumText: (min = 10, max = 500) => Joi.string().min(min).max(max),
  optionalLongText: (min = 10, max = 2000) => Joi.string().min(min).max(max),

  // Enum validations
  enum: (values) => Joi.string().valid(...values).required().messages({
    'any.only': 'Invalid value'
  }),
  optionalEnum: (values) => Joi.string().valid(...values),

  // File validation
  fileType: (types) => Joi.string().valid(...types).required().messages({
    'any.only': 'Invalid file type'
  }),
  optionalFileType: (types) => Joi.string().valid(...types)
}; 