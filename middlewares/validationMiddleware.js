const logger = require('../utils/logger');

/**
 * Middleware to handle validation errors consistently across all endpoints
 * @param {Object} schema - The Joi schema to use for validation
 * @returns {Function} Express middleware function
 */
const validateRequest = (schema) => {
  return (req, res, next) => {
    const { error, value } = schema.validate(req.body, {
      abortEarly: false,
      stripUnknown: true
    });
    
    if (error) {
      logger.warn('Validation error:', {
        path: req.path,
        method: req.method,
        errors: error.details
      });

      return res.status(400).json({
        success: false,
        message: 'Validation failed',
        errors: error.details.map(detail => ({
          field: detail.path.join('.'),
          message: detail.message
        }))
      });
    }

    // Replace request body with validated value
    req.body = value;
    next();
  };
};

/**
 * Middleware to validate file uploads
 * @param {Object} schema - The Joi schema to use for file validation
 * @returns {Function} Express middleware function
 */
const validateFile = (schema) => {
  return (req, res, next) => {
    if (!req.files) {
      return next();
    }

    const { error } = schema.validate(req.files, {
      abortEarly: false,
      stripUnknown: true
    });

    if (error) {
      logger.warn('File validation error:', {
        path: req.path,
        method: req.method,
        errors: error.details
      });

      return res.status(400).json({
        success: false,
        message: 'File validation failed',
        errors: error.details.map(detail => ({
          field: detail.path.join('.'),
          message: detail.message
        }))
      });
    }

    next();
  };
};

/**
 * Middleware to validate URL parameters
 * @param {Object} schema - The Joi schema to use for parameter validation
 * @returns {Function} Express middleware function
 */
const validateParams = (schema) => {
  return (req, res, next) => {
    const { error, value } = schema.validate(req.params, {
      abortEarly: false,
      stripUnknown: true
    });
    
    if (error) {
      logger.warn('Parameter validation error:', {
        path: req.path,
        method: req.method,
        errors: error.details
      });

      return res.status(400).json({
        success: false,
        message: 'Invalid URL parameters',
        errors: error.details.map(detail => ({
          param: detail.path.join('.'),
          message: detail.message
        }))
      });
    }

    // Replace request params with validated value
    req.params = value;
    next();
  };
};

/**
 * Middleware to validate query parameters
 * @param {Object} schema - The Joi schema to use for query validation
 * @returns {Function} Express middleware function
 */
const validateQuery = (schema) => {
  return (req, res, next) => {
    const { error, value } = schema.validate(req.query, {
      abortEarly: false,
      stripUnknown: true
    });
    
    if (error) {
      logger.warn('Query validation error:', {
        path: req.path,
        method: req.method,
        errors: error.details
      });

      return res.status(400).json({
        success: false,
        message: 'Invalid query parameters',
        errors: error.details.map(detail => ({
          param: detail.path.join('.'),
          message: detail.message
        }))
      });
    }

    // Replace request query with validated value
    req.query = value;
    next();
  };
};

module.exports = {
  validateRequest,
  validateFile,
  validateParams,
  validateQuery
}; 