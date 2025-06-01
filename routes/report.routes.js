const express = require("express");
const router = express.Router();
const reportController = require("../controllers/report.controller");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

// 🛡️ Create a report (accessible to regular users)
router.post(
  "/",
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  reportController.createReport
);

// 🛡️ Get all reports (admin/super admin only)
router.get(
  "/",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  reportController.getReports
);

// 🛡️ Get a report by ID (admin/super admin only)
router.get(
  "/:id",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  reportController.getReportById
);

// 🛡️ Update a report (admin/super admin only)
router.put(
  "/:id",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  reportController.updateReport
);

// 🛡️ Delete a report (super admin only)
router.delete(
  "/:id",
  authenticate,
  authorize(roles.SUPER_ADMIN),
  reportController.deleteReport
);

module.exports = router;
