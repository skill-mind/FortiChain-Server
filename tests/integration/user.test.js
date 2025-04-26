const request = require("supertest");
const app = require("../../server");

describe("User Endpoints", () => {
  it("should return 400 for invalid user ID on suspend", async () => {
    const res = await request(app).post("/users/invalid-id/suspend");
    expect(res.statusCode).toBe(400);
    expect(res.body).toHaveProperty("errors");
  });

  it("should return 400 for invalid user ID on profile fetch", async () => {
    const res = await request(app).get("/users/invalid-id");
    expect(res.statusCode).toBe(400);
    expect(res.body).toHaveProperty("errors");
  });
});
