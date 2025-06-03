const express = require('express');
const router = express.Router();
const reportController = require('../controllers/report.controller'); // Ensure this path is correct

// Define routes
router.post('/', reportController.createReport); // Create a report
router.get('/', reportController.getReports); // Get all reports
router.get('/:id', reportController.getReportById); // Get a report by ID
router.put('/:id', reportController.updateReport); // Update a report
router.delete('/:id', reportController.deleteReport); // Delete a report

module.exports = router;