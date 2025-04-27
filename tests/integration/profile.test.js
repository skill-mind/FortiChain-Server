const request = require('supertest');
const app = require('../../server');
const { Profile, User } = require('../../models');
const sequelize = require('../../config/db.config');

const testUserId = 'a1b2c3d4-e5f6-7890-1234-56789abcdef0';
const anotherTestUserId = 'b9c8d7e6-f5a4-3210-fedc-ba9876543210';
let testProfileId;

beforeAll(async () => {
  await sequelize.sync({ force: true });
});

beforeEach(async () => {
  await Profile.destroy({ where: {}, truncate: true });
  await User.destroy({ where: {}, truncate: true });

  await User.create({
    id: testUserId,
    walletAddress: `0x${testUserId.replace(/-/g, '')}`,
  });

  const profile = await Profile.create({
    userId: testUserId,
    personalInfo: { full_name: 'Test User', email_address: 'test@example.com' },
    professionalBackground: {},
    verificationHistory: [],
  });
  testProfileId = profile.id;
});

afterAll(async () => {
  await sequelize.close();
});

describe('Profile Controller Tests', () => {
  describe('POST /api/profiles', () => {
    it('should create a new profile successfully', async () => {
      const newUser = await User.create({
        id: anotherTestUserId,
        walletAddress: `0x${anotherTestUserId.replace(/-/g, '')}`,
      });
      const newProfileData = {
        userId: newUser.id,
        personalInfo: { full_name: 'New User', email_address: 'new@example.com' },
        professionalBackground: { experience: 'Junior Dev' },
      };
      const response = await request(app)
        .post('/api/profiles')
        .send(newProfileData);

      expect(response.status).toBe(201);
      expect(response.body).toHaveProperty('message', 'Profile created successfully.');
      expect(response.body.profile).toHaveProperty('userId', newUser.id);
      expect(response.body.profile.personalInfo).toHaveProperty('full_name', 'New User');
      expect(response.body.profile.professionalBackground).toHaveProperty('experience', 'Junior Dev');

      const createdProfile = await Profile.findOne({ where: { userId: newUser.id } });
      expect(createdProfile).toBeDefined();
    });

    it('should return 400 if userId is missing', async () => {
      const response = await request(app)
        .post('/api/profiles')
        .send({ personalInfo: { full_name: 'No User' } });
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'User ID is required.');
    });

    it('should return 404 if user with provided userId is not found', async () => {
      const nonExistingUserId = 'ffffffff-ffff-ffff-ffff-ffffffffffff';
      const response = await request(app)
        .post('/api/profiles')
        .send({ userId: nonExistingUserId, personalInfo: { full_name: 'Not Found User' } });
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'User not found.');
    });

    it('should return 409 if a profile already exists for the userId', async () => {
      const existingProfileData = {
        userId: testUserId,
        personalInfo: { full_name: 'Another Test' },
      };
      const response = await request(app)
        .post('/api/profiles')
        .send(existingProfileData);
      expect(response.status).toBe(409);
      expect(response.body).toHaveProperty('message', 'Profile already exists for this user.');
    });
  });

  describe('GET /api/profiles/user/:userId', () => {
    it('should return the profile for a valid userId', async () => {
      const response = await request(app).get(`/api/profiles/user/${testUserId}`);
      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('userId', testUserId);
      expect(response.body.personalInfo).toHaveProperty('full_name', 'Test User');
      expect(response.body).toHaveProperty('id', testProfileId);
    });

    it('should return 404 for a non-existing userId', async () => {
      const nonExistingUserId = '00000000-0000-0000-0000-000000000000';
      const response = await request(app).get(`/api/profiles/user/${nonExistingUserId}`);
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Profile not found for this user.');
    });
  });

  describe('PUT /api/profiles/:profileId', () => {
    it('should update a profile successfully', async () => {
      const updatedData = {
        full_name: 'Updated User Name',
        email_address: 'updated@example.com',
        programming_languages: ['JavaScript', 'TypeScript'],
      };
      const response = await request(app)
        .put(`/api/profiles/${testProfileId}`)
        .send(updatedData);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'Profile updated successfully.');
      expect(response.body.profile).toHaveProperty('personalInfo');
      expect(response.body.profile.personalInfo).toHaveProperty('full_name', 'Updated User Name');
      expect(response.body.profile.personalInfo).toHaveProperty('email_address', 'updated@example.com');
      expect(response.body.profile).toHaveProperty('professionalBackground');
      expect(response.body.profile.professionalBackground).toHaveProperty('programming_languages', ['JavaScript', 'TypeScript']);

      const updatedProfile = await Profile.findByPk(testProfileId);
      expect(updatedProfile.personalInfo.full_name).toBe('Updated User Name');
      expect(updatedProfile.personalInfo.email_address).toBe('updated@example.com');
      expect(updatedProfile.professionalBackground.programming_languages).toEqual(['JavaScript', 'TypeScript']);
    });

    it('should return 400 if profileId parameter is missing (though this might be handled by routing)', async () => {
      const response = await request(app)
        .put('/api/profiles/')
        .send({ full_name: 'Update' });
      expect(response.status).toBe(404);
    });

    it('should return 404 if the profile does not exist', async () => {
      const nonExistingProfileId = 99999;
      const response = await request(app)
        .put(`/api/profiles/${nonExistingProfileId}`)
        .send({ full_name: 'Update' });
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Profile not found.');
    });
  });

  describe('DELETE /api/profiles/:profileId', () => {
    it('should delete a profile with a valid profileId', async () => {
      const response = await request(app).delete(`/api/profiles/${testProfileId}`);
      expect(response.status).toBe(204);

      const deletedProfile = await Profile.findByPk(testProfileId);
      expect(deletedProfile).toBeNull();
    });

    it('should return 404 if the profile does not exist', async () => {
      const nonExistingProfileId = 99999;
      const response = await request(app).delete(`/api/profiles/${nonExistingProfileId}`);
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Profile not found.');
    });
  });

  describe('PUT /api/profiles/:profileId/personal-info/verification', () => {
    it('should update personal info verification successfully', async () => {
      const verificationData = { status: 'verified', reason: 'Documents approved' };
      const response = await request(app)
        .put(`/api/profiles/${testProfileId}/personal-info/verification`)
        .send(verificationData);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'Personal info verification updated successfully.');
      expect(response.body.profile).toHaveProperty('personalInfo');
      expect(response.body.profile.personalInfo).toHaveProperty('verificationStatus', 'verified');
      expect(response.body.profile.personalInfo).toHaveProperty('verificationReason', 'Documents approved');
      expect(response.body.profile.verificationHistory).toHaveLength(1);
      expect(response.body.profile.verificationHistory[0]).toHaveProperty('area', 'personalInfo');
      expect(response.body.profile.verificationHistory[0]).toHaveProperty('status', 'verified');
      expect(response.body.profile.verificationHistory[0]).toHaveProperty('reason', 'Documents approved');

      const updatedProfile = await Profile.findByPk(testProfileId);
      expect(updatedProfile.personalInfo).toHaveProperty('verificationStatus', 'verified');
      expect(updatedProfile.personalInfo).toHaveProperty('verificationReason', 'Documents approved');
      expect(updatedProfile.verificationHistory).toHaveLength(1);
      expect(updatedProfile.verificationHistory[0]).toHaveProperty('area', 'personalInfo');
      expect(updatedProfile.verificationHistory[0]).toHaveProperty('status', 'verified');
      expect(updatedProfile.verificationHistory[0]).toHaveProperty('reason', 'Documents approved');
    });

    it('should return 400 if profileId or status is missing', async () => {
      let response = await request(app)
        .put(`/api/profiles/${testProfileId}/personal-info/verification`)
        .send({ reason: 'Missing status' });
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'Profile ID and status are required.');

      response = await request(app)
        .put(`/api/profiles//personal-info/verification`) 
        .send({ status: 'verified' });
      expect(response.status).toBe(404);
    });

    it('should return 404 if the profile does not exist', async () => {
      const nonExistingProfileId = 99999;
      const response = await request(app)
        .put(`/api/profiles/${nonExistingProfileId}/personal-info/verification`)
        .send({ status: 'verified' });
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Profile not found.');
    });
  });

  describe('PUT /api/profiles/:profileId/professional-background/verification', () => {
    it('should update professional background verification successfully', async () => {
      const verificationData = { status: 'under_review', reason: 'Checking references' };
      const response = await request(app)
        .put(`/api/profiles/${testProfileId}/professional-background/verification`)
        .send(verificationData);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'Professional background verification updated successfully.');
      expect(response.body.profile).toHaveProperty('professionalBackground');
      expect(response.body.profile.professionalBackground).toHaveProperty('verificationStatus', 'under_review');
      expect(response.body.profile.professionalBackground).toHaveProperty('verificationReason', 'Checking references');
      expect(response.body.profile.verificationHistory).toHaveLength(1);
      expect(response.body.profile.verificationHistory[0]).toHaveProperty('area', 'professionalBackground');
      expect(response.body.profile.verificationHistory[0]).toHaveProperty('status', 'under_review');
      expect(response.body.profile.verificationHistory[0]).toHaveProperty('reason', 'Checking references');

      const updatedProfile = await Profile.findByPk(testProfileId);
      expect(updatedProfile.professionalBackground).toHaveProperty('verificationStatus', 'under_review');
      expect(updatedProfile.professionalBackground).toHaveProperty('verificationReason', 'Checking references');
      expect(updatedProfile.verificationHistory).toHaveLength(1);
      expect(updatedProfile.verificationHistory[0]).toHaveProperty('area', 'professionalBackground');
      expect(updatedProfile.verificationHistory[0]).toHaveProperty('status', 'under_review');
      expect(updatedProfile.verificationHistory[0]).toHaveProperty('reason', 'Checking references');
    });

    it('should return 400 if profileId or status is missing', async () => {
      let response = await request(app)
        .put(`/api/profiles/${testProfileId}/professional-background/verification`)
        .send({ reason: 'Missing status' });
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'Profile ID and status are required.');

      response = await request(app)
        .put(`/api/profiles//professional-background/verification`) 
        .send({ status: 'under_review' });
      expect(response.status).toBe(404);
    });

    it('should return 404 if the profile does not exist', async () => {
      const nonExistingProfileId = 99999;
      const response = await request(app)
        .put(`/api/profiles/${nonExistingProfileId}/professional-background/verification`)
        .send({ status: 'under_review' });
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Profile not found.');
    });
  });

  describe('PUT /api/profiles/:profileId/user-state', () => {
    it('should update user state successfully', async () => {
      const updateData = { userState: 'suspended' };
      const response = await request(app)
        .put(`/api/profiles/${testProfileId}/user-state`)
        .send(updateData);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'User state updated successfully.');
      expect(response.body.profile).toHaveProperty('userState', 'suspended');

      const updatedProfile = await Profile.findByPk(testProfileId);
      expect(updatedProfile.userState).toBe('suspended');
    });

    it('should return 400 if profileId or userState is missing', async () => {
      let response = await request(app)
        .put(`/api/profiles/${testProfileId}/user-state`)
        .send({});
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'Profile ID and user state are required.');

      response = await request(app)
        .put(`/api/profiles//user-state`)
        .send({ userState: 'active' });
      expect(response.status).toBe(404); 
    });

    it('should return 404 if the profile does not exist', async () => {
      const nonExistingProfileId = 99999;
      const response = await request(app)
        .put(`/api/profiles/${nonExistingProfileId}/user-state`)
        .send({ userState: 'active' });
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Profile not found.');
    });
  });
});



