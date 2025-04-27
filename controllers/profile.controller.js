const Profile = require('../models/profile.model');
const User = require('../models/user.model');

// watin i need to do CRUD well-well:
// 1. Check Wetin Dem Send: Make sure say the data wey dem send dey correct and clean. No let bad-bad data enter.
// 2. Tell Person When Things Go Wrong: If error happen, tell the person clear-clear wetin go wrong and use the correct code (like 404 for 'no dey').
// 3. Tell Person When Things Go Right: If everything work fine, tell the person straight.
// 4. No Block Road: Use 'async/await' when you dey talk to database so your server no go hang.
// 5. Make Your Reply Make Sense: Arrange your JSON reply in a way wey person go understand every time.


exports.createProfile = async (req, res) => {
  try {
    const { userId, personalInfo, professionalBackground } = req.body;

    if (!userId) {
      return res.status(400).json({ message: 'User ID is required.' });
    }

    const user = await User.findByPk(userId);
    if (!user) {
      return res.status(404).json({ message: 'User not found.' });
    }

    const existingProfile = await Profile.findOne({ where: { userId } });
    if (existingProfile) {
      return res.status(409).json({ message: 'Profile already exists for this user.' });
    }

    const profile = await Profile.create({
      userId,
      personalInfo: personalInfo || {},
      professionalBackground: professionalBackground || {},
      ...personalInfo,
      ...professionalBackground,
    });

    res.status(201).json({ message: 'Profile created successfully.', profile });
  } catch (error) {
    console.error('Error creating profile:', error);
    res.status(500).json({ message: 'Failed to create profile.', error: error.message });
  }
};


exports.getProfileByUser = async (req, res) => {
  try {
    const { userId } = req.params;
    console.log('Received userId:', userId); 

    if (!userId) {
      return res.status(400).json({ message: 'User ID parameter is required.' });
    }

    const profile = await Profile.findOne({ where: { userId } });

    if (!profile) {
      return res.status(404).json({ message: 'Profile not found for this user.' });
    }

    res.status(200).json(profile);
  } catch (error) {
    console.error('Error fetching profile by user ID:', error);
    res.status(500).json({ message: 'Failed to fetch profile.', error: error.message });
  }
};


exports.updateProfile = async (req, res) => {
  try {
    const { profileId } = req.params;
    const { full_name, date_of_birth, email_address, nationality, phone_number, programming_languages, technical_expertise } = req.body;

    if (!profileId) {
      return res.status(400).json({ message: 'Profile ID parameter is required.' });
    }

    const profile = await Profile.findByPk(profileId);
    if (!profile) {
      return res.status(404).json({ message: 'Profile not found.' });
    }

    profile.personalInfo = {
      ...profile.personalInfo,
      full_name: full_name !== undefined ? full_name : profile.personalInfo.full_name,
      date_of_birth: date_of_birth !== undefined ? date_of_birth : profile.personalInfo.date_of_birth,
      email_address: email_address !== undefined ? email_address : profile.personalInfo.email_address,
      nationality: nationality !== undefined ? nationality : profile.personalInfo.nationality,
      phone_number: phone_number !== undefined ? phone_number : profile.personalInfo.phone_number,
    };
    profile.professionalBackground = {
      ...profile.professionalBackground,
      programming_languages: programming_languages !== undefined ? programming_languages : profile.professionalBackground.programming_languages,
      technical_expertise: technical_expertise !== undefined ? technical_expertise : profile.professionalBackground.technical_expertise,
    };

    await profile.save();

    const updatedProfile = await Profile.findByPk(profileId);
    res.status(200).json({ message: 'Profile updated successfully.', profile: updatedProfile });
  } catch (error) {
    console.error('Error updating profile:', error);
    res.status(500).json({ message: 'Failed to update profile.', error: error.message });
  }
};



exports.deleteProfile = async (req, res) => {
  try {
    const { profileId } = req.params;



    if (!profileId) {
      return res.status(400).json({ message: 'Profile ID parameter is required.' });
    }

    const deletedRows = await Profile.destroy({
      where: { id: profileId },
    });

    if (deletedRows === 0) {
      return res.status(404).json({ message: 'Profile not found.' });
    }


    res.status(204).send();
  } catch (error) {
    console.error('Error deleting profile:', error);
    res.status(500).json({ message: 'Failed to delete profile.', error: error.message });
  }
};


exports.updatePersonalInfoVerification = async (req, res) => {
  try {
    const { profileId } = req.params;
    const { status, reason } = req.body;

    if (!profileId || !status) {
      return res.status(400).json({ message: 'Profile ID and status are required.' });
    }

    const profile = await Profile.findByPk(profileId);
    if (!profile) {
      return res.status(404).json({ message: 'Profile not found.' });
    }

    profile.personalInfo = {
      ...profile.personalInfo, 
      verificationStatus: status,
      verificationReason: reason,
    };
    profile.verificationHistory = [...(profile.verificationHistory || []), {
      area: 'personalInfo',
      status,
      reason: reason || '',
      timestamp: new Date(),
    }];

    await profile.save();

    const updatedProfile = await Profile.findByPk(profileId);

    res.status(200).json({ message: 'Personal info verification updated successfully.', profile: updatedProfile });
  } catch (error) {
    console.error('Error updating personal info verification:', error);
    res.status(500).json({ message: 'Failed to update personal info verification.', error: error.message });
  }
};


exports.updateProfessionalBackgroundVerification = async (req, res) => {
  try {
    const { profileId } = req.params;
    const { status, reason } = req.body;

    if (!profileId || !status) {
      return res.status(400).json({ message: 'Profile ID and status are required.' });
    }

    const profile = await Profile.findByPk(profileId);
    if (!profile) {
      return res.status(404).json({ message: 'Profile not found.' });
    }

    const newVerification = {
      area: 'professionalBackground',
      status,
      reason: reason || '',
      timestamp: new Date(),
    };

    profile.professionalBackground = {
      ...profile.professionalBackground,
      verificationStatus: status,
      verificationReason: reason,
    };

    profile.verificationHistory = [...(profile.verificationHistory || []), newVerification];

    await profile.save();

    const updatedProfile = await Profile.findByPk(profileId);

    res.status(200).json({ message: 'Professional background verification updated successfully.', profile: updatedProfile });
  } catch (error) {
    console.error('Error updating professional background verification:', error);
    res.status(400).json({ message: 'Failed to update professional background verification.', error: error.message });
  }
};

exports.updateUserState = async (req, res) => {
  try {
    const { profileId } = req.params;
    const { userState } = req.body;

    if (!profileId || !userState) {
      return res.status(400).json({ message: 'Profile ID and user state are required.' });
    }

    const [updatedRows] = await Profile.update(
      { userState, updatedAt: new Date() },
      { where: { id: profileId } }
    );

    if (updatedRows === 0) {
      return res.status(404).json({ message: 'Profile not found.' });
    }

    const updatedProfile = await Profile.findByPk(profileId);


    res.status(200).json({ message: 'User state updated successfully.', profile: updatedProfile });
  } catch (error) {
    console.error('Error updating user state:', error);
    res.status(400).json({ message: 'Failed to update user state.', error: error.message });
  }
};