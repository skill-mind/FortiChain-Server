const Joi = require('joi');

module.exports = {
  // Positive number validations
  positive: Joi.number().min(0).required(),
  optionalPositive: Joi.number().min(0),

  // Percentage validations (0-100)
  percentage: Joi.number().min(0).max(100).required(),
  optionalPercentage: Joi.number().min(0).max(100),

  // Currency amount validations
  currency: (min = 0.01) => Joi.number().min(min).precision(2).required().messages({
    'number.min': 'Amount must be greater than or equal to {{#limit}}'
  }),
  optionalCurrency: (min = 0.01) => Joi.number().min(min).precision(2),

  // Integer validations
  integer: (min = 0) => Joi.number().integer().min(min).required(),
  optionalInteger: (min = 0) => Joi.number().integer().min(min),

  // Score validations (0-10)
  score: Joi.number().min(0).max(10).precision(1).required(),
  optionalScore: Joi.number().min(0).max(10).precision(1),

  // File size validations (in bytes)
  fileSize: (maxSize) => Joi.number().min(0).max(maxSize).required().messages({
    'number.min': 'File size must be greater than or equal to 0',
    'number.max': 'File size must be less than or equal to {{#limit}}'
  }),
  optionalFileSize: (maxSize) => Joi.number().min(0).max(maxSize),

  // Custom range validations
  range: (min, max) => Joi.number().min(min).max(max).required(),
  optionalRange: (min, max) => Joi.number().min(min).max(max)
}; 