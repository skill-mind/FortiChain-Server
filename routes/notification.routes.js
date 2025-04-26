const express = require("express");
const router = express.Router();
const notificationController = require("../controllers/notification.controller");
const { authenticate } = require("../middleware/auth");
const { validateNotification } = require("../middleware/validators");

// Get all notifications with pagination and filters
router.get("/", authenticate, notificationController.getNotifications);

// Get unread notification count
router.get(
  "/unread-count",
  authenticate,
  notificationController.getUnreadCount
);

// Create a new notification
router.post(
  "/",
  authenticate,
  validateNotification,
  notificationController.createNotification
);

// Mark a notification as read
router.patch("/:id/read", authenticate, notificationController.markAsRead);

// Mark all notifications as read
router.patch(
  "/mark-all-read",
  authenticate,
  notificationController.markAllAsRead
);

// Delete a specific notification
router.delete("/:id", authenticate, notificationController.deleteNotification);

// Delete all read notifications
router.delete(
  "/clear-read",
  authenticate,
  notificationController.deleteAllRead
);

// Create a new urgent notification
router.post(
  "/urgent",
  authenticate,
  validateNotification,
  notificationController.createUrgentNotification
);

// Get notifications with filters
router.get(
  "/filter",
  authenticate,
  notificationController.getFilteredNotifications
);

module.exports = router;
