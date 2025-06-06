const Joi = require('joi');
const { string } = require('./core');

const tipSchema = Joi.object({
  title: string.shortText(3, 100),
  content: string.longText(10, 5000),
  category: string.shortText(),
  isBestModel: Joi.boolean().default(false)
});

const updateTipSchema = Joi.object({
  title: string.optionalShortText(3, 100),
  content: string.optionalLongText(10, 5000),
  category: string.optionalShortText(),
  isBestModel: Joi.boolean()
}).min(1);

module.exports = {
  create: tipSchema,
  update: updateTipSchema
}; 