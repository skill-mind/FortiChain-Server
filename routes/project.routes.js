const express = require('express');
const { body } = require('express-validator');
const router = express.Router();
const payoutController = require('../controllers/payoutController');

const payoutValidation = [
  body('userId').notEmpty().withMessage('User ID is required'),
  body('amount').isNumeric().withMessage('Amount must be a number'),
  body('method').notEmpty().withMessage('Payout method is required'),
];

router.post('/', payoutValidation, payoutController.createPayout);
router.get('/', payoutController.getPayouts);
router.get('/:id', payoutController.getPayoutById);
router.put('/:id', payoutValidation, payoutController.updatePayout);
router.delete('/:id', payoutController.deletePayout);

module.exports = router;
