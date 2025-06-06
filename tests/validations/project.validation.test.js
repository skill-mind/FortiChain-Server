const { projectSchema, updateProjectSchema } = require('../../validations/project.validation');

describe('Project Validation Schemas', () => {
  describe('Project Schema', () => {
    const validProject = {
      name: 'Test Project',
      description: 'A test project description',
      category: 'DeFi',
      smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
      website: 'https://testproject.com',
      github: 'https://github.com/testproject',
      twitter: 'https://twitter.com/testproject',
      discord: 'https://discord.gg/testproject',
      telegram: 'https://t.me/testproject',
      autoTopUp: false,
      supportingDocument: {
        mimetype: 'application/pdf',
        size: 2 * 1024 * 1024,
        name: 'test.pdf'
      },
      projectLogo: {
        mimetype: 'image/jpeg',
        size: 1 * 1024 * 1024,
        name: 'logo.jpg'
      }
    };

    it('should validate valid project data', () => {
      const { error } = projectSchema.validate(validProject);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = projectSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate project name', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        name: 'Valid Project Name'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short project name', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        name: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate project description', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        description: 'A valid project description that is long enough'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short project description', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        description: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate project category', () => {
      const validCategories = ['DeFi', 'NFT', 'GameFi', 'Infrastructure', 'Other'];
      validCategories.forEach(category => {
        const { error } = projectSchema.validate({
          ...validProject,
          category
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid project category', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        category: 'InvalidCategory'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate smart contract address', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid smart contract address', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        smartContractAddress: 'invalid-address'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate URLs', () => {
      const validUrls = {
        website: 'https://testproject.com',
        github: 'https://github.com/testproject',
        twitter: 'https://twitter.com/testproject',
        discord: 'https://discord.gg/testproject',
        telegram: 'https://t.me/testproject'
      };

      Object.entries(validUrls).forEach(([field, url]) => {
        const { error } = projectSchema.validate({
          ...validProject,
          [field]: url
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid URLs', () => {
      const invalidUrls = {
        website: 'not-a-url',
        github: 'invalid-github',
        twitter: 'invalid-twitter',
        discord: 'invalid-discord',
        telegram: 'invalid-telegram'
      };

      Object.entries(invalidUrls).forEach(([field, url]) => {
        const { error } = projectSchema.validate({
          ...validProject,
          [field]: url
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });
    });

    it('should validate supporting document', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        supportingDocument: {
          mimetype: 'application/pdf',
          size: 2 * 1024 * 1024,
          name: 'test.pdf'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid supporting document', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        supportingDocument: {
          mimetype: 'image/jpeg',
          size: 2 * 1024 * 1024,
          name: 'test.jpg'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate project logo', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        projectLogo: {
          mimetype: 'image/jpeg',
          size: 1 * 1024 * 1024,
          name: 'logo.jpg'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid project logo', () => {
      const { error } = projectSchema.validate({
        ...validProject,
        projectLogo: {
          mimetype: 'application/pdf',
          size: 1 * 1024 * 1024,
          name: 'logo.pdf'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });

  describe('Update Project Schema', () => {
    const validUpdate = {
      name: 'Updated Project',
      description: 'Updated project description',
      category: 'NFT',
      smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
      website: 'https://updatedproject.com',
      github: 'https://github.com/updatedproject',
      twitter: 'https://twitter.com/updatedproject',
      discord: 'https://discord.gg/updatedproject',
      telegram: 'https://t.me/updatedproject',
      autoTopUp: true,
      supportingDocument: {
        mimetype: 'application/pdf',
        size: 2 * 1024 * 1024,
        name: 'updated.pdf'
      },
      projectLogo: {
        mimetype: 'image/jpeg',
        size: 1 * 1024 * 1024,
        name: 'updated.jpg'
      }
    };

    it('should validate valid update data', () => {
      const { error } = updateProjectSchema.validate(validUpdate);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { name: 'Updated Project' },
        { description: 'Updated description' },
        { category: 'NFT' },
        { smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e' },
        { website: 'https://updatedproject.com' },
        { autoTopUp: true }
      ];

      partialUpdates.forEach(update => {
        const { error } = updateProjectSchema.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = updateProjectSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate updated project name', () => {
      const { error } = updateProjectSchema.validate({
        name: 'Valid Updated Name'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated project name', () => {
      const { error } = updateProjectSchema.validate({
        name: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated project description', () => {
      const { error } = updateProjectSchema.validate({
        description: 'A valid updated project description that is long enough'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated project description', () => {
      const { error } = updateProjectSchema.validate({
        description: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated project category', () => {
      const validCategories = ['DeFi', 'NFT', 'GameFi', 'Infrastructure', 'Other'];
      validCategories.forEach(category => {
        const { error } = updateProjectSchema.validate({ category });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid updated project category', () => {
      const { error } = updateProjectSchema.validate({
        category: 'InvalidCategory'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated smart contract address', () => {
      const { error } = updateProjectSchema.validate({
        smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid updated smart contract address', () => {
      const { error } = updateProjectSchema.validate({
        smartContractAddress: 'invalid-address'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated URLs', () => {
      const validUrls = {
        website: 'https://updatedproject.com',
        github: 'https://github.com/updatedproject',
        twitter: 'https://twitter.com/updatedproject',
        discord: 'https://discord.gg/updatedproject',
        telegram: 'https://t.me/updatedproject'
      };

      Object.entries(validUrls).forEach(([field, url]) => {
        const { error } = updateProjectSchema.validate({
          [field]: url
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid updated URLs', () => {
      const invalidUrls = {
        website: 'not-a-url',
        github: 'invalid-github',
        twitter: 'invalid-twitter',
        discord: 'invalid-discord',
        telegram: 'invalid-telegram'
      };

      Object.entries(invalidUrls).forEach(([field, url]) => {
        const { error } = updateProjectSchema.validate({
          [field]: url
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });
    });

    it('should validate updated supporting document', () => {
      const { error } = updateProjectSchema.validate({
        supportingDocument: {
          mimetype: 'application/pdf',
          size: 2 * 1024 * 1024,
          name: 'updated.pdf'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid updated supporting document', () => {
      const { error } = updateProjectSchema.validate({
        supportingDocument: {
          mimetype: 'image/jpeg',
          size: 2 * 1024 * 1024,
          name: 'updated.jpg'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated project logo', () => {
      const { error } = updateProjectSchema.validate({
        projectLogo: {
          mimetype: 'image/jpeg',
          size: 1 * 1024 * 1024,
          name: 'updated.jpg'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid updated project logo', () => {
      const { error } = updateProjectSchema.validate({
        projectLogo: {
          mimetype: 'application/pdf',
          size: 1 * 1024 * 1024,
          name: 'updated.pdf'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });
}); 