const { validateRequest, validateFile, validateParams, validateQuery } = require('../../middlewares/validationMiddleware');
const Joi = require('joi');

describe('Validation Middleware', () => {
  let mockReq;
  let mockRes;
  let nextFunction;

  beforeEach(() => {
    mockReq = {
      body: {},
      files: {},
      params: {},
      query: {},
      path: '/test',
      method: 'POST'
    };
    mockRes = {
      status: jest.fn().mockReturnThis(),
      json: jest.fn()
    };
    nextFunction = jest.fn();
  });

  describe('validateRequest', () => {
    const testSchema = Joi.object({
      name: Joi.string().required(),
      age: Joi.number().min(0).max(120).required()
    });

    it('should pass validation for valid request body', () => {
      mockReq.body = { name: 'John', age: 25 };
      validateRequest(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockRes.status).not.toHaveBeenCalled();
    });

    it('should fail validation for missing required fields', () => {
      mockReq.body = { name: 'John' };
      validateRequest(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'Validation failed',
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: 'age',
            message: expect.any(String)
          })
        ])
      });
    });

    it('should fail validation for invalid data types', () => {
      mockReq.body = { name: 123, age: 'invalid' };
      validateRequest(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'Validation failed',
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: 'name',
            message: expect.any(String)
          }),
          expect.objectContaining({
            field: 'age',
            message: expect.any(String)
          })
        ])
      });
    });

    it('should strip unknown fields', () => {
      mockReq.body = { name: 'John', age: 25, extra: 'field' };
      validateRequest(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockReq.body).toEqual({ name: 'John', age: 25 });
    });
  });

  describe('validateFile', () => {
    const testSchema = Joi.object({
      document: Joi.object({
        mimetype: Joi.string().valid('application/pdf').required(),
        size: Joi.number().max(5 * 1024 * 1024).required()
      }).required()
    });

    it('should pass validation for valid file', () => {
      mockReq.files = {
        document: {
          mimetype: 'application/pdf',
          size: 1024 * 1024
        }
      };
      validateFile(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockRes.status).not.toHaveBeenCalled();
    });

    it('should pass when no files are present', () => {
      mockReq.files = undefined;
      validateFile(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockRes.status).not.toHaveBeenCalled();
    });

    it('should fail validation for invalid file type', () => {
      mockReq.files = {
        document: {
          mimetype: 'image/jpeg',
          size: 1024 * 1024
        }
      };
      validateFile(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'File validation failed',
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: 'document.mimetype',
            message: expect.any(String)
          })
        ])
      });
    });

    it('should fail validation for file too large', () => {
      mockReq.files = {
        document: {
          mimetype: 'application/pdf',
          size: 10 * 1024 * 1024
        }
      };
      validateFile(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'File validation failed',
        errors: expect.arrayContaining([
          expect.objectContaining({
            field: 'document.size',
            message: expect.any(String)
          })
        ])
      });
    });
  });

  describe('validateParams', () => {
    const testSchema = Joi.object({
      id: Joi.string().uuid().required()
    });

    it('should pass validation for valid UUID', () => {
      mockReq.params = { id: '123e4567-e89b-12d3-a456-426614174000' };
      validateParams(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockRes.status).not.toHaveBeenCalled();
    });

    it('should fail validation for invalid UUID', () => {
      mockReq.params = { id: 'invalid-uuid' };
      validateParams(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'Invalid URL parameters',
        errors: expect.arrayContaining([
          expect.objectContaining({
            param: 'id',
            message: expect.any(String)
          })
        ])
      });
    });

    it('should fail validation for missing required parameter', () => {
      mockReq.params = {};
      validateParams(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'Invalid URL parameters',
        errors: expect.arrayContaining([
          expect.objectContaining({
            param: 'id',
            message: expect.any(String)
          })
        ])
      });
    });
  });

  describe('validateQuery', () => {
    const testSchema = Joi.object({
      page: Joi.number().min(1).required(),
      limit: Joi.number().min(1).max(100).required()
    });

    it('should pass validation for valid query parameters', () => {
      mockReq.query = { page: 1, limit: 10 };
      validateQuery(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockRes.status).not.toHaveBeenCalled();
    });

    it('should fail validation for invalid data types', () => {
      mockReq.query = { page: 'invalid', limit: 'invalid' };
      validateQuery(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'Invalid query parameters',
        errors: expect.arrayContaining([
          expect.objectContaining({
            param: 'page',
            message: expect.any(String)
          }),
          expect.objectContaining({
            param: 'limit',
            message: expect.any(String)
          })
        ])
      });
    });

    it('should fail validation for out of range values', () => {
      mockReq.query = { page: 0, limit: 200 };
      validateQuery(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).not.toHaveBeenCalled();
      expect(mockRes.status).toHaveBeenCalledWith(400);
      expect(mockRes.json).toHaveBeenCalledWith({
        success: false,
        message: 'Invalid query parameters',
        errors: expect.arrayContaining([
          expect.objectContaining({
            param: 'page',
            message: expect.any(String)
          }),
          expect.objectContaining({
            param: 'limit',
            message: expect.any(String)
          })
        ])
      });
    });

    it('should strip unknown query parameters', () => {
      mockReq.query = { page: 1, limit: 10, extra: 'param' };
      validateQuery(testSchema)(mockReq, mockRes, nextFunction);
      expect(nextFunction).toHaveBeenCalled();
      expect(mockReq.query).toEqual({ page: 1, limit: 10 });
    });
  });
}); 