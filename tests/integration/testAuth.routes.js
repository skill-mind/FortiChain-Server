const express = require("express");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

const router = express.Router();

router.get(
  "/admin-only",
  authenticate,
  authorize(roles.ADMIN),
  (req, res) => res.status(200).json({ message: "Welcome Admin" })
);

router.get(
  "/user-or-admin",
  authenticate,
  authorize(roles.USER, roles.ADMIN),
  (req, res) => res.status(200).json({ message: "Welcome User or Admin" })
);

module.exports = router;
