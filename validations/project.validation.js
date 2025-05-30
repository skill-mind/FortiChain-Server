const Joi = require('joi');
const { string, number, date, file } = require('./core');

const projectSchema = Joi.object({
  name: string.shortText(3, 100),
  description: string.longText(10, 2000),
  category: string.shortText(),
  smartContractAddress: string.ethereumAddress,
  contactInfo: string.email,
  repositoryHost: string.shortText(),
  repositoryName: string.shortText(),
  repositoryLink: string.url,
  token: string.shortText(),
  bountyCurrency: string.shortText(),
  dateOfExpiry: date.futureDate,
  allocatedBounty: number.currency(0),
  autoTopUp: Joi.boolean().default(false),
  supportingDocument: file.document(),
  projectLogo: file.image()
});

const updateProjectSchema = Joi.object({
  name: string.optionalShortText(3, 100),
  description: string.optionalLongText(10, 2000),
  category: string.optionalShortText(),
  smartContractAddress: string.optionalEnum([string.ethereumAddress]),
  contactInfo: string.optionalEmail,
  repositoryHost: string.optionalShortText(),
  repositoryName: string.optionalShortText(),
  repositoryLink: string.optionalUrl,
  token: string.optionalShortText(),
  bountyCurrency: string.optionalShortText(),
  dateOfExpiry: date.optionalFutureDate,
  allocatedBounty: number.optionalCurrency(0),
  autoTopUp: Joi.boolean(),
  supportingDocument: file.document(),
  projectLogo: file.image()
}).min(1);

module.exports = {
  create: projectSchema,
  update: updateProjectSchema
}; 