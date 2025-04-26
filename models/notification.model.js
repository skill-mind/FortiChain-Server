const mongoose = require('mongoose');

const notificationSchema = new mongoose.Schema({
  type: {
    type: String,
    required: true,
    enum: ['URGENT', 'FUND_RELEASE', 'BOUNTY_PAID', 'NEW_SUBMISSION']
  },
  title: {
    type: String,
    required: true
  },
  message: {
    type: String,
    required: true
  },
  status: {
    type: String,
    enum: ['READ', 'UNREAD'],
    default: 'UNREAD'
  },
  priority: {
    type: String,
    enum: ['HIGH', 'MEDIUM', 'LOW'],
    required: true
  },
  icon: {
    type: String,
    required: false
  },
  metadata: {
    type: Map,
    of: mongoose.Schema.Types.Mixed,
    default: {}
  },
  timestamp: {
    type: Date,
    default: Date.now
  },
  userId: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    required: true
  },
  actionRequired: {
    type: Boolean,
    default: false
  },
  actionUrl: {
    type: String,
    required: false
  }
}, {
  timestamps: true
});

// Index for faster queries
notificationSchema.index({ userId: 1, timestamp: -1 });
notificationSchema.index({ status: 1 });
notificationSchema.index({ type: 1 });

const Notification = mongoose.model('Notification', notificationSchema);

module.exports = Notification; 