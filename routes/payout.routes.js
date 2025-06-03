const express = require('express');
const PayoutController = require('../controllers/payout.controller');

const router = express.Router();

router.post('/', PayoutController.createPayout);
router.get('/:id', PayoutController.getPayoutById);
router.get('/', PayoutController.getAllPayouts);
router.patch('/:id', PayoutController.updatePayout);
router.delete('/:id', PayoutController.deletePayout);

module.exports = router;
