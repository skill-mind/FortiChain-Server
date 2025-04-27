const { HelpRequest } = require('../models');
const { helpRequestSchema } = require('../utils/validators');
const logger = require('../utils/logger');
const path = require('path');
const fs = require('fs');

// Create uploads directory if it doesn't exist
const uploadDir = path.join(__dirname, '../uploads/help-requests');
if (!fs.existsSync(uploadDir)) {
  fs.mkdirSync(uploadDir, { recursive: true });
}

exports.createHelpRequest = async (req, res) => {
  try {
    // Validate text fields
    const { error, value } = helpRequestSchema.validate(req.body);
    if (error) {
      logger.error(`Validation error: ${error.details[0].message}`);
      return res.status(400).json({ error: error.details[0].message });
    }

    const { email, subject, message } = value;
    let documentPath = null;

    // Handle file upload if present
    if (req.files && req.files.document) {
      const file = req.files.document;
      
      // Validate file type and size
      if (file.mimetype !== 'application/pdf') {
        return res.status(400).json({ error: 'Only PDF files are allowed' });
      }
      if (file.size > 5 * 1024 * 1024) {
        return res.status(400).json({ error: 'File size exceeds 5MB limit' });
      }

      // Generate unique filename
      const uniqueSuffix = Date.now() + '-' + Math.round(Math.random() * 1e9);
      const filename = uniqueSuffix + path.extname(file.name);
      documentPath = path.join(uploadDir, filename);

      // Save file
      await file.mv(documentPath);
    }

    // Create help request
    const helpRequest = await HelpRequest.create({
      email,
      subject,
      message,
      documentPath,
      status: 'open',
    });

    logger.info(`New help request created: ${helpRequest.id}`);
    res.status(201).json(helpRequest);
  } catch (error) {
    logger.error(`Error creating help request: ${error.message}`);
    res.status(500).json({ error: 'Internal server error' });
  }
};

// Keep other CRUD operations (getAll, get, update, delete) the same as before

// Get all help requests (admin only)
exports.getAllHelpRequests = async (req, res) => {
  try {
    const helpRequests = await HelpRequest.findAll({
      order: [['createdAt', 'DESC']],
    });
    res.json(helpRequests);
  } catch (error) {
    logger.error(`Error fetching help requests: ${error.message}`);
    res.status(500).json({ error: 'Internal server error' });
  }
};

// Get a single help request
exports.getHelpRequest = async (req, res) => {
  try {
    const helpRequest = await HelpRequest.findByPk(req.params.id);
    if (!helpRequest) {
      return res.status(404).json({ error: 'Help request not found' });
    }
    res.json(helpRequest);
  } catch (error) {
    logger.error(`Error fetching help request: ${error.message}`);
    res.status(500).json({ error: 'Internal server error' });
  }
};

// Update a help request (admin only)
exports.updateHelpRequest = async (req, res) => {
  try {
    const helpRequest = await HelpRequest.findByPk(req.params.id);
    if (!helpRequest) {
      return res.status(404).json({ error: 'Help request not found' });
    }

    const updates = {};
    if (req.body.status) updates.status = req.body.status;
    if (req.body.message) updates.message = req.body.message;

    await helpRequest.update(updates);
    logger.info(`Help request updated: ${helpRequest.id}`);
    res.json(helpRequest);
  } catch (error) {
    logger.error(`Error updating help request: ${error.message}`);
    res.status(500).json({ error: 'Internal server error' });
  }
};

// Delete a help request (admin only)
exports.deleteHelpRequest = async (req, res) => {
  try {
    const helpRequest = await HelpRequest.findByPk(req.params.id);
    if (!helpRequest) {
      return res.status(404).json({ error: 'Help request not found' });
    }

    await helpRequest.destroy();
    logger.info(`Help request deleted: ${req.params.id}`);
    res.status(204).send();
  } catch (error) {
    logger.error(`Error deleting help request: ${error.message}`);
    res.status(500).json({ error: 'Internal server error' });
  }
};