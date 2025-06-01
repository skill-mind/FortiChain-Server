const express = require("express");
const ProfileController = require("../controllers/profile.controller");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

const router = express.Router();

// Create profile - accessible to USER, ADMIN, SUPER_ADMIN
router.post(
  "/profiles",
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  ProfileController.createProfile
);

// Get profile by userId - accessible to USER, ADMIN, SUPER_ADMIN
router.get(
  "/profiles/user/:userId",
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  ProfileController.getProfileByUser
);

// Update profile - accessible to ADMIN, SUPER_ADMIN only
router.put(
  "/profiles/:profileId",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  ProfileController.updateProfile
);

// Delete profile - accessible to SUPER_ADMIN only
router.delete(
  "/profiles/:profileId",
  authenticate,
  authorize(roles.SUPER_ADMIN),
  ProfileController.deleteProfile
);

// Update personal info verification - accessible to ADMIN, SUPER_ADMIN only
router.put(
  "/profiles/:profileId/personal-info/verification",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  ProfileController.updatePersonalInfoVerification
);

// Update professional background verification - accessible to ADMIN, SUPER_ADMIN only
router.put(
  "/profiles/:profileId/professional-background/verification",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  ProfileController.updateProfessionalBackgroundVerification
);

// Update user state - accessible to ADMIN, SUPER_ADMIN only
router.put(
  "/profiles/:profileId/user-state",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  ProfileController.updateUserState
);

module.exports = router;
