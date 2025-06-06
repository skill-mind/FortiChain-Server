const Joi = require('joi');
const { file } = require('./core/file.validation');
const { string } = require('./core/string.validation');

const helpRequestSchema = Joi.object({
  email: string.email,
  subject: string.shortText(1, 100),
  message: string.longText(1, 1000),
  document: file.document()
}).required();

const updateHelpRequestSchema = Joi.object({
  email: string.optionalEmail,
  subject: string.optionalShortText(1, 100),
  message: string.optionalLongText(1, 1000),
  document: file.document(),
  status: string.enum(['pending', 'in_progress', 'resolved', 'closed'])
}).min(1).required().messages({
  'object.min': 'At least one field must be provided for update'
});

module.exports = {
  helpRequestSchema,
  updateHelpRequestSchema
}; 