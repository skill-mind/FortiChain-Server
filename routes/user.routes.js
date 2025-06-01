const express = require("express");
const UserController = require("../controllers/user.controller");
const { validateIdParam } = require("../middlewares/validators");
const { validationResult } = require("express-validator");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

const router = express.Router();

// Suspend user – Only admin and super_admin can do this
router.post(
  "/:id/suspend",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validateIdParam,
  (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty())
      return res.status(400).json({ errors: errors.array() });
    next();
  },
  UserController.suspendUser
);

// Get user profile – Any authenticated user can do this
router.get(
  "/:id",
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  validateIdParam,
  (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty())
      return res.status(400).json({ errors: errors.array() });
    next();
  },
  UserController.getUserProfile
);

// Reject user profile – Only admin and super_admin
router.post(
  "/:id/reject-profile",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validateIdParam,
  (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty())
      return res.status(400).json({ errors: errors.array() });
    next();
  },
  UserController.rejectUserProfile
);

module.exports = router;
