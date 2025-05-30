const Joi = require('joi');
const { string } = require('./core');

const walletSchema = Joi.object({
  address: Joi.alternatives().try(
    string.ethereumAddress,
    string.starknetAddress,
    string.stellarAddress
  ).required(),
  role: string.enum(['project_owner', 'researcher', 'validator', 'admin'])
});

const updateWalletSchema = Joi.object({
  address: Joi.alternatives().try(
    string.ethereumAddress,
    string.starknetAddress,
    string.stellarAddress
  ),
  role: string.optionalEnum(['project_owner', 'researcher', 'validator', 'admin'])
}).min(1);

module.exports = {
  create: walletSchema,
  update: updateWalletSchema
}; 