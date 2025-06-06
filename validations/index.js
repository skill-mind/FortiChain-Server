const core = require('./core');
const wallet = require('./wallet.validation');
const project = require('./project.validation');
const helpRequest = require('./helpRequest.validation');
const validatorRanking = require('./validatorRanking.validation');
const tip = require('./tip.validation');
const report = require('./report.validation');
const payout = require('./payout.validation');

module.exports = {
  core,
  wallet,
  project,
  helpRequest,
  validatorRanking,
  tip,
  report,
  payout
}; 