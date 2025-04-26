const Payout = require('../models/Payout');
exports.createPayout = async (req, res) => {
  try {
    const payout = await Payout.create(req.body);
    res.status(201).json(payout);
  } catch (error) {
    res.status(400).json({ message: error.message });
  }
};

exports.getPayouts = async (req, res) => {
  try {
    const payouts = await Payout.find().populate('userId', 'name email');
    res.json(payouts);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
};

exports.getPayoutById = async (req, res) => {
  try {
    const payout = await Payout.findById(req.params.id).populate('userId', 'name email');
    if (!payout) return res.status(404).json({ message: 'Payout not found' });
    res.json(payout);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
};

exports.updatePayout = async (req, res) => {
  try {
    const payout = await Payout.findByIdAndUpdate(req.params.id, req.body, { new: true });
    if (!payout) return res.status(404).json({ message: 'Payout not found' });
    res.json(payout);
  } catch (error) {
    res.status(400).json({ message: error.message });
  }
};


exports.deletePayout = async (req, res) => {
  try {
    const payout = await Payout.findByIdAndDelete(req.params.id);
    if (!payout) return res.status(404).json({ message: 'Payout not found' });
    res.json({ message: 'Payout deleted' });
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
};
