const express = require('express');
const { body } = require('express-validator');
const router = express.Router();
const transactionController = require('../controllers/transactionController');


const transactionValidation = [
  body('userId').notEmpty().withMessage('User ID is required'),
  body('amount').isNumeric().withMessage('Amount must be a number'),
  body('reference').notEmpty().withMessage('Reference is required'),
];

router.post('/', transactionValidation, transactionController.createTransaction);
router.get('/', transactionController.getTransactions);
router.get('/:id', transactionController.getTransactionById);
router.put('/:id', transactionValidation, transactionController.updateTransaction);
router.delete('/:id', transactionController.deleteTransaction);

module.exports = router;
