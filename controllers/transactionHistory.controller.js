// controllers/transactionHistory.controller.js

const TransactionHistory = require('../models/transactionHistory.model');

const TransactionHistoryController = {
  // Create a new transaction history record
  createTransaction: async (req, res, next) => {
    try {
      const { userId, type, amount, currency = 'USD', status, fee = 0, metadata } = req.body;
      if (!userId || !type || isNaN(amount) || !status) {
        return res.status(400).json({
          success: false,
          message: 'Missing required transaction fields',
        });
      }

      const tx = await TransactionHistory.create({
        userId,
        type,
        amount: Number(amount),
        currency,
        status,
        fee: Number(fee),
        metadata,
      });

      return res.status(201).json({ success: true, data: tx });
    } catch (err) {
      next(err);
    }
  },

  // Get a single transaction by its primary key
  getTransactionById: async (req, res, next) => {
    try {
      const tx = await TransactionHistory.findByPk(req.params.id);
      if (!tx) {
        return res.status(404).json({ success: false, message: 'Transaction not found' });
      }
      return res.json({ success: true, data: tx });
    } catch (err) {
      next(err);
    }
  },

  // List transactions
  getAllTransactions: async (req, res, next) => {
    try {
      const { userId, type, status, limit = 20, offset = 0 } = req.query;
      const where = {};
      if (userId) where.userId = userId;
      if (type) where.type = type;
      if (status) where.status = status;

      const txs = await TransactionHistory.findAll({
        where,
        limit: parseInt(limit, 10),
        offset: parseInt(offset, 10),
        order: [['date', 'DESC']],
      });
      return res.json({ success: true, data: txs });
    } catch (err) {
      next(err);
    }
  },

  // Update a transaction’s status or metadata
  updateTransaction: async (req, res, next) => {
    try {
      const updates = (({ status, metadata }) => ({ status, metadata }))(req.body);

      // 1) run update
      const result = await TransactionHistory.update(updates, {
        where: { transactionId: req.params.transactionId },
      });
      const count = Array.isArray(result) ? result[0] : result;

      if (count === 0) {
        return res.status(404).json({ success: false, message: 'Transaction not found' });
      }

      // 2) re-fetch updated record
      const updated = await TransactionHistory.findOne({
        where: { transactionId: req.params.transactionId },
      });
      return res.json({ success: true, data: updated });
    } catch (err) {
      next(err);
    }
  },

  // Delete a transaction record
  deleteTransaction: async (req, res, next) => {
    try {
      const count = await TransactionHistory.destroy({
        where: { transactionId: req.params.transactionId },
      });
      if (count === 0) {
        return res.status(404).json({ success: false, message: 'Transaction not found' });
      }
      return res.json({ success: true, message: 'Transaction deleted' });
    } catch (err) {
      next(err);
    }
  },
};

module.exports = TransactionHistoryController;
