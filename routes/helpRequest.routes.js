const express = require('express');
const Joi = require('joi');
const router = express.Router();
const { validateRequest, validateFile, validateParams } = require('../middlewares/validationMiddleware');
const { helpRequestSchema, updateHelpRequestSchema } = require('../validations/helpRequest.validation');
const { pdfSchema } = require('../validations/core/file.validation');
const helpRequestController = require('../controllers/helpRequest.controller');
const authMiddleware = require('../middlewares/authMiddleware');

// Create a new help request
router.post(
  '/',
  validateRequest(helpRequestSchema),
  validateFile(pdfSchema),
  helpRequestController.createHelpRequest
);

// Get all help requests
router.get(
  '/',
  authMiddleware.requireAuth,
  helpRequestController.getAllHelpRequests
);

// Get help request by ID
router.get(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  helpRequestController.getHelpRequestById
);

// Update help request
router.put(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validateRequest(updateHelpRequestSchema),
  validateFile(pdfSchema),
  helpRequestController.updateHelpRequest
);

// Delete help request
router.delete(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  helpRequestController.deleteHelpRequest
);

module.exports = router;