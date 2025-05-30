const express = require('express');
const Joi = require('joi');
const router = express.Router();
const { validateRequest, validateParams } = require('../middlewares/validationMiddleware');
const { validatorRankingSchema, updateValidatorRankingSchema } = require('../validations/validatorRanking.validation');
const validatorRankingController = require('../controllers/validatorRanking.controller');
const authMiddleware = require('../middlewares/authMiddleware');

// Create a new validator ranking
router.post(
  '/',
  authMiddleware.requireAuth,
  validateRequest(validatorRankingSchema),
  validatorRankingController.createValidatorRanking
);

// Get all validator rankings
router.get(
  '/',
  authMiddleware.requireAuth,
  validatorRankingController.getAllValidatorRankings
);

// Get validator ranking by ID
router.get(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validatorRankingController.getValidatorRankingById
);

// Update validator ranking
router.put(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validateRequest(updateValidatorRankingSchema),
  validatorRankingController.updateValidatorRanking
);

// Delete validator ranking
router.delete(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validatorRankingController.deleteValidatorRanking
);

module.exports = router; 