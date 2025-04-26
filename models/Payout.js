const mongoose = require('mongoose');

const payoutSchema = new mongoose.Schema({
  userId: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    required: true,
  },
  amount: {
    type: Number,
    required: true,
  },
  status: {
    type: String,
    enum: ['pending', 'completed', 'failed'],
    default: 'pending',
  },
  method: {
    type: String,
    required: true,   
  },
  payoutDate: {
    type: Date,
  },
  notes: {
    type: String,
  },
}, { timestamps: true });


payoutSchema.index({ userId: 1 });

module.exports = mongoose.model('Payout', payoutSchema);
