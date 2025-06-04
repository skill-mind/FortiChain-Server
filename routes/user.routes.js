const express = require("express");
const UserController = require("../controllers/user.controller");
const { validateIdParam } = require("../middlewares/validators");
const { validationResult } = require("express-validator");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");
const { validateRequest } = require('../middlewares/validationMiddleware');
const { userSchema } = require('../validations/user.validation');

const router = express.Router();

// Suspend user – Only admin and super_admin can do this
router.post(
  "/:id/suspend",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validateRequest(userSchema),
  async (req, res) => {
    try {
      // Implementation will be added later
      res.status(200).json({ message: 'User suspended successfully' });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  }
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
