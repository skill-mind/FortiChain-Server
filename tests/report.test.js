const request = require('supertest');
const app = require('../server');
const { sequelize } = require('../models');
const Report = require('../models/report.model');

describe('Report API Endpoints', () => {
  beforeAll(async () => {
    await sequelize.sync({ force: true }); // Sync the database before tests
  });

  afterAll(async () => {
    await sequelize.close(); // Close the database connection after tests
  });

  beforeEach(async () => {
    await Report.destroy({ where: {}, truncate: true }); // Clear the Reports table before each test
  });

  test('POST /api/reports - Create a new report', async () => {
    const response = await request(app)
      .post('/api/reports')
      .send({
        projectName: 'InheritX',
        projectStatus: 'Ongoing',
        submittedBy: 'John Doe',
        vulnerabilityTitle: 'SQL Injection',
        description: "SQL injection vulnerability in the 'id' parameter.",
        severity: 'Critical',
        proofOfConcept: ['http://example.com/poc1.png', 'http://example.com/poc2.png'],
        status: 'pending',
      })
      .expect(201);

    expect(response.body).toHaveProperty('message', 'Report created successfully');
    expect(response.body.report).toHaveProperty('id');
    expect(response.body.report).toHaveProperty('projectName', 'InheritX');
  });

  test('GET /api/reports - Fetch all reports', async () => {
    await Report.create({
      projectName: 'InheritX',
      projectStatus: 'Ongoing',
      submittedBy: 'John Doe',
      vulnerabilityTitle: 'SQL Injection',
      description: "SQL injection vulnerability in the 'id' parameter.",
      severity: 'Critical',
      status: 'pending',
    });

    const response = await request(app).get('/api/reports').expect(200);

    expect(Array.isArray(response.body)).toBe(true);
    expect(response.body.length).toBe(1);
    expect(response.body[0]).toHaveProperty('projectName', 'InheritX');
  });

  test('GET /api/reports/:id - Fetch a single report by ID', async () => {
    const report = await Report.create({
      projectName: 'InheritX',
      projectStatus: 'Ongoing',
      submittedBy: 'John Doe',
      vulnerabilityTitle: 'SQL Injection',
      description: "SQL injection vulnerability in the 'id' parameter.",
      severity: 'Critical',
      status: 'pending',
    });

    const response = await request(app).get(`/api/reports/${report.id}`).expect(200);

    expect(response.body).toHaveProperty('id', report.id);
    expect(response.body).toHaveProperty('projectName', 'InheritX');
  });

  test('PUT /api/reports/:id - Update a report', async () => {
    const report = await Report.create({
      projectName: 'InheritX',
      projectStatus: 'Ongoing',
      submittedBy: 'John Doe',
      vulnerabilityTitle: 'SQL Injection',
      description: "SQL injection vulnerability in the 'id' parameter.",
      severity: 'Critical',
      status: 'pending',
    });

    const response = await request(app)
      .put(`/api/reports/${report.id}`)
      .send({
        projectStatus: 'Closed',
        severity: 'High',
        description: 'Updated description',
      })
      .expect(200);

    expect(response.body).toHaveProperty('message', 'Report updated successfully');
    expect(response.body.report).toHaveProperty('projectStatus', 'Closed');
    expect(response.body.report).toHaveProperty('severity', 'High');
  });

  test('DELETE /api/reports/:id - Delete a report', async () => {
    const report = await Report.create({
      projectName: 'InheritX',
      projectStatus: 'Ongoing',
      submittedBy: 'John Doe',
      vulnerabilityTitle: 'SQL Injection',
      description: "SQL injection vulnerability in the 'id' parameter.",
      severity: 'Critical',
      status: 'pending',
    });

    const response = await request(app).delete(`/api/reports/${report.id}`).expect(200);

    expect(response.body).toHaveProperty('message', 'Report deleted successfully');

    await request(app).get(`/api/reports/${report.id}`).expect(404); // Verify the report no longer exists
  });
});