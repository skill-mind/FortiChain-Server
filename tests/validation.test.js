const {
  validateWallet,
  validateProject,
  validateHelpRequest,
  validateDocument,
  validateValidatorRanking,
  validateTip,
  validateReport,
  validatePayout,
  validateSupportReply
} = require('../utils/validators');

describe('Validation Rules', () => {
  describe('Wallet Validation', () => {
    it('should validate a valid Ethereum address', () => {
      const result = validateWallet({
        address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
        role: 'researcher'
      });
      expect(result.error).toBeUndefined();
    });

    it('should validate a valid Starknet address', () => {
      const result = validateWallet({
        address: '0x057d35a858fc7a5238b9339d640648bb2363cddd729deb357d035d6f27c2d476',
        role: 'validator'
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject an invalid address', () => {
      const result = validateWallet({
        address: 'invalid-address',
        role: 'researcher'
      });
      expect(result.error).toBeDefined();
    });

    it('should reject an invalid role', () => {
      const result = validateWallet({
        address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
        role: 'invalid_role'
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Project Validation', () => {
    it('should validate a valid project', () => {
      const result = validateProject({
        name: 'Test Project',
        description: 'A test project description that is long enough',
        category: 'Security',
        smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
        contactInfo: 'test@example.com',
        repositoryHost: 'github.com',
        repositoryName: 'test-repo',
        repositoryLink: 'https://github.com/test/test-repo',
        token: 'ETH',
        bountyCurrency: 'USD',
        dateOfExpiry: new Date(Date.now() + 86400000).toISOString(),
        allocatedBounty: 1000
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a project with invalid email', () => {
      const result = validateProject({
        name: 'Test Project',
        description: 'A test project description',
        category: 'Security',
        smartContractAddress: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
        contactInfo: 'invalid-email',
        repositoryHost: 'github.com',
        repositoryName: 'test-repo',
        repositoryLink: 'https://github.com/test/test-repo',
        token: 'ETH',
        bountyCurrency: 'USD',
        dateOfExpiry: new Date(Date.now() + 86400000).toISOString(),
        allocatedBounty: 1000
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Help Request Validation', () => {
    it('should validate a valid help request', () => {
      const result = validateHelpRequest({
        email: 'test@example.com',
        subject: 'Test Subject',
        message: 'This is a test message that is long enough to be valid.'
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a help request with short message', () => {
      const result = validateHelpRequest({
        email: 'test@example.com',
        subject: 'Test Subject',
        message: 'Too short'
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Document Validation', () => {
    it('should validate a valid document', () => {
      const result = validateDocument({
        document: {
          mimetype: 'application/pdf',
          size: 1024 * 1024 // 1MB
        }
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a document that is too large', () => {
      const result = validateDocument({
        document: {
          mimetype: 'application/pdf',
          size: 6 * 1024 * 1024 // 6MB
        }
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Validator Ranking Validation', () => {
    it('should validate a valid ranking', () => {
      const result = validateValidatorRanking({
        validatorId: '123e4567-e89b-12d3-a456-426614174000',
        rank: 1,
        score: 95.5,
        notes: 'Excellent validator'
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a ranking with invalid score', () => {
      const result = validateValidatorRanking({
        validatorId: '123e4567-e89b-12d3-a456-426614174000',
        rank: 1,
        score: 150, // Invalid score > 100
        notes: 'Excellent validator'
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Tip Validation', () => {
    it('should validate a valid tip', () => {
      const result = validateTip({
        title: 'Test Tip',
        content: 'This is a test tip with sufficient content length.',
        category: 'Security'
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a tip with short title', () => {
      const result = validateTip({
        title: 'Te',
        content: 'This is a test tip with sufficient content length.',
        category: 'Security'
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Report Validation', () => {
    it('should validate a valid report', () => {
      const result = validateReport({
        projectName: 'Test Project',
        projectStatus: 'Ongoing',
        submittedBy: 'John Doe',
        vulnerabilityTitle: 'SQL Injection',
        description: 'Detailed description of the vulnerability',
        severity: 'High',
        proofOfConcept: ['https://example.com/poc1.png'],
        status: 'pending'
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a report with invalid severity', () => {
      const result = validateReport({
        projectName: 'Test Project',
        projectStatus: 'Ongoing',
        submittedBy: 'John Doe',
        vulnerabilityTitle: 'SQL Injection',
        description: 'Detailed description of the vulnerability',
        severity: 'Invalid',
        proofOfConcept: ['https://example.com/poc1.png'],
        status: 'pending'
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Payout Validation', () => {
    it('should validate a valid payout', () => {
      const result = validatePayout({
        userId: '123e4567-e89b-12d3-a456-426614174000',
        amount: 100.50,
        currency: 'USD',
        fee: 5.00
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject a payout with negative amount', () => {
      const result = validatePayout({
        userId: '123e4567-e89b-12d3-a456-426614174000',
        amount: -100.50,
        currency: 'USD',
        fee: 5.00
      });
      expect(result.error).toBeDefined();
    });
  });

  describe('Support Reply Validation', () => {
    it('should validate a valid support reply', () => {
      const result = validateSupportReply({
        message: 'This is a valid support reply message.'
      });
      expect(result.error).toBeUndefined();
    });

    it('should reject an empty support reply', () => {
      const result = validateSupportReply({
        message: ''
      });
      expect(result.error).toBeDefined();
    });
  });
}); 