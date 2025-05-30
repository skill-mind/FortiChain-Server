const express = require("express");
const UserController = require("../controllers/user.controller");
const { validateRequest, validateParams } = require("../middlewares/validationMiddleware");
const { user, auth, profile, settings } = require("../middlewares/validators");
const authMiddleware = require("../middlewares/authMiddleware");

const router = express.Router();

// Authentication routes
router.post("/register", validateRequest(auth.register), UserController.register);
router.post("/login", validateRequest(auth.login), UserController.login);
router.post("/reset-password", validateRequest(auth.resetPassword), UserController.resetPassword);
router.post("/change-password", authMiddleware.requireAuth, validateRequest(auth.changePassword), UserController.changePassword);

// User management routes
router.get("/:id", validateParams({ id: user.id }), UserController.getUserById);
router.put("/:id", authMiddleware.requireAuth, validateParams({ id: user.id }), validateRequest(user), UserController.updateUser);
router.delete("/:id", authMiddleware.requireAuth, validateParams({ id: user.id }), UserController.deleteUser);
router.post("/:id/suspend", authMiddleware.requireAuth, validateParams({ id: user.id }), UserController.suspendUser);
router.post("/:id/activate", authMiddleware.requireAuth, validateParams({ id: user.id }), UserController.activateUser);

// Profile routes
router.get("/:id/profile", validateParams({ id: user.id }), UserController.getProfile);
router.put("/:id/profile", authMiddleware.requireAuth, validateParams({ id: user.id }), validateRequest(profile.update), UserController.updateProfile);

// Settings routes
router.get("/:id/settings", authMiddleware.requireAuth, validateParams({ id: user.id }), UserController.getSettings);
router.put("/:id/settings", authMiddleware.requireAuth, validateParams({ id: user.id }), validateRequest(settings.update), UserController.updateSettings);

module.exports = router;
