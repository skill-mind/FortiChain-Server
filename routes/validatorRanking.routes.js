const express = require('express');
const Joi = require('joi');
const router = express.Router();
const { validateRequest, validateParams } = require('../middlewares/validationMiddleware');
const { validatorRankingSchema, updateValidatorRankingSchema } = require('../validations/validatorRanking.validation');
const validatorRankingController = require('../controllers/validatorRanking.controller');
const { authenticate, authorize } = require('../middlewares/auth');
const { roles } = require('../config/roles');

// Only admins and super_admins can create rankings
router.post(
  '/',
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validatorRankingController.createRanking
);

// All authenticated users can view all rankings
router.get(
  '/',
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  validatorRankingController.getAllRankings
);

// Authenticated users can view specific ranking by ID
router.get(
  '/:id',
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  validatorRankingController.getRankingById
);

// Only admins and super_admins can update rankings
router.put(
  '/:id',
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validatorRankingController.updateRanking
);

// Only super_admins can delete rankings
router.delete(
  '/:id',
  authenticate,
  authorize(roles.SUPER_ADMIN),
  validatorRankingController.deleteRanking
);

module.exports = router;
