const express = require('express');
const Joi = require('joi');
const router = express.Router();
const { validateRequest, validateParams } = require('../middlewares/validationMiddleware');
const { payoutSchema, updatePayoutSchema } = require('../validations/payout.validation');
const payoutController = require('../controllers/payout.controller');
const authMiddleware = require('../middlewares/authMiddleware');

// Create a new payout
router.post(
  '/',
  authMiddleware.requireAuth,
  validateRequest(payoutSchema),
  payoutController.createPayout
);

// Get all payouts
router.get(
  '/',
  authMiddleware.requireAuth,
  payoutController.getAllPayouts
);

// Get payout by ID
router.get(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  payoutController.getPayoutById
);

// Update payout
router.put(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validateRequest(updatePayoutSchema),
  payoutController.updatePayout
);

// Delete payout
router.delete(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  payoutController.deletePayout
);

module.exports = router;
