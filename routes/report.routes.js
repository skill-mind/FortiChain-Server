const express = require('express');
const Joi = require('joi');
const router = express.Router();
const { validateRequest, validateFile, validateParams } = require('../middlewares/validationMiddleware');
const { reportSchema, updateReportSchema } = require('../validations/report.validation');
const { pdfSchema } = require('../validations/core/file.validation');
const reportController = require('../controllers/report.controller');
const authMiddleware = require('../middlewares/authMiddleware');

// Create a new report
router.post(
  '/',
  authMiddleware.requireAuth,
  validateRequest(reportSchema),
  validateFile(pdfSchema),
  reportController.createReport
);

// Get all reports
router.get(
  '/',
  authMiddleware.requireAuth,
  reportController.getAllReports
);

// Get report by ID
router.get(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  reportController.getReportById
);

// Update report
router.put(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validateRequest(updateReportSchema),
  validateFile(pdfSchema),
  reportController.updateReport
);

// Delete report
router.delete(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  reportController.deleteReport
);

module.exports = router;