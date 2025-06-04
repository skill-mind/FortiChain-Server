const express = require("express");
const SupportController = require("../controllers/support.controller");
const {
  validateIdParam,
  validateReplyMessage,
} = require("../middlewares/validators");
const { validationResult } = require("express-validator");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

const router = express.Router();

// 🔓 Public route to get support tickets (adjust this if you want to limit it)
router.get(
  "/",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  SupportController.getSupportTickets
);

// 🛡️ Protected: Reply to support ticket
router.post(
  "/:id/reply",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validateIdParam,
  validateReplyMessage,
  async (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty())
      return res.status(400).json({ errors: errors.array() });
    next();
  },
  SupportController.replyToSupportTicket
);

// 🛡️ Protected: Close support ticket
router.post(
  "/:id/close",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  validateIdParam,
  async (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty())
      return res.status(400).json({ errors: errors.array() });
    next();
  },
  SupportController.closeSupportTicket
);

module.exports = router;
