const express = require('express');
const ProfileController = require('../controllers/profile.controller');



const router = express.Router();



router.post('/profiles', ProfileController.createProfile);
router.get('/profiles/user/:userId', ProfileController.getProfileByUser);



router.put('/profiles/:profileId', ProfileController.updateProfile);
router.delete('/profiles/:profileId', ProfileController.deleteProfile);
router.put('/profiles/:profileId/personal-info/verification', ProfileController.updatePersonalInfoVerification);
router.put('/profiles/:profileId/professional-background/verification', ProfileController.updateProfessionalBackgroundVerification);
router.put('/profiles/:profileId/user-state', ProfileController.updateUserState);

module.exports = router;


