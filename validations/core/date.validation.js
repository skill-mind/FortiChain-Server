const Joi = require('joi');

module.exports = {
  // ISO date validations
  isoDate: Joi.date().iso().required(),
  optionalIsoDate: Joi.date().iso(),

  // Future date validations
  futureDate: Joi.date().min('now').required(),
  optionalFutureDate: Joi.date().min('now'),

  // Past date validations
  pastDate: Joi.date().max('now').required(),
  optionalPastDate: Joi.date().max('now'),

  // Date range validations
  dateRange: (min, max) => Joi.date().min(min).max(max).required(),
  optionalDateRange: (min, max) => Joi.date().min(min).max(max),

  // Timestamp validations
  timestamp: Joi.date().timestamp().required(),
  optionalTimestamp: Joi.date().timestamp()
}; 