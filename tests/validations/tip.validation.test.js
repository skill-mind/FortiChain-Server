const { tipSchema, updateTipSchema } = require('../../validations/tip.validation');

describe('Tip Validation Schemas', () => {
  describe('Tip Schema', () => {
    const validTip = {
      title: 'Security Best Practices',
      content: 'Here are some important security best practices to follow when developing smart contracts.',
      category: 'Security',
      isBestModel: false
    };

    it('should validate valid tip data', () => {
      const { error } = tipSchema.validate(validTip);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = tipSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate title', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        title: 'Valid Tip Title'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short title', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        title: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should reject too long title', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        title: 'a'.repeat(101)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate content', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        content: 'A valid tip content that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short content', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        content: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should reject too long content', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        content: 'a'.repeat(5001)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate category', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        category: 'Valid Category'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too long category', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        category: 'a'.repeat(101)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate isBestModel', () => {
      const { error } = tipSchema.validate({
        ...validTip,
        isBestModel: true
      });
      expect(error).toBeUndefined();
    });

    it('should use default isBestModel value when not provided', () => {
      const { value } = tipSchema.validate({
        title: 'Test Tip',
        content: 'Test content',
        category: 'Test'
      });
      expect(value.isBestModel).toBe(false);
    });
  });

  describe('Update Tip Schema', () => {
    const validUpdate = {
      title: 'Updated Security Best Practices',
      content: 'Updated content with more detailed security best practices.',
      category: 'Updated Security',
      isBestModel: true
    };

    it('should validate valid update data', () => {
      const { error } = updateTipSchema.validate(validUpdate);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { title: 'Updated Title' },
        { content: 'Updated content' },
        { category: 'Updated Category' },
        { isBestModel: true }
      ];

      partialUpdates.forEach(update => {
        const { error } = updateTipSchema.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = updateTipSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate updated title', () => {
      const { error } = updateTipSchema.validate({
        title: 'Valid Updated Title'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated title', () => {
      const { error } = updateTipSchema.validate({
        title: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should reject too long updated title', () => {
      const { error } = updateTipSchema.validate({
        title: 'a'.repeat(101)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated content', () => {
      const { error } = updateTipSchema.validate({
        content: 'A valid updated content that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated content', () => {
      const { error } = updateTipSchema.validate({
        content: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should reject too long updated content', () => {
      const { error } = updateTipSchema.validate({
        content: 'a'.repeat(5001)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated category', () => {
      const { error } = updateTipSchema.validate({
        category: 'Valid Updated Category'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too long updated category', () => {
      const { error } = updateTipSchema.validate({
        category: 'a'.repeat(101)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated isBestModel', () => {
      const { error } = updateTipSchema.validate({
        isBestModel: true
      });
      expect(error).toBeUndefined();
    });
  });
}); 