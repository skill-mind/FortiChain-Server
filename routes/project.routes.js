const express = require("express");
const { body, param } = require("express-validator");
const multer = require("multer")
const path = require("path");
const {
  createProject,
  getAllProjects,
  getProjectById,
  updateProject,
  deleteProject,
} = require("../controllers/project.controller");
const handleValidationErrors = require("../middlewares/handleValidationErr");
const router = express.Router();

// Multer configuration for file uploads
const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    cb(null, "uploads/"); // Directory where files will be stored
  },
  filename: (req, file, cb) => {
    cb(null, `${Date.now()}-${file.originalname}`);
  },
});
const upload = multer({
  storage,
  fileFilter: (req, file, cb) => {
    const allowedTypes = ["image/jpeg", "image/png", "application/pdf"];
    if (allowedTypes.includes(file.mimetype)) {
      cb(null, true);
    } else {
      cb(new Error("Invalid file type. Only JPEG, PNG, and PDF are allowed."));
    }
  },
});

// Create Project
router.post(
  "/",
  upload.fields([
    { name: "supportingDocument", maxCount: 1 },
    { name: "projectLogo", maxCount: 1 },
  ]),
  [
    body("name").notEmpty().withMessage("Project name is required"),
    body("description").notEmpty().withMessage("Description is required"),
    body("category").notEmpty().withMessage("Category is required"),
    body("smartContractAddress")
      .matches(/^0x[a-fA-F0-9]{40}$/)
      .withMessage("Invalid Ethereum address format"),
    body("contactInfo").isEmail().withMessage("Invalid email format"),
    body("repositoryHost").notEmpty().withMessage("Repository host is required"),
    body("repositoryName").notEmpty().withMessage("Repository name is required"),
    body("repositoryLink")
      .isURL()
      .withMessage("Repository link must be a valid URL"),
    body("token").notEmpty().withMessage("Token type is required"),
    body("allocatedBounty")
      .isFloat({ min: 0 })
      .withMessage("Allocated bounty must be non-negative"),
    body("bountyCurrency").notEmpty().withMessage("Bounty currency is required"),
    body("dateOfExpiry")
      .isISO8601()
      .toDate()
      .withMessage("Date of expiry must be a valid date"),
  ],
  handleValidationErrors,
  createProject
);

// Read All Projects
router.get("/", getAllProjects);

// Read a Single Project
router.get(
  "/:id",
  [param("id").isInt().withMessage("Project ID must be an integer")],
  handleValidationErrors,
  getProjectById
);

// Update Project
router.put(
  "/:id",
  upload.fields([
    { name: "supportingDocument", maxCount: 1 },
    { name: "projectLogo", maxCount: 1 },
  ]),
  [
    param("id").isInt().withMessage("Project ID must be an integer"),
    body("name")
      .optional()
      .notEmpty()
      .withMessage("Project name cannot be empty"),
    body("description")
      .optional()
      .notEmpty()
      .withMessage("Description cannot be empty"),
    body("category")
      .optional()
      .notEmpty()
      .withMessage("Category cannot be empty"),
    body("smartContractAddress")
      .optional()
      .matches(/^0x[a-fA-F0-9]{40}$/)
      .withMessage("Invalid Ethereum address format"),
    body("contactInfo")
      .optional()
      .isEmail()
      .withMessage("Invalid email format"),
    body("repositoryHost")
      .optional()
      .notEmpty()
      .withMessage("Repository host cannot be empty"),
    body("repositoryName")
      .optional()
      .notEmpty()
      .withMessage("Repository name cannot be empty"),
    body("repositoryLink")
      .optional()
      .isURL()
      .withMessage("Repository link must be a valid URL"),
    body("token").optional().notEmpty().withMessage("Token type cannot be empty"),
    body("allocatedBounty")
      .optional()
      .isFloat({ min: 0 })
      .withMessage("Allocated bounty must be non-negative"),
    body("bountyCurrency")
      .optional()
      .notEmpty()
      .withMessage("Bounty currency cannot be empty"),
    body("dateOfExpiry")
      .optional()
      .isISO8601()
      .toDate()
      .withMessage("Date of expiry must be a valid date"),
  ],
  handleValidationErrors,
  updateProject
);

// Delete Project
router.delete(
  "/:id",
  [param("id").isInt().withMessage("Project ID must be an integer")],
  handleValidationErrors,
  deleteProject
);

module.exports = router;
