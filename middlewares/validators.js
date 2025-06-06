const Joi = require('joi');

module.exports = {
  // User validation schemas
  user: {
    id: Joi.string().uuid().required(),
    email: Joi.string().email().required(),
    password: Joi.string().min(8).required(),
    role: Joi.string().valid('user', 'admin', 'moderator').required(),
    status: Joi.string().valid('active', 'suspended', 'inactive').default('active')
  },

  // Authentication validation schemas
  auth: {
    login: Joi.object({
      email: Joi.string().email().required(),
      password: Joi.string().required()
    }),
    register: Joi.object({
      email: Joi.string().email().required(),
      password: Joi.string().min(8).required(),
      confirmPassword: Joi.string().valid(Joi.ref('password')).required()
    }),
    resetPassword: Joi.object({
      email: Joi.string().email().required()
    }),
    changePassword: Joi.object({
      currentPassword: Joi.string().required(),
      newPassword: Joi.string().min(8).required(),
      confirmPassword: Joi.string().valid(Joi.ref('newPassword')).required()
    })
  },

  // Profile validation schemas
  profile: {
    update: Joi.object({
      name: Joi.string().min(2).max(100),
      bio: Joi.string().max(500),
      avatar: Joi.string().uri(),
      socialLinks: Joi.object({
        twitter: Joi.string().uri(),
        github: Joi.string().uri(),
        linkedin: Joi.string().uri()
      })
    }).min(1)
  },

  // Settings validation schemas
  settings: {
    update: Joi.object({
      notifications: Joi.object({
        email: Joi.boolean(),
        push: Joi.boolean(),
        sms: Joi.boolean()
      }),
      privacy: Joi.object({
        profileVisibility: Joi.string().valid('public', 'private', 'connections'),
        activityVisibility: Joi.string().valid('public', 'private', 'connections')
      }),
      preferences: Joi.object({
        language: Joi.string().valid('en', 'es', 'fr', 'de'),
        timezone: Joi.string(),
        theme: Joi.string().valid('light', 'dark', 'system')
      })
    }).min(1)
  },

  // Common parameter validation schemas
  params: {
    id: Joi.string().uuid().required(),
    userId: Joi.string().uuid().required(),
    projectId: Joi.string().uuid().required(),
    reportId: Joi.string().uuid().required()
  },

  // Common query validation schemas
  query: {
    pagination: Joi.object({
      page: Joi.number().min(1).default(1),
      limit: Joi.number().min(1).max(100).default(10),
      sort: Joi.string().valid('asc', 'desc').default('desc'),
      orderBy: Joi.string()
    })
  }
};
