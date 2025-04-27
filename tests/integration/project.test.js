const request = require("supertest");
const app = require("../../server");
const { Project } = require("../../models");

describe("Project Endpoints", () => {
  let project1, project2;

  // Before all tests, set up the database
  beforeAll(async () => {
    try {
      // Force sync to ensure tables are created
      await Project.sync({ force: true });

      // Add test data with all required fields
      project1 = await Project.create({
        name: "My Project",
        description: "This is a test project",
        category: "Blockchain",
        smartContractAddress: "0x1234567890abcdef1234567890abcdef12345678",
        contactInfo: "test@example.com",
        supportingDocument: "uploads/1745717698544-fastDeliv PRD.pdf",
        projectLogo: "uploads/1745717698547-GlXKyQIWwAAAMgg.jpeg",
        repositoryHost: "Github",
        repositoryName: "fortichain",
        repositoryLink: "https://github.com/fortichain",
        token: "BTC",
        escrowBalance: 0,
        walletBalance: 0,
        allocatedBounty: 1000,
        bountyCurrency: "USD",
        dateOfExpiry: new Date("2025-12-31"),
        autoTopUp: false,
      });

      project2 = await Project.create({
        name: "Test Project 2",
        description: "This is another test project",
        category: "NFT",
        smartContractAddress: "0xabcdef1234567890abcdef1234567890abcdef12",
        contactInfo: "test2@example.com",
        repositoryHost: "GitLab",
        repositoryName: "test-repo-2",
        repositoryLink: "https://gitlab.com/test-repo-2",
        token: "ETH",
        escrowBalance: 0,
        walletBalance: 0,
        allocatedBounty: 500,
        bountyCurrency: "ETH",
        dateOfExpiry: new Date("2026-12-31"),
        autoTopUp: false,
      });
    } catch (error) {
      console.error("Test setup failed:", error);
    }
  });

  it("should fetch all projects", async () => {
    const res = await request(app).get("/projects");
    expect(res.statusCode).toBe(200);
    expect(Array.isArray(res.body)).toBe(true);
    expect(res.body).toHaveLength(2); // Ensure both projects are returned

    // Validate the structure of the first project
    const project = res.body[0];
    expect(project).toHaveProperty("id", project1.id);
    expect(project).toHaveProperty("name", "My Project");
    expect(project).toHaveProperty("description", "This is a test project");
    expect(project).toHaveProperty("category", "Blockchain");
    expect(project).toHaveProperty("smartContractAddress", "0x1234567890abcdef1234567890abcdef12345678");
    expect(project).toHaveProperty("contactInfo", "test@example.com");
    expect(project).toHaveProperty("supportingDocument", "uploads/1745717698544-fastDeliv PRD.pdf");
    expect(project).toHaveProperty("projectLogo", "uploads/1745717698547-GlXKyQIWwAAAMgg.jpeg");
    expect(project).toHaveProperty("repositoryHost", "Github");
    expect(project).toHaveProperty("repositoryName", "fortichain");
    expect(project).toHaveProperty("repositoryLink", "https://github.com/fortichain");
    expect(project).toHaveProperty("token", "BTC");
    expect(project).toHaveProperty("escrowBalance", 0);
    expect(project).toHaveProperty("walletBalance", 0);
    expect(project).toHaveProperty("allocatedBounty", 1000);
    expect(project).toHaveProperty("bountyCurrency", "USD");
    expect(project).toHaveProperty("dateOfExpiry", "2025-12-31T00:00:00.000Z");
    expect(project).toHaveProperty("autoTopUp", false);
  });

  it("should fetch a single project by ID", async () => {
    const res = await request(app).get(`/projects/${project1.id}`);
    expect(res.statusCode).toBe(200);
    expect(res.body).toHaveProperty("id", project1.id);
    expect(res.body).toHaveProperty("name", "My Project");
    expect(res.body).toHaveProperty("description", "This is a test project");
    expect(res.body).toHaveProperty("category", "Blockchain");
    expect(res.body).toHaveProperty("smartContractAddress", "0x1234567890abcdef1234567890abcdef12345678");
    expect(res.body).toHaveProperty("contactInfo", "test@example.com");
    expect(res.body).toHaveProperty("supportingDocument", "uploads/1745717698544-fastDeliv PRD.pdf");
    expect(res.body).toHaveProperty("projectLogo", "uploads/1745717698547-GlXKyQIWwAAAMgg.jpeg");
    expect(res.body).toHaveProperty("repositoryHost", "Github");
    expect(res.body).toHaveProperty("repositoryName", "fortichain");
    expect(res.body).toHaveProperty("repositoryLink", "https://github.com/fortichain");
    expect(res.body).toHaveProperty("token", "BTC");
    expect(res.body).toHaveProperty("escrowBalance", 0);
    expect(res.body).toHaveProperty("walletBalance", 0);
    expect(res.body).toHaveProperty("allocatedBounty", 1000);
    expect(res.body).toHaveProperty("bountyCurrency", "USD");
    expect(res.body).toHaveProperty("dateOfExpiry", "2025-12-31T00:00:00.000Z");
    expect(res.body).toHaveProperty("autoTopUp", false);
  });

  it("should create a new project", async () => {
    const newProject = {
      name: "Test Project 3",
      description: "This is a new test project",
      category: "Gaming",
      smartContractAddress: "0xabcdefabcdefabcdefabcdefabcdefabcdef",
      contactInfo: "test3@example.com",
      supportingDocument: "uploads/1745717698548-newDocument.pdf",
      projectLogo: "uploads/1745717698549-newLogo.jpeg",
      repositoryHost: "Bitbucket",
      repositoryName: "test-repo-3",
      repositoryLink: "https://bitbucket.org/test-repo-3",
      token: "BTC",
      escrowBalance: 0,
      walletBalance: 0,
      allocatedBounty: 200,
      bountyCurrency: "BTC",
      dateOfExpiry: "2027-12-31",
      autoTopUp: false,
    };

    const res = await request(app).post("/projects").send(newProject);
    expect(res.statusCode).toBe(201);
    expect(res.body).toHaveProperty("id");
    expect(res.body).toHaveProperty("name", "Test Project 3");
    expect(res.body).toHaveProperty("description", "This is a new test project");
    expect(res.body).toHaveProperty("category", "Gaming");
    expect(res.body).toHaveProperty("smartContractAddress", "0xabcdefabcdefabcdefabcdefabcdefabcdef");
    expect(res.body).toHaveProperty("contactInfo", "test3@example.com");
    expect(res.body).toHaveProperty("supportingDocument", "uploads/1745717698548-newDocument.pdf");
    expect(res.body).toHaveProperty("projectLogo", "uploads/1745717698549-newLogo.jpeg");
    expect(res.body).toHaveProperty("repositoryHost", "Bitbucket");
    expect(res.body).toHaveProperty("repositoryName", "test-repo-3");
    expect(res.body).toHaveProperty("repositoryLink", "https://bitbucket.org/test-repo-3");
    expect(res.body).toHaveProperty("token", "BTC");
    expect(res.body).toHaveProperty("escrowBalance", 0);
    expect(res.body).toHaveProperty("walletBalance", 0);
    expect(res.body).toHaveProperty("allocatedBounty", 200);
    expect(res.body).toHaveProperty("bountyCurrency", "BTC");
    expect(res.body).toHaveProperty("dateOfExpiry", "2027-12-31T00:00:00.000Z");
    expect(res.body).toHaveProperty("autoTopUp", false);
  });

  it("should update a project", async () => {
    const updatedData = {
      name: "Updated Project 1",
      description: "Updated description for Project 1",
      allocatedBounty: 600,
      autoTopUp: true,
    };

    const res = await request(app).put(`/projects/${project1.id}`).send(updatedData);
    expect(res.statusCode).toBe(200);
    expect(res.body).toHaveProperty("id", project1.id);
    expect(res.body).toHaveProperty("name", "Updated Project 1");
    expect(res.body).toHaveProperty("description", "Updated description for Project 1");
    expect(res.body).toHaveProperty("allocatedBounty", 600);
    expect(res.body).toHaveProperty("autoTopUp", true);
  });

  it("should delete a project", async () => {
    const res = await request(app).delete(`/projects/${project2.id}`);
    expect(res.statusCode).toBe(204);

    // Verify the project is deleted
    const fetchRes = await request(app).get(`/projects/${project2.id}`);
    expect(fetchRes.statusCode).toBe(404);
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