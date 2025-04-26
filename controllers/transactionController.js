const Transaction = require('../models/Transaction');
const { validationResult } = require('express-validator');

exports.createTransaction = async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  try {
    const tx = new Transaction(req.body);
    await tx.save();
    return res.status(201).json(tx);
  } catch (err) {
    console.error('Create Transaction Error:', err);
    return res.status(500).json({ message: 'Server error' });
  }
};

exports.getTransactions = async (req, res) => {
  try {
    const { page = 1, limit = 25 } = req.query;
    const transactions = await Transaction.find()
      .populate('userId', 'name email')
      .sort({ transactionDate: -1 })
      .skip((page - 1) * limit)
      .limit(parseInt(limit));
    const count = await Transaction.countDocuments();

    return res.json({
      data: transactions,
      meta: {
        total: count,
        page: parseInt(page),
        pages: Math.ceil(count / limit),
      },
    });
  } catch (err) {
    console.error('Fetch Transactions Error:', err);
    return res.status(500).json({ message: 'Server error' });
  }
};

exports.getTransactionById = async (req, res) => {
  try {
    const tx = await Transaction.findById(req.params.id).populate('userId', 'name email');
    if (!tx) return res.status(404).json({ message: 'Transaction not found' });

    return res.json(tx);
  } catch (err) {
    console.error('Fetch Transaction Error:', err);
    return res.status(500).json({ message: 'Server error' });
  }
};

exports.updateTransaction = async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });

  try {
    const updated = await Transaction.findByIdAndUpdate(
      req.params.id,
      { $set: req.body },
      { new: true, runValidators: true }
    );
    if (!updated) return res.status(404).json({ message: 'Transaction not found' });

    return res.json(updated);
  } catch (err) {
    console.error('Update Transaction Error:', err);
    return res.status(500).json({ message: 'Server error' });
  }
};

exports.deleteTransaction = async (req, res) => {
  try {
    const deleted = await Transaction.findByIdAndDelete(req.params.id);
    if (!deleted) return res.status(404).json({ message: 'Transaction not found' });

    return res.json({ message: 'Transaction deleted successfully' });
  } catch (err) {
    console.error('Delete Transaction Error:', err);
    return res.status(500).json({ message: 'Server error' });
  }
};
