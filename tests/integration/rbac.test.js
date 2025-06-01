const request = require("supertest");
const express = require("express");
const jwt = require("jsonwebtoken");
const testAuthRoutes = require("./testAuth.routes");

jest.mock("jsonwebtoken");

const app = express();
app.use(express.json());
app.use(testAuthRoutes);

describe("Auth Middleware Tests", () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  test("rejects request without auth header", async () => {
    const res = await request(app).get("/admin-only");
    expect(res.status).toBe(401);
    expect(res.body.message).toBe("Unauthorized");
  });

  test("rejects request with invalid token (jwt.verify throws)", async () => {
    jwt.verify.mockImplementation(() => { throw new Error("Invalid token"); });

    const res = await request(app)
      .get("/admin-only")
      .set("Authorization", "Bearer invalid.token.here");

    expect(res.status).toBe(401);
    expect(res.body.message).toBe("Invalid token");
  });

  test("allows access for admin role", async () => {
    jwt.verify.mockReturnValue({ id: 1, role: "admin" });

    const res = await request(app)
      .get("/admin-only")
      .set("Authorization", "Bearer valid.token.here");

    expect(res.status).toBe(200);
    expect(res.body.message).toBe("Welcome Admin");
  });

  test("forbids access for user role on admin-only route", async () => {
    jwt.verify.mockReturnValue({ id: 2, role: "user" });

    const res = await request(app)
      .get("/admin-only")
      .set("Authorization", "Bearer valid.token.here");

    expect(res.status).toBe(403);
    expect(res.body.message).toBe("Forbidden: Insufficient role");
  });

  test("allows access for user role on user-or-admin route", async () => {
    jwt.verify.mockReturnValue({ id: 2, role: "user" });

    const res = await request(app)
      .get("/user-or-admin")
      .set("Authorization", "Bearer valid.token.here");

    expect(res.status).toBe(200);
    expect(res.body.message).toBe("Welcome User or Admin");
  });
});
