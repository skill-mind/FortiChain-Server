const Joi = require('joi');

module.exports = {
  // File object validation
  fileObject: Joi.object({
    mimetype: Joi.string().required(),
    size: Joi.number().required(),
    name: Joi.string().required()
  }).required(),

  // Multiple files validation
  multipleFiles: Joi.array().items(
    Joi.object({
      mimetype: Joi.string().required(),
      size: Joi.number().required(),
      name: Joi.string().required()
    })
  ).required(),

  // File validation with options
  file: (options = {}) => {
    const schema = Joi.object({
      mimetype: Joi.string().required(),
      size: Joi.number().required(),
      name: Joi.string().required()
    });

    if (options.mimetypes) {
      schema.mimetype = Joi.string().valid(...options.mimetypes).required();
    }

    if (options.maxSize) {
      schema.size = Joi.number().max(options.maxSize).required();
    }

    return schema.required();
  },

  // Multiple files validation with options
  files: (options = {}) => {
    const fileSchema = module.exports.file(options);
    return Joi.array().items(fileSchema).required();
  },

  // Common file type validations
  image: (maxSize = 5 * 1024 * 1024) => module.exports.file({
    mimetypes: ['image/jpeg', 'image/png', 'image/gif'],
    maxSize
  }),

  pdf: (maxSize = 5 * 1024 * 1024) => module.exports.file({
    mimetypes: ['application/pdf'],
    maxSize
  }),

  document: (maxSize = 10 * 1024 * 1024) => module.exports.file({
    mimetypes: [
      'application/pdf',
      'application/msword',
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      'application/vnd.ms-excel',
      'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
    ],
    maxSize
  })
}; 