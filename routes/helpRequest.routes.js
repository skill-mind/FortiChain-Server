const express = require("express");
const router = express.Router();
const helpRequestController = require("../controllers/helpRequest.controller");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

// Public route: create a help request
router.post("/", helpRequestController.createHelpRequest);

// Admin routes: protected
router.get(
  "/",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  helpRequestController.getAllHelpRequests
);

router.get(
  "/:id",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  helpRequestController.getHelpRequest
);

router.put(
  "/:id",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  helpRequestController.updateHelpRequest
);

router.delete(
  "/:id",
  authenticate,
  authorize(roles.SUPER_ADMIN),
  helpRequestController.deleteHelpRequest
);

module.exports = router;
