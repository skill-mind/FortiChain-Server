jest.mock("../middlewares/auth", () => ({
  authenticate: (req, res, next) => {
    req.user = { id: 1, role: "admin" }; // Fake logged-in user
    next();
  },
  authorize:
    (...allowedRoles) =>
    (req, res, next) => {
      next(); // Skip role checks in tests
    },
}));
