const Report = require('../models/report.model');

// Create a new report
exports.createReport = async (req, res) => {
  try {
    const report = await Report.create(req.body); // Use Sequelize's create method
    res.status(201).json({ message: 'Report created successfully', report });
  } catch (error) {
    console.error('Error creating report:', error);
    res.status(400).json({ error: 'Failed to create report', details: error.message });
  }
};

// Get all reports
exports.getReports = async (req, res) => {
  try {
    const reports = await Report.findAll(); // Use Sequelize's findAll method
    res.status(200).json(reports);
  } catch (error) {
    console.error('Error fetching reports:', error);
    res.status(500).json({ error: 'Failed to fetch reports', details: error.message });
  }
};

// Get a single report by ID
exports.getReportById = async (req, res) => {
  try {
    const report = await Report.findByPk(req.params.id); // Use Sequelize's findByPk method
    if (!report) return res.status(404).json({ message: 'Report not found' });
    res.status(200).json(report);
  } catch (error) {
    console.error('Error fetching report by ID:', error);
    res.status(500).json({ error: 'Failed to fetch report', details: error.message });
  }
};

// Update a report
exports.updateReport = async (req, res) => {
  try {
    const report = await Report.findByPk(req.params.id); // Find the report by primary key
    if (!report) return res.status(404).json({ message: 'Report not found' });

    await report.update(req.body); // Update the report with the request body
    res.status(200).json({ message: 'Report updated successfully', report });
  } catch (error) {
    console.error('Error updating report:', error);
    res.status(400).json({ error: 'Failed to update report', details: error.message });
  }
};

// Delete a report
exports.deleteReport = async (req, res) => {
  try {
    const report = await Report.findByPk(req.params.id); // Find the report by primary key
    if (!report) return res.status(404).json({ message: 'Report not found' });

    await report.destroy(); // Delete the report
    res.status(200).json({ message: 'Report deleted successfully' });
  } catch (error) {
    console.error('Error deleting report:', error);
    res.status(500).json({ error: 'Failed to delete report', details: error.message });
  }
};