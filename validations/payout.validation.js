const Joi = require('joi');
const { string, number } = require('./core');

const payoutSchema = Joi.object({
  userId: string.uuid,
  amount: number.currency(0.01),
  currency: string.shortText().default('USD'),
  fee: number.currency(0).default(0),
  metadata: Joi.object().allow(null)
});

const updatePayoutSchema = Joi.object({
  userId: string.optionalUuid,
  amount: number.optionalCurrency(0.01),
  currency: string.optionalShortText(),
  fee: number.optionalCurrency(0),
  metadata: Joi.object().allow(null),
  status: string.optionalEnum(['Pending', 'Completed', 'Failed'])
}).min(1);

module.exports = {
  create: payoutSchema,
  update: updatePayoutSchema
}; 