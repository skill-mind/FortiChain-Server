const request = require("supertest");
const app = require("../../server");

describe("Support Endpoints", () => {
  it("should return 400 for invalid ticket ID on reply", async () => {
    const res = await request(app)
      .post("/support/invalid-id/reply")
      .send({ message: "This is a reply." });
    expect(res.statusCode).toBe(400);
    expect(res.body).toHaveProperty("errors");
  });

  it("should return 400 if message is missing", async () => {
    const res = await request(app)
      .post("/support/507f1f77bcf86cd799439011/reply") 
      .send({});
    expect(res.statusCode).toBe(400);
    expect(res.body).toHaveProperty("errors");
  });
});
