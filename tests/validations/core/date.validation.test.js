const { date } = require('../../../validations/core');

describe('Date Validation Schemas', () => {
  describe('ISO Date Validation', () => {
    it('should validate correct ISO date', () => {
      const { error } = date.isoDate.validate('2024-03-15T12:00:00Z');
      expect(error).toBeUndefined();
    });

    it('should reject invalid date format', () => {
      const { error } = date.isoDate.validate('15-03-2024');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('iso');
    });

    it('should allow optional ISO date', () => {
      const { error } = date.optionalIsoDate.validate('2024-03-15T12:00:00Z');
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional ISO date', () => {
      const { error } = date.optionalIsoDate.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Future Date Validation', () => {
    it('should validate future date', () => {
      const futureDate = new Date();
      futureDate.setDate(futureDate.getDate() + 1);
      const { error } = date.futureDate.validate(futureDate);
      expect(error).toBeUndefined();
    });

    it('should reject past date', () => {
      const pastDate = new Date();
      pastDate.setDate(pastDate.getDate() - 1);
      const { error } = date.futureDate.validate(pastDate);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject current date', () => {
      const { error } = date.futureDate.validate(new Date());
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should allow optional future date', () => {
      const futureDate = new Date();
      futureDate.setDate(futureDate.getDate() + 1);
      const { error } = date.optionalFutureDate.validate(futureDate);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional future date', () => {
      const { error } = date.optionalFutureDate.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Past Date Validation', () => {
    it('should validate past date', () => {
      const pastDate = new Date();
      pastDate.setDate(pastDate.getDate() - 1);
      const { error } = date.pastDate.validate(pastDate);
      expect(error).toBeUndefined();
    });

    it('should reject future date', () => {
      const futureDate = new Date();
      futureDate.setDate(futureDate.getDate() + 1);
      const { error } = date.pastDate.validate(futureDate);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should reject current date', () => {
      const { error } = date.pastDate.validate(new Date());
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should allow optional past date', () => {
      const pastDate = new Date();
      pastDate.setDate(pastDate.getDate() - 1);
      const { error } = date.optionalPastDate.validate(pastDate);
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional past date', () => {
      const { error } = date.optionalPastDate.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Date Range Validation', () => {
    const minDate = new Date('2024-01-01');
    const maxDate = new Date('2024-12-31');
    const dateRange = date.dateRange(minDate, maxDate);
    const optionalDateRange = date.optionalDateRange(minDate, maxDate);

    it('should validate date within range', () => {
      const { error } = dateRange.validate(new Date('2024-06-15'));
      expect(error).toBeUndefined();
    });

    it('should validate minimum date', () => {
      const { error } = dateRange.validate(minDate);
      expect(error).toBeUndefined();
    });

    it('should validate maximum date', () => {
      const { error } = dateRange.validate(maxDate);
      expect(error).toBeUndefined();
    });

    it('should reject date before range', () => {
      const { error } = dateRange.validate(new Date('2023-12-31'));
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject date after range', () => {
      const { error } = dateRange.validate(new Date('2025-01-01'));
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should allow optional date range', () => {
      const { error } = optionalDateRange.validate(new Date('2024-06-15'));
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional date range', () => {
      const { error } = optionalDateRange.validate(undefined);
      expect(error).toBeUndefined();
    });
  });

  describe('Timestamp Validation', () => {
    it('should validate valid timestamp', () => {
      const { error } = date.timestamp.validate(new Date().getTime());
      expect(error).toBeUndefined();
    });

    it('should reject invalid timestamp', () => {
      const { error } = date.timestamp.validate('invalid');
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('timestamp');
    });

    it('should allow optional timestamp', () => {
      const { error } = date.optionalTimestamp.validate(new Date().getTime());
      expect(error).toBeUndefined();
    });

    it('should allow undefined for optional timestamp', () => {
      const { error } = date.optionalTimestamp.validate(undefined);
      expect(error).toBeUndefined();
    });
  });
}); 