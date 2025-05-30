const { user, auth, profile, settings } = require('../../middlewares/validators');

describe('User Validation Schemas', () => {
  describe('User Schema', () => {
    const validUser = {
      id: '123e4567-e89b-12d3-a456-426614174000',
      email: 'test@example.com',
      password: 'password123',
      role: 'user',
      status: 'active'
    };

    it('should validate valid user data', () => {
      const { error } = user.validate(validUser);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = user.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate email format', () => {
      const { error } = user.validate({
        ...validUser,
        email: 'valid.email@example.com'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid email format', () => {
      const { error } = user.validate({
        ...validUser,
        email: 'invalid-email'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate password length', () => {
      const { error } = user.validate({
        ...validUser,
        password: 'password123'
      });
      expect(error).toBeUndefined();
    });

    it('should reject short password', () => {
      const { error } = user.validate({
        ...validUser,
        password: 'short'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate role', () => {
      const validRoles = ['user', 'admin', 'moderator'];
      validRoles.forEach(role => {
        const { error } = user.validate({
          ...validUser,
          role
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid role', () => {
      const { error } = user.validate({
        ...validUser,
        role: 'invalid_role'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate status', () => {
      const validStatuses = ['active', 'suspended', 'inactive'];
      validStatuses.forEach(status => {
        const { error } = user.validate({
          ...validUser,
          status
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid status', () => {
      const { error } = user.validate({
        ...validUser,
        status: 'invalid_status'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });

  describe('Authentication Schema', () => {
    describe('Login Schema', () => {
      const validLogin = {
        email: 'test@example.com',
        password: 'password123'
      };

      it('should validate valid login data', () => {
        const { error } = auth.login.validate(validLogin);
        expect(error).toBeUndefined();
      });

      it('should reject missing required fields', () => {
        const { error } = auth.login.validate({});
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('required');
      });

      it('should validate email format', () => {
        const { error } = auth.login.validate({
          ...validLogin,
          email: 'valid.email@example.com'
        });
        expect(error).toBeUndefined();
      });

      it('should reject invalid email format', () => {
        const { error } = auth.login.validate({
          ...validLogin,
          email: 'invalid-email'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });
    });

    describe('Register Schema', () => {
      const validRegister = {
        email: 'test@example.com',
        password: 'password123',
        confirmPassword: 'password123'
      };

      it('should validate valid registration data', () => {
        const { error } = auth.register.validate(validRegister);
        expect(error).toBeUndefined();
      });

      it('should reject missing required fields', () => {
        const { error } = auth.register.validate({});
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('required');
      });

      it('should validate matching passwords', () => {
        const { error } = auth.register.validate({
          ...validRegister,
          confirmPassword: 'password123'
        });
        expect(error).toBeUndefined();
      });

      it('should reject non-matching passwords', () => {
        const { error } = auth.register.validate({
          ...validRegister,
          confirmPassword: 'different'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('match');
      });
    });

    describe('Reset Password Schema', () => {
      const validReset = {
        email: 'test@example.com'
      };

      it('should validate valid reset data', () => {
        const { error } = auth.resetPassword.validate(validReset);
        expect(error).toBeUndefined();
      });

      it('should reject missing required fields', () => {
        const { error } = auth.resetPassword.validate({});
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('required');
      });

      it('should validate email format', () => {
        const { error } = auth.resetPassword.validate({
          email: 'valid.email@example.com'
        });
        expect(error).toBeUndefined();
      });

      it('should reject invalid email format', () => {
        const { error } = auth.resetPassword.validate({
          email: 'invalid-email'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });
    });

    describe('Change Password Schema', () => {
      const validChange = {
        currentPassword: 'oldpassword123',
        newPassword: 'newpassword123',
        confirmPassword: 'newpassword123'
      };

      it('should validate valid change data', () => {
        const { error } = auth.changePassword.validate(validChange);
        expect(error).toBeUndefined();
      });

      it('should reject missing required fields', () => {
        const { error } = auth.changePassword.validate({});
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('required');
      });

      it('should validate matching new passwords', () => {
        const { error } = auth.changePassword.validate({
          ...validChange,
          confirmPassword: 'newpassword123'
        });
        expect(error).toBeUndefined();
      });

      it('should reject non-matching new passwords', () => {
        const { error } = auth.changePassword.validate({
          ...validChange,
          confirmPassword: 'different'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('match');
      });
    });
  });

  describe('Profile Schema', () => {
    const validProfile = {
      name: 'John Doe',
      bio: 'Software developer with 5 years of experience',
      avatar: 'https://example.com/avatar.jpg',
      socialLinks: {
        twitter: 'https://twitter.com/johndoe',
        github: 'https://github.com/johndoe',
        linkedin: 'https://linkedin.com/in/johndoe'
      }
    };

    it('should validate valid profile data', () => {
      const { error } = profile.update.validate(validProfile);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { name: 'John Doe' },
        { bio: 'New bio' },
        { avatar: 'https://example.com/new-avatar.jpg' },
        { socialLinks: { twitter: 'https://twitter.com/johndoe' } }
      ];

      partialUpdates.forEach(update => {
        const { error } = profile.update.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = profile.update.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate name length', () => {
      const { error } = profile.update.validate({
        name: 'John Doe'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too short name', () => {
      const { error } = profile.update.validate({
        name: 'J'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate bio length', () => {
      const { error } = profile.update.validate({
        bio: 'A valid bio'
      });
      expect(error).toBeUndefined();
    });

    it('should reject too long bio', () => {
      const { error } = profile.update.validate({
        bio: 'a'.repeat(501)
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('length');
    });

    it('should validate avatar URL', () => {
      const { error } = profile.update.validate({
        avatar: 'https://example.com/avatar.jpg'
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid avatar URL', () => {
      const { error } = profile.update.validate({
        avatar: 'not-a-url'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate social links', () => {
      const { error } = profile.update.validate({
        socialLinks: {
          twitter: 'https://twitter.com/johndoe',
          github: 'https://github.com/johndoe',
          linkedin: 'https://linkedin.com/in/johndoe'
        }
      });
      expect(error).toBeUndefined();
    });

    it('should reject invalid social links', () => {
      const { error } = profile.update.validate({
        socialLinks: {
          twitter: 'not-a-url',
          github: 'invalid-github',
          linkedin: 'invalid-linkedin'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });

  describe('Settings Schema', () => {
    const validSettings = {
      notifications: {
        email: true,
        push: false,
        sms: true
      },
      privacy: {
        profileVisibility: 'public',
        activityVisibility: 'connections'
      },
      preferences: {
        language: 'en',
        timezone: 'UTC',
        theme: 'dark'
      }
    };

    it('should validate valid settings data', () => {
      const { error } = settings.update.validate(validSettings);
      expect(error).toBeUndefined();
    });

    it('should allow partial updates', () => {
      const partialUpdates = [
        { notifications: { email: true } },
        { privacy: { profileVisibility: 'private' } },
        { preferences: { language: 'es' } }
      ];

      partialUpdates.forEach(update => {
        const { error } = settings.update.validate(update);
        expect(error).toBeUndefined();
      });
    });

    it('should reject empty update object', () => {
      const { error } = settings.update.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('at least one');
    });

    it('should validate notification settings', () => {
      const { error } = settings.update.validate({
        notifications: {
          email: true,
          push: false,
          sms: true
        }
      });
      expect(error).toBeUndefined();
    });

    it('should validate privacy settings', () => {
      const validVisibilities = ['public', 'private', 'connections'];
      validVisibilities.forEach(visibility => {
        const { error } = settings.update.validate({
          privacy: {
            profileVisibility: visibility,
            activityVisibility: visibility
          }
        });
        expect(error).toBeUndefined();
      });
    });

    it('should reject invalid privacy settings', () => {
      const { error } = settings.update.validate({
        privacy: {
          profileVisibility: 'invalid',
          activityVisibility: 'invalid'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate preferences', () => {
      const validLanguages = ['en', 'es', 'fr', 'de'];
      const validThemes = ['light', 'dark', 'system'];

      validLanguages.forEach(language => {
        validThemes.forEach(theme => {
          const { error } = settings.update.validate({
            preferences: {
              language,
              theme
            }
          });
          expect(error).toBeUndefined();
        });
      });
    });

    it('should reject invalid preferences', () => {
      const { error } = settings.update.validate({
        preferences: {
          language: 'invalid',
          theme: 'invalid'
        }
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });
  });
}); 