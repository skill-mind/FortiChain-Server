const Joi = require('joi');
const { string, number } = require('./core');

const validatorRankingSchema = Joi.object({
  validatorId: string.uuid,
  rank: number.integer(1),
  score: number.percentage,
  notes: string.optionalMediumText(0, 500)
});

const updateValidatorRankingSchema = Joi.object({
  validatorId: string.optionalUuid,
  rank: number.optionalInteger(1),
  score: number.optionalPercentage,
  notes: string.optionalMediumText(0, 500)
}).min(1);

module.exports = {
  create: validatorRankingSchema,
  update: updateValidatorRankingSchema
}; 