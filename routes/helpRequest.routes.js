const express = require('express');
const router = express.Router();
const helpRequestController = require('../controllers/helpRequest.controller');

// Public routes
router.post('/', helpRequestController.createHelpRequest);

// Admin routes (no auth for now)
router.get('/', helpRequestController.getAllHelpRequests);
router.get('/:id', helpRequestController.getHelpRequest);
router.put('/:id', helpRequestController.updateHelpRequest);
router.delete('/:id', helpRequestController.deleteHelpRequest);

module.exports = router;