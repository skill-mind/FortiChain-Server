const Joi = require('joi');

const number = {
  // Basic number validations
  required: Joi.number().required(),
  optional: Joi.number().allow(null),
  integer: (min = 0) => Joi.number().integer().min(min).required(),
  optionalInteger: (min = 0) => Joi.number().integer().min(min).allow(null),

  // Positive number validations
  positive: Joi.number().positive().required(),
  optionalPositive: Joi.number().positive().allow(null),

  // Percentage validation (0-100)
  percentage: Joi.number().min(0).max(100).required(),
  optionalPercentage: Joi.number().min(0).max(100).allow(null),

  // Currency validation
  currency: (min = 0.01) => Joi.number().min(min).precision(2).required().messages({
    'number.min': 'Amount must be greater than or equal to {{#limit}}',
    'number.precision': 'Amount must have at most 2 decimal places'
  }),
  optionalCurrency: (min = 0.01) => Joi.number().min(min).precision(2).allow(null),

  // Integer range validation
  integerRange: (min, max) => Joi.number().integer().min(min).max(max).required(),
  optionalIntegerRange: (min, max) => Joi.number().integer().min(min).max(max).allow(null),

  // Score validation (0-100)
  score: Joi.number().min(0).max(100).required(),
  optionalScore: Joi.number().min(0).max(100).allow(null),

  // File size validation (in bytes)
  fileSize: (maxSize) => Joi.number().min(0).max(maxSize).required().messages({
    'number.min': 'File size must be greater than or equal to 0',
    'number.max': `File size must be less than or equal to ${maxSize} bytes`
  }),
  optionalFileSize: (maxSize) => Joi.number().min(0).max(maxSize).allow(null),

  // Custom range validation
  range: (min, max) => Joi.number().min(min).max(max).required(),
  optionalRange: (min, max) => Joi.number().min(min).max(max).allow(null)
};

module.exports = number; 