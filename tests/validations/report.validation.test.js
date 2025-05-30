const { reportSchema, updateReportSchema } = require('../../validations/report.validation');

describe('Report Validation Schemas', () => {
  describe('Report Schema', () => {
    const validReport = {
      projectName: 'Test Project',
      projectStatus: 'active',
      submittedBy: '123e4567-e89b-12d3-a456-426614174000',
      vulnerabilityTitle: 'Reentrancy Vulnerability',
      description: 'A detailed description of the reentrancy vulnerability found in the smart contract.',
      severity: 'high',
      proofOfConcept: 'Steps to reproduce the vulnerability...',
      status: 'pending',
      cvssScore: 8.5,
      bounty: 1000,
      reviewerReward: 100,
      mitigation: 'Suggested mitigation steps...'
    };

    it('should validate valid report data', () => {
      const { error } = reportSchema.validate(validReport);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = reportSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate project name', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        projectName: 'Valid Project Name'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short project name', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        projectName: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate project status', () => {
      const validStatuses = ['active', 'inactive', 'completed'];
      validStatuses.forEach(status => {
        const { error } = reportSchema.validate({
          ...validReport,
          projectStatus: status
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid project status', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        projectStatus: 'invalid_status'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate submitted by', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        submittedBy: '123e4567-e89b-12d3-a456-426614174000'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid submitted by', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        submittedBy: 'invalid-uuid'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate vulnerability title', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        vulnerabilityTitle: 'Valid Vulnerability Title'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short vulnerability title', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        vulnerabilityTitle: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate description', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        description: 'A valid description that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short description', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        description: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate severity', () => {
      const validSeverities = ['low', 'medium', 'high', 'critical'];
      validSeverities.forEach(severity => {
        const { error } = reportSchema.validate({
          ...validReport,
          severity
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid severity', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        severity: 'invalid_severity'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate proof of concept', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        proofOfConcept: 'A valid proof of concept that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short proof of concept', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        proofOfConcept: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate status', () => {
      const validStatuses = ['pending', 'in_review', 'resolved', 'rejected'];
      validStatuses.forEach(status => {
        const { error } = reportSchema.validate({
          ...validReport,
          status
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid status', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        status: 'invalid_status'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate CVSS score', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        cvssScore: 7.5
      });
      expect(error).toBeUndefined();
    });

    it('should reject CVSS score below 0', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        cvssScore: -1
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject CVSS score above 10', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        cvssScore: 10.1
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should validate bounty', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        bounty: 500
      });
      expect(error).toBeUndefined();
    });

    it('should reject negative bounty', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        bounty: -100
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should validate reviewer reward', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        reviewerReward: 50
      });
      expect(error).toBeUndefined();
    });

    it('should reject negative reviewer reward', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        reviewerReward: -10
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should validate mitigation', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        mitigation: 'A valid mitigation that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short mitigation', () => {
      const { error } = reportSchema.validate({
        ...validReport,
        mitigation: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });
  });

  describe('Update Report Schema', () => {
    const validUpdate = {
      projectName: 'Updated Project',
      projectStatus: 'inactive',
      submittedBy: '123e4567-e89b-12d3-a456-426614174000',
      vulnerabilityTitle: 'Updated Vulnerability',
      description: 'Updated description of the vulnerability.',
      severity: 'critical',
      proofOfConcept: 'Updated proof of concept...',
      status: 'in_review',
      cvssScore: 9.5,
      bounty: 2000,
      reviewerReward: 200,
      mitigation: 'Updated mitigation steps...'
    };

    it('should validate valid update data', () => {
      const { error } = updateReportSchema.validate(validUpdate);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { projectName: 'Updated Project' },
        { projectStatus: 'inactive' },
        { vulnerabilityTitle: 'Updated Vulnerability' },
        { description: 'Updated description' },
        { severity: 'critical' },
        { status: 'in_review' },
        { cvssScore: 9.5 },
        { bounty: 2000 },
        { reviewerReward: 200 },
        { mitigation: 'Updated mitigation' }
      ];

      partialUpdates.forEach(update => {
        const { error } = updateReportSchema.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = updateReportSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate updated project name', () => {
      const { error } = updateReportSchema.validate({
        projectName: 'Valid Updated Project Name'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated project name', () => {
      const { error } = updateReportSchema.validate({
        projectName: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated project status', () => {
      const validStatuses = ['active', 'inactive', 'completed'];
      validStatuses.forEach(status => {
        const { error } = updateReportSchema.validate({
          projectStatus: status
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid updated project status', () => {
      const { error } = updateReportSchema.validate({
        projectStatus: 'invalid_status'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated submitted by', () => {
      const { error } = updateReportSchema.validate({
        submittedBy: '123e4567-e89b-12d3-a456-426614174000'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid updated submitted by', () => {
      const { error } = updateReportSchema.validate({
        submittedBy: 'invalid-uuid'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated vulnerability title', () => {
      const { error } = updateReportSchema.validate({
        vulnerabilityTitle: 'Valid Updated Vulnerability Title'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated vulnerability title', () => {
      const { error } = updateReportSchema.validate({
        vulnerabilityTitle: 'A'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated description', () => {
      const { error } = updateReportSchema.validate({
        description: 'A valid updated description that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated description', () => {
      const { error } = updateReportSchema.validate({
        description: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated severity', () => {
      const validSeverities = ['low', 'medium', 'high', 'critical'];
      validSeverities.forEach(severity => {
        const { error } = updateReportSchema.validate({
          severity
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid updated severity', () => {
      const { error } = updateReportSchema.validate({
        severity: 'invalid_severity'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated proof of concept', () => {
      const { error } = updateReportSchema.validate({
        proofOfConcept: 'A valid updated proof of concept that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated proof of concept', () => {
      const { error } = updateReportSchema.validate({
        proofOfConcept: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate updated status', () => {
      const validStatuses = ['pending', 'in_review', 'resolved', 'rejected'];
      validStatuses.forEach(status => {
        const { error } = updateReportSchema.validate({
          status
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid updated status', () => {
      const { error } = updateReportSchema.validate({
        status: 'invalid_status'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate updated CVSS score', () => {
      const { error } = updateReportSchema.validate({
        cvssScore: 8.5
      });
      expect(error).toBeUndefined();
    });

    it('should reject updated CVSS score below 0', () => {
      const { error } = updateReportSchema.validate({
        cvssScore: -1
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should reject updated CVSS score above 10', () => {
      const { error } = updateReportSchema.validate({
        cvssScore: 10.1
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });

    it('should validate updated bounty', () => {
      const { error } = updateReportSchema.validate({
        bounty: 1500
      });
      expect(error).toBeUndefined();
    });

    it('should reject negative updated bounty', () => {
      const { error } = updateReportSchema.validate({
        bounty: -100
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should validate updated reviewer reward', () => {
      const { error } = updateReportSchema.validate({
        reviewerReward: 150
      });
      expect(error).toBeUndefined();
    });

    it('should reject negative updated reviewer reward', () => {
      const { error } = updateReportSchema.validate({
        reviewerReward: -10
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('greater');
    });

    it('should validate updated mitigation', () => {
      const { error } = updateReportSchema.validate({
        mitigation: 'A valid updated mitigation that is long enough to meet the minimum length requirement'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short updated mitigation', () => {
      const { error } = updateReportSchema.validate({
        mitigation: 'Too short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });
  });
}); 