const request = require("supertest");
const app = require("../../server");
const { Project } = require("../../models");

describe("Project Endpoints", () => {
  // Before all tests, set up the database
  beforeAll(async () => {
    try {
      // Force sync to ensure tables are created
      await Project.sync({ force: true });
      
      // Add test data
      await Project.create({
        title: "Test Project 1",
        description: "Description for test project 1",
        status: "active"
      });
      
      await Project.create({
        title: "Test Project 2",
        description: "Description for test project 2",
        status: "active"
      });
    } catch (error) {
      console.error("Test setup failed:", error);
    }
  });

  it("should fetch all projects", async () => {
    const res = await request(app).get("/projects");
    expect(res.statusCode).toBe(200);
    expect(Array.isArray(res.body)).toBe(true);
  });

  it("should return 400 for invalid project ID on close", async () => {
    const res = await request(app).post("/projects/invalid-id/close");
    expect(res.statusCode).toBe(400);
    expect(res.body).toHaveProperty("errors");
  });
  
  // After all tests, clean up
  afterAll(async () => {
    try {
      await Project.drop();
    } catch (error) {
      console.error("Test cleanup failed:", error);
    }
  });
});