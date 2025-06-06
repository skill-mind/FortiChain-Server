const { string } = require('../../../validations/core');

describe('String Validation Schemas', () => {
  describe('Email Validation', () => {
    it('should validate correct email format', () => {
      const { error } = string.email.validate('test@example.com');
      expect(error).toBeUndefined();
    });

    it('should reject invalid email format', () => {
      const { error } = string.email.validate('invalid-email');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('email');
    });

    it('should allow optional email', () => {
      const { error } = string.optionalEmail.validate('test@example.com');
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional email', () => {
      const { error } = string.optionalEmail.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('URL Validation', () => {
    it('should validate correct URL format', () => {
      const { error } = string.url.validate('https://example.com');
      expect(error).toBeUndefined();
    });

    it('should reject invalid URL format', () => {
      const { error } = string.url.validate('not-a-url');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('uri');
    });

    it('should allow optional URL', () => {
      const { error } = string.optionalUrl.validate('https://example.com');
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional URL', () => {
      const { error } = string.optionalUrl.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('UUID Validation', () => {
    it('should validate correct UUID format', () => {
      const { error } = string.uuid.validate('123e4567-e89b-12d3-a456-426614174000');
      expect(error).toBeUndefined();
    });

    it('should reject invalid UUID format', () => {
      const { error } = string.uuid.validate('invalid-uuid');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('uuid');
    });

    it('should allow optional UUID', () => {
      const { error } = string.optionalUuid.validate('123e4567-e89b-12d3-a456-426614174000');
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional UUID', () => {
      const { error } = string.optionalUuid.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Blockchain Address Validation', () => {
    it('should validate correct Ethereum address', () => {
      const { error } = string.ethereumAddress.validate('0x742d35Cc6634C0532925a3b844Bc454e4438f44e');
      expect(error).toBeUndefined();
    });

    it('should reject invalid Ethereum address', () => {
      const { error } = string.ethereumAddress.validate('0xinvalid');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('pattern');
    });

    it('should validate correct Starknet address', () => {
      const { error } = string.starknetAddress.validate('0x1234567890123456789012345678901234567890123456789012345678901234');
      expect(error).toBeUndefined();
    });

    it('should reject invalid Starknet address', () => {
      const { error } = string.starknetAddress.validate('0xinvalid');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('pattern');
    });

    it('should validate correct Stellar address', () => {
      const { error } = string.stellarAddress.validate('GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7');
      expect(error).toBeUndefined();
    });

    it('should reject invalid Stellar address', () => {
      const { error } = string.stellarAddress.validate('invalid');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('pattern');
    });
  });

  describe('Text Length Validation', () => {
    describe('Short Text', () => {
      const shortText = string.shortText(3, 10);

      it('should validate text within length limits', () => {
        const { error } = shortText.validate('valid');
        expect(error).toBeUndefined();
      });

      it('should reject text too short', () => {
        const { error } = shortText.validate('ab');
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('length');
      });

      it('should reject text too long', () => {
        const { error } = shortText.validate('this is too long');
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('length');
      });
    });

    describe('Medium Text', () => {
      const mediumText = string.mediumText(10, 20);

      it('should validate text within length limits', () => {
        const { error } = mediumText.validate('valid medium text');
        expect(error).toBeUndefined();
      });

      it('should reject text too short', () => {
        const { error } = mediumText.validate('too short');
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('length');
      });

      it('should reject text too long', () => {
        const { error } = mediumText.validate('this is way too long for a medium text field');
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('length');
      });
    });

    describe('Long Text', () => {
      const longText = string.longText(10, 30);

      it('should validate text within length limits', () => {
        const { error } = longText.validate('this is a valid long text');
        expect(error).toBeUndefined();
      });

      it('should reject text too short', () => {
        const { error } = longText.validate('too short');
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('length');
      });

      it('should reject text too long', () => {
        const { error } = longText.validate('this is way too long for a long text field and should be rejected');
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('length');
      });
    });
  });

  describe('Enum Validation', () => {
    const validValues = ['option1', 'option2', 'option3'];
    const enumSchema = string.enum(validValues);
    const optionalEnumSchema = string.optionalEnum(validValues);

    it('should validate correct enum value', () => {
      const { error } = enumSchema.validate('option1');
      expect(error).toBeUndefined();
    });

    it('should reject invalid enum value', () => {
      const { error } = enumSchema.validate('invalid');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should allow optional enum value', () => {
      const { error } = optionalEnumSchema.validate('option1');
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional enum', () => {
      const { error } = optionalEnumSchema.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('File Type Validation', () => {
    const validTypes = ['image/jpeg', 'image/png'];
    const fileTypeSchema = string.fileType(validTypes);
    const optionalFileTypeSchema = string.optionalFileType(validTypes);

    it('should validate correct file type', () => {
      const { error } = fileTypeSchema.validate('image/jpeg');
      expect(error).toBeUndefined();
    });

    it('should reject invalid file type', () => {
      const { error } = fileTypeSchema.validate('application/pdf');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should allow optional file type', () => {
      const { error } = optionalFileTypeSchema.validate('image/jpeg');
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional file type', () => {
      const { error } = optionalFileTypeSchema.validate(undefined);
      expect(error).toBeUndefined();
    });
  });
}); 