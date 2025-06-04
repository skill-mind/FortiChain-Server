const express = require("express");
const jwt = require("jsonwebtoken");
const { roles } = require("../config/roles");

const router = express.Router();

router.post("/login", (req, res) => {
  const { id, role } = req.body;

  if (!Object.values(roles).includes(role)) {
    return res.status(400).json({ message: "Invalid role" });
  }

  const token = jwt.sign({ id, role }, process.env.JWT_SECRET || "testsecret", {
    expiresIn: "1h",
  });

  res.json({ token, role });
});

module.exports = router;