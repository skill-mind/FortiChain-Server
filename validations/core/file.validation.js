const Joi = require('joi');

const file = {
  // File object validation
  fileObject: Joi.object({
    fieldname: Joi.string().required(),
    originalname: Joi.string().required(),
    encoding: Joi.string().required(),
    mimetype: Joi.string().required(),
    size: Joi.number().min(0).required(),
    buffer: Joi.binary().required()
  }),

  // Multiple files validation
  multipleFiles: Joi.array().items(Joi.object({
    fieldname: Joi.string().required(),
    originalname: Joi.string().required(),
    encoding: Joi.string().required(),
    mimetype: Joi.string().required(),
    size: Joi.number().min(0).required(),
    buffer: Joi.binary().required()
  })),

  // File validation with options
  file: (options = {}) => {
    const schema = Joi.object({
      fieldname: Joi.string().required(),
      originalname: Joi.string().required(),
      encoding: Joi.string().required(),
      mimetype: Joi.string().required(),
      size: Joi.number().min(0).required(),
      buffer: Joi.binary().required()
    });

    if (options.mimetypes) {
      schema.mimetype = Joi.string().valid(...options.mimetypes).required().messages({
        'any.only': 'File type must be one of: {{#valids}}'
      });
    }

    if (options.maxSize) {
      schema.size = Joi.number().min(0).max(options.maxSize).required().messages({
        'number.max': `File size must be less than or equal to ${options.maxSize} bytes`
      });
    }

    return schema;
  },

  // Multiple files validation with options
  files: (options = {}) => {
    const schema = Joi.array().items(Joi.object({
      fieldname: Joi.string().required(),
      originalname: Joi.string().required(),
      encoding: Joi.string().required(),
      mimetype: Joi.string().required(),
      size: Joi.number().min(0).required(),
      buffer: Joi.binary().required()
    }));

    if (options.mimetypes) {
      schema.items.mimetype = Joi.string().valid(...options.mimetypes).required().messages({
        'any.only': 'File type must be one of: {{#valids}}'
      });
    }

    if (options.maxSize) {
      schema.items.size = Joi.number().min(0).max(options.maxSize).required().messages({
        'number.max': `File size must be less than or equal to ${options.maxSize} bytes`
      });
    }

    return schema;
  },

  // Common file type validations
  image: () => file.file({
    mimetypes: ['image/jpeg', 'image/png', 'image/gif'],
    maxSize: 5 * 1024 * 1024 // 5MB
  }),

  pdf: () => file.file({
    mimetypes: ['application/pdf'],
    maxSize: 10 * 1024 * 1024 // 10MB
  }),

  document: () => file.file({
    mimetypes: ['application/pdf', 'application/msword', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'],
    maxSize: 15 * 1024 * 1024 // 15MB
  })
};

module.exports = file; 