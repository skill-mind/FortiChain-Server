const { Notification } = require('../models');

// Create Notification
exports.createNotification = async (req, res) => {
  try {
    const notification = await Notification.create(req.body);
    res.status(201).json(notification);
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
};

// Get All Notifications (optionally filter by userId)
exports.getNotifications = async (req, res) => {
  try {
    const where = {};
    if (req.query.userId) where.userId = req.query.userId;
    const notifications = await Notification.findAll({ where, order: [['createdAt', 'DESC']] });
    res.json(notifications);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
};

// Get Notification by ID
exports.getNotificationById = async (req, res) => {
  try {
    const notification = await Notification.findByPk(req.params.id);
    if (!notification) return res.status(404).json({ error: 'Notification not found' });
    res.json(notification);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
};

// Update Notification (e.g., mark as read)
exports.updateNotification = async (req, res) => {
  try {
    const notification = await Notification.findByPk(req.params.id);
    if (!notification) return res.status(404).json({ error: 'Notification not found' });
    await notification.update(req.body);
    res.json(notification);
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
};

// Delete Notification
exports.deleteNotification = async (req, res) => {
  try {
    const notification = await Notification.findByPk(req.params.id);
    if (!notification) return res.status(404).json({ error: 'Notification not found' });
    await notification.destroy();
    res.status(204).send();
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
};



