const { number } = require('../../../validations/core');

describe('Number Validation Schemas', () => {
  describe('Positive Number Validation', () => {
    it('should validate positive number', () => {
      const { error } = number.positive.validate(42);
      expect(error).toBeUndefined();
    });

    it('should validate zero', () => {
      const { error } = number.positive.validate(0);
      expect(error).toBeUndefined();
    });

    it('should reject negative number', () => {
      const { error } = number.positive.validate(-42);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should allow optional positive number', () => {
      const { error } = number.optionalPositive.validate(42);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional positive number', () => {
      const { error } = number.optionalPositive.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Percentage Validation', () => {
    it('should validate percentage within range', () => {
      const { error } = number.percentage.validate(75);
      expect(error).toBeUndefined();
    });

    it('should validate zero percentage', () => {
      const { error } = number.percentage.validate(0);
      expect(error).toBeUndefined();
    });

    it('should validate 100 percent', () => {
      const { error } = number.percentage.validate(100);
      expect(error).toBeUndefined();
    });

    it('should reject negative percentage', () => {
      const { error } = number.percentage.validate(-10);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject percentage over 100', () => {
      const { error } = number.percentage.validate(150);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should allow optional percentage', () => {
      const { error } = number.optionalPercentage.validate(75);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional percentage', () => {
      const { error } = number.optionalPercentage.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Currency Validation', () => {
    const currency = number.currency(0.01);
    const optionalCurrency = number.optionalCurrency(0.01);

    it('should validate valid currency amount', () => {
      const { error } = currency.validate(42.50);
      expect(error).toBeUndefined();
    });

    it('should validate minimum amount', () => {
      const { error } = currency.validate(0.01);
      expect(error).toBeUndefined();
    });

    it('should reject amount below minimum', () => {
      const { error } = currency.validate(0.005);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject negative amount', () => {
      const { error } = currency.validate(-42.50);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should allow optional currency amount', () => {
      const { error } = optionalCurrency.validate(42.50);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional currency amount', () => {
      const { error } = optionalCurrency.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Integer Validation', () => {
    const integer = number.integer(1);
    const optionalInteger = number.optionalInteger(1);

    it('should validate valid integer', () => {
      const { error } = integer.validate(42);
      expect(error).toBeUndefined();
    });

    it('should validate minimum value', () => {
      const { error } = integer.validate(1);
      expect(error).toBeUndefined();
    });

    it('should reject decimal number', () => {
      const { error } = integer.validate(42.5);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('integer');
    });

    it('should reject value below minimum', () => {
      const { error } = integer.validate(0);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should allow optional integer', () => {
      const { error } = optionalInteger.validate(42);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional integer', () => {
      const { error } = optionalInteger.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Score Validation', () => {
    it('should validate score within range', () => {
      const { error } = number.score.validate(7.5);
      expect(error).toBeUndefined();
    });

    it('should validate minimum score', () => {
      const { error } = number.score.validate(0);
      expect(error).toBeUndefined();
    });

    it('should validate maximum score', () => {
      const { error } = number.score.validate(10);
      expect(error).toBeUndefined();
    });

    it('should reject negative score', () => {
      const { error } = number.score.validate(-1);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject score over 10', () => {
      const { error } = number.score.validate(11);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should allow optional score', () => {
      const { error } = number.optionalScore.validate(7.5);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional score', () => {
      const { error } = number.optionalScore.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('File Size Validation', () => {
    const maxSize = 5 * 1024 * 1024; // 5MB
    const fileSize = number.fileSize(maxSize);
    const optionalFileSize = number.optionalFileSize(maxSize);

    it('should validate file size within limit', () => {
      const { error } = fileSize.validate(2 * 1024 * 1024);
      expect(error).toBeUndefined();
    });

    it('should validate maximum file size', () => {
      const { error } = fileSize.validate(maxSize);
      expect(error).toBeUndefined();
    });

    it('should reject file size over limit', () => {
      const { error } = fileSize.validate(6 * 1024 * 1024);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should reject negative file size', () => {
      const { error } = fileSize.validate(-1024);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should allow optional file size', () => {
      const { error } = optionalFileSize.validate(2 * 1024 * 1024);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional file size', () => {
      const { error } = optionalFileSize.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Range Validation', () => {
    const range = number.range(1, 10);
    const optionalRange = number.optionalRange(1, 10);

    it('should validate number within range', () => {
      const { error } = range.validate(5);
      expect(error).toBeUndefined();
    });

    it('should validate minimum value', () => {
      const { error } = range.validate(1);
      expect(error).toBeUndefined();
    });

    it('should validate maximum value', () => {
      const { error } = range.validate(10);
      expect(error).toBeUndefined();
    });

    it('should reject number below range', () => {
      const { error } = range.validate(0);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject number above range', () => {
      const { error } = range.validate(11);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should allow optional range value', () => {
      const { error } = optionalRange.validate(5);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional range value', () => {
      const { error } = optionalRange.validate(undefined);
      expect(error).toBeUndefined();
    });
  });
}); 