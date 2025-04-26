const express = require("express");
const ProjectController = require("../controllers/project.controller");
const { validateIdParam } = require("../middlewares/validators");
const { validationResult } = require("express-validator");

const router = express.Router();

router.get("/", ProjectController.getAllProjects);

router.post("/:id/close", validateIdParam, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, ProjectController.closeProject);

router.get("/:id", validateIdParam, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, ProjectController.getProjectById);

module.exports = router;
