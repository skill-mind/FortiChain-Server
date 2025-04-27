const Project = require('../models/project.model');

// Create a Project
exports.createProject = async (req, res) => {
  try {
    const projectData = req.body;

    // Handle file uploads
    if (req.files) {
      if (req.files.supportingDocument) {
        projectData.supportingDocument = req.files.supportingDocument[0].path;
      }
      if (req.files.projectLogo) {
        projectData.projectLogo = req.files.projectLogo[0].path;
      }
    }

    // Validate allocatedBounty
    if (projectData.allocatedBounty === undefined || projectData.allocatedBounty === null) {
      return res.status(400).json({ message: "Allocated bounty is required" });
    }
    if (isNaN(projectData.allocatedBounty) || projectData.allocatedBounty < 0) {
      return res.status(400).json({ message: "Allocated bounty must be a non-negative number" });
    }

    const project = await Project.create(projectData);
    res.status(201).json(project);
  } catch (error) {
    res.status(500).json({ message: "Error creating project", error });
  }
};

// Get All Projects
exports.getAllProjects = async (req, res) => {
  try {
    const projects = await Project.findAll();
    res.status(200).json(projects);
  } catch (error) {
    res.status(500).json({ message: 'Error fetching projects', error });
  }
};

// Get a Single Project by ID
exports.getProjectById = async (req, res) => {
  try {
    const project = await Project.findByPk(req.params.id);
    if (!project) {
      return res.status(404).json({ message: 'Project not found' });
    }
    res.status(200).json(project);
  } catch (error) {
    res.status(500).json({ message: 'Error fetching project', error });
  }
};

// Update a Project
exports.updateProject = async (req, res) => {
  try {
    const project = await Project.findByPk(req.params.id);
    if (!project) {
      return res.status(404).json({ message: 'Project not found' });
    }

    const updatedData = req.body;

    // Handle file uploads
    if (req.files) {
      if (req.files.supportingDocument) {
        updatedData.supportingDocument = req.files.supportingDocument[0].path;
      }
      if (req.files.projectLogo) {
        updatedData.projectLogo = req.files.projectLogo[0].path;
      }
    }

    await project.update(updatedData);
    res.status(200).json(project);
  } catch (error) {
    res.status(500).json({ message: 'Error updating project', error });
  }
};

// Delete a Project
exports.deleteProject = async (req, res) => {
  try {
    const project = await Project.findByPk(req.params.id);
    if (!project) {
      return res.status(404).json({ message: 'Project not found' });
    }
    await project.destroy();
    res.status(204).send();
  } catch (error) {
    res.status(500).json({ message: 'Error deleting project', error });
  }
};