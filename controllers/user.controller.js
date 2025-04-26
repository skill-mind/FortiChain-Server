const User = require("../models/user.model");

// Suspend a user
exports.suspendUser = async (req, res) => {
  try {
    const user = await User.findById(req.params.id);
    if (!user) return res.status(404).json({ error: "User not found." });

    user.isSuspended = true;
    await user.save();

    res.json({ message: "User suspended successfully." });
  } catch (err) {
    res.status(500).json({ error: "Failed to suspend user." });
  }
};

// Fetch user profile
exports.getUserProfile = async (req, res) => {
  try {
    const user = await User.findById(req.params.id);
    if (!user) return res.status(404).json({ error: "User not found." });

    res.json(user);
  } catch (err) {
    res.status(500).json({ error: "Failed to fetch user profile." });
  }
};

// Reject user profile
exports.rejectUserProfile = async (req, res) => {
  try {
    const user = await User.findById(req.params.id);
    if (!user) return res.status(404).json({ error: "User not found." });

    user.profileRejected = true;
    await user.save();

    res.json({ message: "User profile rejected." });
  } catch (err) {
    res.status(500).json({ error: "Failed to reject user profile." });
  }
};
