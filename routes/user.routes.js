const express = require("express");
const UserController = require("../controllers/user.controller");
const { validateIdParam } = require("../middlewares/validators");
const { validationResult } = require("express-validator");

const router = express.Router();

router.post("/:id/suspend", validateIdParam, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, UserController.suspendUser);

router.get("/:id", validateIdParam, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, UserController.getUserProfile);

router.post("/:id/reject-profile", validateIdParam, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, UserController.rejectUserProfile);

module.exports = router;
