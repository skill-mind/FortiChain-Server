const { walletSchema, updateWalletSchema } = require('../../validations/wallet.validation');

describe('Wallet Validation Schemas', () => {
  describe('Wallet Schema', () => {
    const validWallet = {
      address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
      role: 'validator'
    };

    it('should validate valid wallet data', () => {
      const { error } = walletSchema.validate(validWallet);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = walletSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate Ethereum address', () => {
      const { error } = walletSchema.validate({
        ...validWallet,
        address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e'
      });
      expect(error).toBeUndefined();
    });

    it('should validate Starknet address', () => {
      const { error } = walletSchema.validate({
        ...validWallet,
        address: '0x1234567890123456789012345678901234567890123456789012345678901234'
      });
      expect(error).toBeUndefined();
    });

    it('should validate Stellar address', () => {
      const { error } = walletSchema.validate({
        ...validWallet,
        address: 'GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid address format', () => {
      const { error } = walletSchema.validate({
        ...validWallet,
        address: 'invalid-address'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate valid role', () => {
      const validRoles = ['validator', 'project_owner', 'user'];
      validRoles.forEach(role => {
        const { error } = walletSchema.validate({
          ...validWallet,
          role
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid role', () => {
      const { error } = walletSchema.validate({
        ...validWallet,
        role: 'invalid_role'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });

  describe('Update Wallet Schema', () => {
    const validUpdate = {
      address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e',
      role: 'validator'
    };

    it('should validate valid update data', () => {
      const { error } = updateWalletSchema.validate(validUpdate);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e' },
        { role: 'validator' }
      ];

      partialUpdates.forEach(update => {
        const { error } = updateWalletSchema.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = updateWalletSchema.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate Ethereum address in update', () => {
      const { error } = updateWalletSchema.validate({
        address: '0x742d35Cc6634C0532925a3b844Bc454e4438f44e'
      });
      expect(error).toBeUndefined();
    });

    it('should validate Starknet address in update', () => {
      const { error } = updateWalletSchema.validate({
        address: '0x1234567890123456789012345678901234567890123456789012345678901234'
      });
      expect(error).toBeUndefined();
    });

    it('should validate Stellar address in update', () => {
      const { error } = updateWalletSchema.validate({
        address: 'GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid address format in update', () => {
      const { error } = updateWalletSchema.validate({
        address: 'invalid-address'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate valid role in update', () => {
      const validRoles = ['validator', 'project_owner', 'user'];
      validRoles.forEach(role => {
        const { error } = updateWalletSchema.validate({ role });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid role in update', () => {
      const { error } = updateWalletSchema.validate({
        role: 'invalid_role'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });
}); 