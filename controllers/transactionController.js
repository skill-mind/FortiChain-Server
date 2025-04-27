const mongoose = require('mongoose');
const { validationResult } = require('express-validator');
const Transaction = require('../models/Transaction');

// Create a Transaction
exports.createTransaction = async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    return res.status(400).json({ success: false, errors: errors.array() });
  }

  try {
    const { userId, amount, type, reference } = req.body;

    const transaction = new Transaction({
      userId,
      amount,
      type,
      reference,
    });

    const savedTransaction = await transaction.save();

    return res.status(201).json({ success: true, data: savedTransaction });
  } catch (err) {
    console.error('Create Transaction Error:', err);
    return res.status(500).json({ success: false, message: 'Server error' });
  }
};

// Get All Transactions
exports.getTransactions = async (req, res) => {
  try {
    const transactions = await Transaction.find();
    return res.json({ success: true, data: transactions });
  } catch (err) {
    console.error('Fetch Transactions Error:', err);
    return res.status(500).json({ success: false, message: 'Server error' });
  }
};

// Get Transaction by ID
exports.getTransactionById = async (req, res) => {
  try {
    const transactionId = req.params.id;

    if (!mongoose.Types.ObjectId.isValid(transactionId)) {
      return res.status(400).json({ success: false, message: 'Invalid transaction ID' });
    }

    const transaction = await Transaction.findById(transactionId);

    if (!transaction) {
      return res.status(404).json({ success: false, message: 'Transaction not found' });
    }

    return res.json({ success: true, data: transaction });
  } catch (err) {
    console.error('Fetch Transaction Error:', err);
    return res.status(500).json({ success: false, message: 'Server error' });
  }
};


exports.updateTransaction = async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    return res.status(400).json({ success: false, errors: errors.array() });
  }

  try {
    const transactionId = req.params.id;

    if (!mongoose.Types.ObjectId.isValid(transactionId)) {
      return res.status(400).json({ success: false, message: 'Invalid transaction ID' });
    }

    const { userId, amount, type, reference } = req.body;

    const updatedTransaction = await Transaction.findByIdAndUpdate(
      transactionId,
      { userId, amount, type, reference },
      { new: true, runValidators: true }
    );

    if (!updatedTransaction) {
      return res.status(404).json({ success: false, message: 'Transaction not found' });
    }

    return res.json({ success: true, data: updatedTransaction });
  } catch (err) {
    console.error('Update Transaction Error:', err);
    return res.status(500).json({ success: false, message: 'Server error' });
  }
};


exports.deleteTransaction = async (req, res) => {
  try {
    const transactionId = req.params.id;

    if (!mongoose.Types.ObjectId.isValid(transactionId)) {
      return res.status(400).json({ success: false, message: 'Invalid transaction ID' });
    }

    const transaction = await Transaction.findByIdAndDelete(transactionId);

    if (!transaction) {
      return res.status(404).json({ success: false, message: 'Transaction not found' });
    }

    return res.json({ success: true, message: 'Transaction deleted successfully' });
  } catch (err) {
    console.error('Delete Transaction Error:', err);
    return res.status(500).json({ success: false, message: 'Server error' });
  }
};
