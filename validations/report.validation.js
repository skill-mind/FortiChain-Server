const Joi = require('joi');
const { string, number } = require('./core');

const reportSchema = Joi.object({
  projectName: string.shortText(),
  projectStatus: string.enum(['Ongoing', 'Closed']),
  submittedBy: string.shortText(),
  vulnerabilityTitle: string.shortText(),
  description: string.longText(),
  severity: string.enum(['Critical', 'High', 'Medium', 'Low']),
  proofOfConcept: Joi.array().items(string.url).max(5),
  status: string.enum(['pending', 'approved', 'rejected']).default('pending'),
  cvssScore: number.optionalScore,
  bounty: number.optionalCurrency(0),
  reviewerReward: number.optionalCurrency(0),
  mitigation: string.optionalLongText()
});

const updateReportSchema = Joi.object({
  projectName: string.optionalShortText(),
  projectStatus: string.optionalEnum(['Ongoing', 'Closed']),
  submittedBy: string.optionalShortText(),
  vulnerabilityTitle: string.optionalShortText(),
  description: string.optionalLongText(),
  severity: string.optionalEnum(['Critical', 'High', 'Medium', 'Low']),
  proofOfConcept: Joi.array().items(string.url).max(5),
  status: string.optionalEnum(['pending', 'approved', 'rejected']),
  cvssScore: number.optionalScore,
  bounty: number.optionalCurrency(0),
  reviewerReward: number.optionalCurrency(0),
  mitigation: string.optionalLongText()
}).min(1);

module.exports = {
  create: reportSchema,
  update: updateReportSchema
}; 