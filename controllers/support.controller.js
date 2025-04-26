const SupportTicket = require("../models/supportTicket.model");

// Get all support tickets
exports.getSupportTickets = async (req, res) => {
  try {
    const tickets = await SupportTicket.find();
    res.json(tickets);
  } catch (err) {
    res.status(500).json({ error: "Failed to fetch support tickets." });
  }
};

// Reply to a support ticket
exports.replyToSupportTicket = async (req, res) => {
  try {
    const ticket = await SupportTicket.findById(req.params.id);
    if (!ticket) return res.status(404).json({ error: "Ticket not found." });

    ticket.replies.push({ message: req.body.message, repliedAt: new Date() });
    await ticket.save();

    res.json({ message: "Reply added successfully." });
  } catch (err) {
    res.status(500).json({ error: "Failed to reply to ticket." });
  }
};

// Close a support ticket
exports.closeSupportTicket = async (req, res) => {
  try {
    const ticket = await SupportTicket.findById(req.params.id);
    if (!ticket) return res.status(404).json({ error: "Ticket not found." });

    ticket.status = "closed";
    await ticket.save();

    res.json({ message: "Support ticket closed successfully." });
  } catch (err) {
    res.status(500).json({ error: "Failed to close ticket." });
  }
};
