const express = require("express");
const SupportController = require("../controllers/support.controller");
const { validateIdParam, validateReplyMessage } = require("../middlewares/validators");
const { validationResult } = require("express-validator");

const router = express.Router();

router.get("/", SupportController.getSupportTickets);

router.post("/:id/reply", validateIdParam, validateReplyMessage, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, SupportController.replyToSupportTicket);

router.post("/:id/close", validateIdParam, async (req, res, next) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
  next();
}, SupportController.closeSupportTicket);

module.exports = router;
