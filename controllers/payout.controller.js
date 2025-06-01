// controllers/payout.controller.js

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
          message: 'Valid userId and amount are required',
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

      return res.status(201).json({ success: true, data: payout });
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
      return res.json({ success: true, data: payout });
    } catch (err) {
      next(err);
    }
  },

  // List payouts
  getAllPayouts: async (req, res, next) => {
    try {
      const { userId, status, limit = 20, offset = 0 } = req.query;
      const where = {};
      if (userId) where.userId = userId;
      if (status) where.status = status;

      const payouts = await Payout.findAll({
        where,
        limit: parseInt(limit, 10),
        offset: parseInt(offset, 10),
        order: [['requestedAt', 'DESC']],
      });
      return res.json({ success: true, data: payouts });
    } catch (err) {
      next(err);
    }
  },

  // Update a payout
  updatePayout: async (req, res, next) => {
    try {
      const { id } = req.params;
      const updates = (({ status, processedAt, fee, metadata }) => ({ status, processedAt, fee, metadata }))(req.body);

      // 1) run update
      const result = await Payout.update(updates, { where: { id } });
      // result might be a number or [number]
      const count = Array.isArray(result) ? result[0] : result;

      if (count === 0) {
        return res.status(404).json({ success: false, message: 'Payout not found' });
      }

      // 2) re-fetch the updated record
      const updated = await Payout.findByPk(id);
      return res.json({ success: true, data: updated });
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
      return res.json({ success: true, message: 'Payout deleted' });
    } catch (err) {
      next(err);
    }
  },
};

module.exports = PayoutController;
