const { helpRequestSchema, updateHelpRequestSchema } = require('../../validations/helpRequest.validation');

describe('Help Request Validation Schemas', () => {
  describe('Help Request Schema', () => {
    const validHelpRequest = {
      email: 'test@example.com',
      subject: 'Vulnerability Report',
      message: 'I found a potential security issue in the smart contract.',
      document: {
        mimetype: 'application/pdf',
        size: 2 * 1024 * 1024,
        name: 'vulnerability.pdf'
      }
    };

    it('should validate valid help request data', () => {
      const { error } = helpRequestSchema.validate(validHelpRequest);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = helpRequestSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate email', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        email: 'valid.email@example.com'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid email', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        email: 'invalid-email'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate subject', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        subject: 'Custom Subject'
      });
      expect(error).toBeUndefined();
    });

    it('should use default subject when not provided', () => {
      const { value } = helpRequestSchema.validate({
        email: 'test@example.com',
        message: 'Test message',
        document: validHelpRequest.document
      });
      expect(value.subject).toBe('Vulnerability Report');
    });

    it('should validate message', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        message: 'A valid message that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short message', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        message: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate document', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        document: {
          mimetype: 'application/pdf',
          size: 2 * 1024 * 1024,
          name: 'vulnerability.pdf'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject non-PDF document', () => {
      const { error } = helpRequestSchema.validate({
        ...validHelpRequest,
        document: {
          mimetype: 'image/jpeg',
          size: 2 * 1024 * 1024,
          name: 'vulnerability.jpg'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });

  describe('Update Help Request Schema', () => {
    const validUpdate = {
      email: 'updated@example.com',
      subject: 'Updated Subject',
      message: 'Updated message with more details about the vulnerability.',
      document: {
        mimetype: 'application/pdf',
        size: 2 * 1024 * 1024,
        name: 'updated.pdf'
      },
      status: 'in_progress'
    };

    it('should validate valid update data', () => {
      const { error } = updateHelpRequestSchema.validate(validUpdate);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { email: 'updated@example.com' },
        { subject: 'Updated Subject' },
        { message: 'Updated message' },
        { status: 'in_progress' }
      ];

      partialUpdates.forEach(update => {
        const { error } = updateHelpRequestSchema.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = updateHelpRequestSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate updated email', () => {
      const { error } = updateHelpRequestSchema.validate({
        email: 'valid.updated@example.com'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid updated email', () => {
      const { error } = updateHelpRequestSchema.validate({
        email: 'invalid-email'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated subject', () => {
      const { error } = updateHelpRequestSchema.validate({
        subject: 'Valid Updated Subject'
      });
      expect(error).toBeUndefined();
    });

    it('should validate updated message', () => {
      const { error } = updateHelpRequestSchema.validate({
        message: 'A valid updated message that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated message', () => {
      const { error } = updateHelpRequestSchema.validate({
        message: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated document', () => {
      const { error } = updateHelpRequestSchema.validate({
        document: {
          mimetype: 'application/pdf',
          size: 2 * 1024 * 1024,
          name: 'updated.pdf'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject non-PDF updated document', () => {
      const { error } = updateHelpRequestSchema.validate({
        document: {
          mimetype: 'image/jpeg',
          size: 2 * 1024 * 1024,
          name: 'updated.jpg'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate status', () => {
      const validStatuses = ['open', 'in_progress', 'resolved', 'closed'];
      validStatuses.forEach(status => {
        const { error } = updateHelpRequestSchema.validate({ status });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid status', () => {
      const { error } = updateHelpRequestSchema.validate({
        status: 'invalid_status'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });
}); 