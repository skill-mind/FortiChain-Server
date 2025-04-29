const request = require('supertest');
const app = require('../../server');
const { Tip, User } = require('../../models');
const sequelize = require('../../config/db.config');

const testUserId = 'a1b2c3d4-e5f6-7890-1234-56789abcdef0';
const anotherTestUserId = 'b9c8d7e6-f5a4-3210-fedc-ba9876543210';
let testTipId;

beforeAll(async () => {
  await sequelize.sync({ force: true });
});

beforeEach(async () => {
  await Tip.destroy({ where: {}, truncate: true });
  await User.destroy({ where: {}, truncate: true });

  await User.create({
    id: testUserId,
    walletAddress: `0x${testUserId.replace(/-/g, '')}`,
    name: 'Test User',
    email: 'test@example.com',
    password: 'hashedpassword',
    role: 'user'
  });

  const tip = await Tip.create({
    title: 'Test Tip',
    content: 'This is a test tip content with sufficient length.',
    category: 'technology',
    authorId: testUserId,
    walletAddress: `0x${testUserId.replace(/-/g, '')}`,
    isBestModel: false,
    upvotes: 0,
    downvotes: 0
  });
  testTipId = tip.id;
});

afterAll(async () => {
  await sequelize.close();
});

describe('Tips Controller Tests', () => {
  describe('POST /api/tips', () => {
    it('should create a new tip successfully', async () => {
      const newTipData = {
        title: 'New Tip',
        content: 'This is a new tip content with sufficient length.',
        category: 'business',
        walletAddress: `0x${testUserId.replace(/-/g, '')}`
      };

      const response = await request(app)
        .post('/api/tips')
        .send(newTipData);

      expect(response.status).toBe(201);
      expect(response.body).toHaveProperty('message', 'Tip created successfully.');
      expect(response.body.tip).toHaveProperty('id');
      expect(response.body.tip.title).toBe('New Tip');
      expect(response.body.tip.category).toBe('business');
      expect(response.body.tip.authorId).toBe(testUserId);

      const createdTip = await Tip.findOne({ where: { title: 'New Tip' } });
      expect(createdTip).toBeDefined();
    });

    it('should return 400 if required fields are missing', async () => {
      const response = await request(app)
        .post('/api/tips')
        .send({
          content: 'Missing title',
          category: 'technology'
        });
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'Title, content, category, and walletAddress are required.');
    });

    it('should return 400 if content is too short', async () => {
      const response = await request(app)
        .post('/api/tips')
        .send({
          title: 'Short Content',
          content: 'Too short',
          category: 'technology',
          walletAddress: `0x${testUserId.replace(/-/g, '')}`
        });
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'Content must be at least 20 characters long.');
    });

    it('should return 400 if category is invalid', async () => {
      const response = await request(app)
        .post('/api/tips')
        .send({
          title: 'Invalid Category',
          content: 'This is a valid content length.',
          category: 'invalid-category',
          walletAddress: `0x${testUserId.replace(/-/g, '')}`
        });
      expect(response.status).toBe(400);
      expect(response.body).toHaveProperty('message', 'Invalid category specified.');
    });
  });

  describe('GET /api/tips', () => {
    it('should retrieve all tips', async () => {
      // Create additional test tip
      await Tip.create({
        title: 'Another Tip',
        content: 'Another test tip content with sufficient length.',
        category: 'business',
        authorId: testUserId,
        walletAddress: `0x${testUserId.replace(/-/g, '')}`
      });

      const response = await request(app).get('/api/tips');
      expect(response.status).toBe(200);
      expect(response.body).toHaveLength(2);
      expect(response.body[0]).toHaveProperty('title', 'Test Tip');
      expect(response.body[1]).toHaveProperty('title', 'Another Tip');
    });

    it('should return empty array when no tips exist', async () => {
      await Tip.destroy({ where: {} });
      const response = await request(app).get('/api/tips');
      expect(response.status).toBe(200);
      expect(response.body).toEqual([]);
    });

    it('should filter tips by category', async () => {
      await Tip.create({
        title: 'Business Tip',
        content: 'Business related tip content.',
        category: 'business',
        authorId: testUserId,
        walletAddress: `0x${testUserId.replace(/-/g, '')}`
      });

      const response = await request(app)
        .get('/api/tips?category=business');
      
      expect(response.status).toBe(200);
      expect(response.body).toHaveLength(1);
      expect(response.body[0]).toHaveProperty('category', 'business');
    });
  });

  describe('GET /api/tips/:id', () => {
    it('should retrieve a single tip by ID', async () => {
      const response = await request(app).get(`/api/tips/${testTipId}`);
      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('id', testTipId);
      expect(response.body).toHaveProperty('title', 'Test Tip');
    });

    it('should return 404 for non-existent tip ID', async () => {
      const nonExistentId = '99999999-9999-9999-9999-999999999999';
      const response = await request(app).get(`/api/tips/${nonExistentId}`);
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Tip not found.');
    });
  });

  describe('PATCH /api/tips/:id/best-model', () => {
    it('should mark a tip as best model', async () => {
      const response = await request(app)
        .patch(`/api/tips/${testTipId}/best-model`)
        .send({ isBestModel: true });

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'Tip best model status updated successfully.');
      expect(response.body.tip).toHaveProperty('isBestModel', true);

      const updatedTip = await Tip.findByPk(testTipId);
      expect(updatedTip.isBestModel).toBe(true);
    });

    it('should unmark a tip as best model', async () => {
      // First mark as best model
      await Tip.update({ isBestModel: true }, { where: { id: testTipId } });

      const response = await request(app)
        .patch(`/api/tips/${testTipId}/best-model`)
        .send({ isBestModel: false });

      expect(response.status).toBe(200);
      expect(response.body.tip).toHaveProperty('isBestModel', false);

      const updatedTip = await Tip.findByPk(testTipId);
      expect(updatedTip.isBestModel).toBe(false);
    });

    it('should return 404 for non-existent tip ID', async () => {
      const nonExistentId = '99999999-9999-9999-9999-999999999999';
      const response = await request(app)
        .patch(`/api/tips/${nonExistentId}/best-model`)
        .send({ isBestModel: true });
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Tip not found.');
    });
  });

  describe('PATCH /api/tips/:id/upvote', () => {
    it('should increment upvotes for a tip', async () => {
      const response = await request(app)
        .patch(`/api/tips/${testTipId}/upvote`);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'Tip upvoted successfully.');
      expect(response.body.tip).toHaveProperty('upvotes', 1);

      const updatedTip = await Tip.findByPk(testTipId);
      expect(updatedTip.upvotes).toBe(1);
    });

    it('should return 404 for non-existent tip ID', async () => {
      const nonExistentId = '99999999-9999-9999-9999-999999999999';
      const response = await request(app)
        .patch(`/api/tips/${nonExistentId}/upvote`);
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Tip not found.');
    });
  });

  describe('PATCH /api/tips/:id/downvote', () => {
    it('should increment downvotes for a tip', async () => {
      const response = await request(app)
        .patch(`/api/tips/${testTipId}/downvote`);

      expect(response.status).toBe(200);
      expect(response.body).toHaveProperty('message', 'Tip downvoted successfully.');
      expect(response.body.tip).toHaveProperty('downvotes', 1);

      const updatedTip = await Tip.findByPk(testTipId);
      expect(updatedTip.downvotes).toBe(1);
    });

    it('should return 404 for non-existent tip ID', async () => {
      const nonExistentId = '99999999-9999-9999-9999-999999999999';
      const response = await request(app)
        .patch(`/api/tips/${nonExistentId}/downvote`);
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Tip not found.');
    });
  });

  describe('DELETE /api/tips/:id', () => {
    it('should delete a tip successfully', async () => {
      const response = await request(app).delete(`/api/tips/${testTipId}`);
      expect(response.status).toBe(204);

      const deletedTip = await Tip.findByPk(testTipId);
      expect(deletedTip).toBeNull();
    });

    it('should return 404 for non-existent tip ID', async () => {
      const nonExistentId = '99999999-9999-9999-9999-999999999999';
      const response = await request(app).delete(`/api/tips/${nonExistentId}`);
      expect(response.status).toBe(404);
      expect(response.body).toHaveProperty('message', 'Tip not found.');
    });
  });
});