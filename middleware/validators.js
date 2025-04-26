const Joi = require('joi');

exports.validateNotification = (req, res, next) => {
  const schema = Joi.object({
    type: Joi.string()
      .required()
      .valid('URGENT', 'FUND_RELEASE', 'BOUNTY_PAID', 'NEW_SUBMISSION'),
    
    title: Joi.string()
      .required()
      .min(3)
      .max(100),
    
    message: Joi.string()
      .required()
      .min(5)
      .max(500),
    
    priority: Joi.string()
      .required()
      .valid('HIGH', 'MEDIUM', 'LOW'),
    
    icon: Joi.string()
      .uri()
      .allow(null, ''),
    
    metadata: Joi.object()
      .default({}),
    
    actionRequired: Joi.boolean()
      .default(false),
    
    actionUrl: Joi.string()
      .uri()
      .allow(null, '')
  });

  const { error } = schema.validate(req.body);
  
  if (error) {
    return res.status(400).json({
      message: 'Invalid notification data',
      details: error.details.map(detail => detail.message)
    });
  }

  next();
}; 