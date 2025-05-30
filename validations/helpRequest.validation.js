const Joi = require('joi');
const { string, file } = require('./core');

const helpRequestSchema = Joi.object({
  email: string.email,
  subject: string.optionalShortText().default('Vulnerability Report'),
  message: string.mediumText(10, 256),
  document: file.pdf()
});

const updateHelpRequestSchema = Joi.object({
  email: string.optionalEmail,
  subject: string.optionalShortText(),
  message: string.optionalMediumText(10, 256),
  document: file.pdf(),
  status: string.optionalEnum(['open', 'in_progress', 'resolved', 'closed'])
}).min(1);

module.exports = {
  helpRequestSchema,
  updateHelpRequestSchema
}; 