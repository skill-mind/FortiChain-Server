const Notification = require("../models/notification.model");
const logger = require("../utils/logger");

// Create a new notification
exports.createNotification = async (req, res) => {
  try {
    const notification = new Notification({
      ...req.body,
      userId: req.user._id, // Assuming user is authenticated
    });

    await notification.save();

    // Here you would typically emit a websocket event for real-time updates
    // socketIO.emit('newNotification', notification);

    logger.info(`New notification created: ${notification._id}`);
    res.status(201).json(notification);
  } catch (error) {
    logger.error("Error creating notification:", error);
    res.status(400).json({ message: error.message });
  }
};

// Get all notifications for a user
exports.getNotifications = async (req, res) => {
  try {
    const { page = 1, limit = 10, status, type } = req.query;
    const query = { userId: req.user._id };

    // Add filters if provided
    if (status) query.status = status;
    if (type) query.type = type;

    const notifications = await Notification.find(query)
      .sort({ timestamp: -1 })
      .limit(limit * 1)
      .skip((page - 1) * limit)
      .exec();

    const count = await Notification.countDocuments(query);

    res.json({
      notifications,
      totalPages: Math.ceil(count / limit),
      currentPage: page,
      totalNotifications: count,
    });
  } catch (error) {
    logger.error("Error fetching notifications:", error);
    res.status(500).json({ message: error.message });
  }
};

// Get unread count
exports.getUnreadCount = async (req, res) => {
  try {
    const count = await Notification.countDocuments({
      userId: req.user._id,
      status: "UNREAD",
    });
    res.json({ unreadCount: count });
  } catch (error) {
    logger.error("Error getting unread count:", error);
    res.status(500).json({ message: error.message });
  }
};

// Mark notification as read
exports.markAsRead = async (req, res) => {
  try {
    const notification = await Notification.findOneAndUpdate(
      { _id: req.params.id, userId: req.user._id },
      { status: "READ" },
      { new: true }
    );

    if (!notification) {
      return res.status(404).json({ message: "Notification not found" });
    }

    res.json(notification);
  } catch (error) {
    logger.error("Error marking notification as read:", error);
    res.status(500).json({ message: error.message });
  }
};

// Mark all as read
exports.markAllAsRead = async (req, res) => {
  try {
    await Notification.updateMany(
      { userId: req.user._id, status: "UNREAD" },
      { status: "READ" }
    );

    res.json({ message: "All notifications marked as read" });
  } catch (error) {
    logger.error("Error marking all notifications as read:", error);
    res.status(500).json({ message: error.message });
  }
};

// Delete a notification
exports.deleteNotification = async (req, res) => {
  try {
    const notification = await Notification.findOneAndDelete({
      _id: req.params.id,
      userId: req.user._id,
    });

    if (!notification) {
      return res.status(404).json({ message: "Notification not found" });
    }

    res.json({ message: "Notification deleted successfully" });
  } catch (error) {
    logger.error("Error deleting notification:", error);
    res.status(500).json({ message: error.message });
  }
};

// Delete all read notifications
exports.deleteAllRead = async (req, res) => {
  try {
    const result = await Notification.deleteMany({
      userId: req.user._id,
      status: "READ",
    });

    res.json({
      message: "All read notifications deleted successfully",
      count: result.deletedCount,
    });
  } catch (error) {
    logger.error("Error deleting read notifications:", error);
    res.status(500).json({ message: error.message });
  }
};
