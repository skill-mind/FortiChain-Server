const request = require("supertest");
const app = require("../../server");

describe("Project Endpoints", () => {
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
});