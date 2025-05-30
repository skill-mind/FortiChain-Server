const express = require('express');
const Joi = require('joi');
const router = express.Router();
const { validateRequest, validateParams } = require('../middlewares/validationMiddleware');
const { tipSchema, updateTipSchema } = require('../validations/tip.validation');
const tipController = require('../controllers/tip.controller');
const authMiddleware = require('../middlewares/authMiddleware');

// Create a new tip
router.post(
  '/',
  authMiddleware.requireAuth,
  validateRequest(tipSchema),
  tipController.createTip
);

// Get all tips
router.get('/', tipController.getAllTips);

// Get tip by ID
router.get(
  '/:id',
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  tipController.getTipById
);

// Update tip
router.put(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validateRequest(updateTipSchema),
  tipController.updateTip
);

// Delete tip
router.delete(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  tipController.deleteTip
);

module.exports = router; 