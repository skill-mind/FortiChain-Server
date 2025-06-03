const express = require("express");
const router = express.Router();
const tipController = require("../controllers/tips.controller");
const { check } = require("express-validator");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

const tipValidationRules = [
  check("title").isString().isLength({ min: 3, max: 100 }),
  check("content").isString().isLength({ min: 10, max: 5000 }),
  check("category").isString().notEmpty(),
];

// Public routes
router.get("/", tipController.getTips);
router.get("/:id", tipController.getTip);

// Protected routes
router.post(
  "/",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  tipValidationRules,
  tipController.createTip
);

router.put(
  "/:id",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  tipValidationRules,
  tipController.updateTip
);

router.delete(
  "/:id",
  authenticate,
  authorize(roles.SUPER_ADMIN),
  tipController.deleteTip
);

router.patch(
  "/:id/best-model",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  check("isBestModel").isBoolean(),
  tipController.markAsBestModel
);

module.exports = router;
