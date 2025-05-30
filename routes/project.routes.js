const express = require('express');
const router = express.Router();
const { validateRequest, validateFile, validateParams } = require('../middlewares/validationMiddleware');
const { projectSchema, updateProjectSchema } = require('../validations/project.validation');
const { fileSchema } = require('../validations/core/file.validation');
const projectController = require('../controllers/project.controller');
const authMiddleware = require('../middlewares/authMiddleware');

// Create a new project
router.post(
  '/',
  authMiddleware.requireAuth,
  validateRequest(projectSchema),
  validateFile(fileSchema),
  projectController.createProject
);

// Get all projects
router.get('/', projectController.getAllProjects);

// Get project by ID
router.get(
  '/:id',
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  projectController.getProjectById
);

// Update project
router.put(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  validateRequest(updateProjectSchema),
  validateFile(fileSchema),
  projectController.updateProject
);

// Delete project
router.delete(
  '/:id',
  authMiddleware.requireAuth,
  validateParams(Joi.object({
    id: Joi.string().uuid().required()
  })),
  projectController.deleteProject
);

module.exports = router;