const { Project } = require("../models");

// Fetch all projects
exports.getAllProjects = async (req, res) => {
  try {
    const projects = await Project.findAll();
    res.json(projects);
  } catch (err) {
    console.error("Failed to fetch projects:", err);
    res.status(500).json({ error: "Failed to fetch projects." });
  }
};

// Close a project
exports.closeProject = async (req, res) => {
  try {
    const project = await Project.findByPk(req.params.id);
    if (!project) return res.status(404).json({ error: "Project not found." });

    project.status = "closed";
    await project.save();

    res.json({ message: "Project closed successfully." });
  } catch (err) {
    console.error("Failed to close project:", err);
    res.status(500).json({ error: "Failed to close project." });
  }
};

// View single project by ID
exports.getProjectById = async (req, res) => {
  try {
    const project = await Project.findByPk(req.params.id);
    if (!project) return res.status(404).json({ error: "Project not found." });

    res.json(project);
  } catch (err) {
    console.error("Failed to fetch project:", err);
    res.status(500).json({ error: "Failed to fetch project." });
  }
};
