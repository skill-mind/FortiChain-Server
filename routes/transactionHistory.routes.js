const express = require('express');
const TransactionHistoryController = require('../controllers/transactionHistory.controller');

const router = express.Router();

router.post('/', TransactionHistoryController.createTransaction);
router.get('/:id', TransactionHistoryController.getTransactionById);
router.get('/', TransactionHistoryController.getAllTransactions);
router.patch('/:transactionId', TransactionHistoryController.updateTransaction);
router.delete('/:transactionId', TransactionHistoryController.deleteTransaction);

module.exports = router;
