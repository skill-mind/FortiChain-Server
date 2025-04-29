const { v4: uuidv4 } = require('uuid');
const Payout = require('../models/payout.model');

const PayoutController = {
  // Create a new payout request
  createPayout: async (req, res, next) => {
    try {
      const { userId, amount, currency = 'USD', fee = 0, metadata } = req.body;
      if (!userId || isNaN(amount) || Number(amount) <= 0) {
        return res.status(400).json({
          success: false,
          message: 'Valid userId and amount are required'
        });
      }

      const payout = await Payout.create({
        id: uuidv4(),
        userId,
        amount: Number(amount),
        currency,
        fee: Number(fee),
        metadata,
        status: 'Pending',
      });

      res.status(201).json({ success: true, data: payout });
    } catch (err) {
      next(err);
    }
  },

  // Get a single payout by its ID
  getPayoutById: async (req, res, next) => {
    try {
      const payout = await Payout.findByPk(req.params.id);
      if (!payout) {
        return res.status(404).json({ success: false, message: 'Payout not found' });
      }
      res.json({ success: true, data: payout });
    } catch (err) {
      next(err);
    }
  },

  // List payouts (with optional filtering / pagination)
  getAllPayouts: async (req, res, next) => {
    try {
      const { userId, status, limit = 20, offset = 0 } = req.query;
      const where = {};
      if (userId) where.userId = userId;
      if (status) where.status = status;

      const payouts = await Payout.findAll({
        where,
        limit: parseInt(limit),
        offset: parseInt(offset),
        order: [['requestedAt', 'DESC']],
      });
      res.json({ success: true, data: payouts });
    } catch (err) {
      next(err);
    }
  },

  // Update a payout (e.g. mark as Completed, set processedAt, change fee)
  updatePayout: async (req, res, next) => {
    try {
      const { id } = req.params;
      const updates = (({ status, processedAt, fee, metadata }) => ({ status, processedAt, fee, metadata }))(req.body);

      const [count, [updated]] = await Payout.update(updates, {
        where: { id },
        returning: true,
      });
      if (count === 0) {
        return res.status(404).json({ success: false, message: 'Payout not found' });
      }

      res.json({ success: true, data: updated });
    } catch (err) {
      next(err);
    }
  },

  // Delete a payout
  deletePayout: async (req, res, next) => {
    try {
      const count = await Payout.destroy({ where: { id: req.params.id } });
      if (count === 0) {
        return res.status(404).json({ success: false, message: 'Payout not found' });
      }
      res.json({ success: true, message: 'Payout deleted' });
    } catch (err) {
      next(err);
    }
  },
};

module.exports = PayoutController;
