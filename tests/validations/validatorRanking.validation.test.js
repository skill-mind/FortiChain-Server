const { validatorRankingSchema, updateValidatorRankingSchema } = require('../../validations/validatorRanking.validation');

describe('Validator Ranking Validation Schemas', () => {
  describe('Validator Ranking Schema', () => {
    const validRanking = {
      validatorId: '123e4567-e89b-12d3-a456-426614174000',
      rank: 1,
      score: 95.5,
      notes: 'Excellent performance in security audits'
    };

    it('should validate valid ranking data', () => {
      const { error } = validatorRankingSchema.validate(validRanking);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = validatorRankingSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate validator ID', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        validatorId: '123e4567-e89b-12d3-a456-426614174000'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid validator ID', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        validatorId: 'invalid-uuid'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate rank', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        rank: 5
      });
      expect(error).toBeUndefined();
    });

    it('should reject non-positive rank', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        rank: 0
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject non-integer rank', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        rank: 1.5
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('integer');
    });

    it('should validate score', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        score: 85.5
      });
      expect(error).toBeUndefined();
    });

    it('should reject score below 0', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        score: -1
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject score above 100', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        score: 101
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should validate notes', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        notes: 'Valid notes within the length limit'
      });
      expect(error).toBeUndefined();
    });

    it('should reject notes exceeding length limit', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        notes: 'a'.repeat(501)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should allow empty notes', () => {
      const { error } = validatorRankingSchema.validate({
        ...validRanking,
        notes: ''
      });
      expect(error).toBeUndefined();
    });
  });

  describe('Update Validator Ranking Schema', () => {
    const validUpdate = {
      validatorId: '123e4567-e89b-12d3-a456-426614174000',
      rank: 2,
      score: 90.5,
      notes: 'Updated performance notes'
    };

    it('should validate valid update data', () => {
      const { error } = updateValidatorRankingSchema.validate(validUpdate);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { validatorId: '123e4567-e89b-12d3-a456-426614174000' },
        { rank: 3 },
        { score: 88.5 },
        { notes: 'Updated notes' }
      ];

      partialUpdates.forEach(update => {
        const { error } = updateValidatorRankingSchema.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = updateValidatorRankingSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate updated validator ID', () => {
      const { error } = updateValidatorRankingSchema.validate({
        validatorId: '123e4567-e89b-12d3-a456-426614174000'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid updated validator ID', () => {
      const { error } = updateValidatorRankingSchema.validate({
        validatorId: 'invalid-uuid'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated rank', () => {
      const { error } = updateValidatorRankingSchema.validate({
        rank: 4
      });
      expect(error).toBeUndefined();
    });

    it('should reject non-positive updated rank', () => {
      const { error } = updateValidatorRankingSchema.validate({
        rank: 0
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject non-integer updated rank', () => {
      const { error } = updateValidatorRankingSchema.validate({
        rank: 2.5
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('integer');
    });

    it('should validate updated score', () => {
      const { error } = updateValidatorRankingSchema.validate({
        score: 92.5
      });
      expect(error).toBeUndefined();
    });

    it('should reject updated score below 0', () => {
      const { error } = updateValidatorRankingSchema.validate({
        score: -1
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject updated score above 100', () => {
      const { error } = updateValidatorRankingSchema.validate({
        score: 101
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should validate updated notes', () => {
      const { error } = updateValidatorRankingSchema.validate({
        notes: 'Valid updated notes within the length limit'
      });
      expect(error).toBeUndefined();
    });

    it('should reject updated notes exceeding length limit', () => {
      const { error } = updateValidatorRankingSchema.validate({
        notes: 'a'.repeat(501)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should allow empty updated notes', () => {
      const { error } = updateValidatorRankingSchema.validate({
        notes: ''
      });
      expect(error).toBeUndefined();
    });
  });
}); 