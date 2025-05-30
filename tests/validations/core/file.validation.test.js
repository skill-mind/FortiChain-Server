const { file } = require('../../../validations/core');

describe('File Validation Schemas', () => {
  describe('File Object Validation', () => {
    const validFile = {
      mimetype: 'image/jpeg',
      size: 1024 * 1024, // 1MB
      name: 'test.jpg'
    };

    it('should validate valid file object', () => {
      const { error } = file.fileObject.validate(validFile);
      expect(error).toBeUndefined();
    });

    it('should reject missing required fields', () => {
      const { error } = file.fileObject.validate({});
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('required');
    });

    it('should validate file with mimetype constraints', () => {
      const schema = file.fileObject.keys({
        mimetype: file.fileObject.extract('mimetype').valid('image/jpeg', 'image/png')
      });
      const { error } = schema.validate(validFile);
      expect(error).toBeUndefined();
    });

    it('should reject file with invalid mimetype', () => {
      const schema = file.fileObject.keys({
        mimetype: file.fileObject.extract('mimetype').valid('image/jpeg', 'image/png')
      });
      const { error } = schema.validate({
        ...validFile,
        mimetype: 'application/pdf'
      });
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('valid');
    });

    it('should validate file with size constraints', () => {
      const schema = file.fileObject.keys({
        size: file.fileObject.extract('size').max(2 * 1024 * 1024) // 2MB
      });
      const { error } = schema.validate(validFile);
      expect(error).toBeUndefined();
    });

    it('should reject file exceeding size limit', () => {
      const schema = file.fileObject.keys({
        size: file.fileObject.extract('size').max(512 * 1024) // 512KB
      });
      const { error } = schema.validate(validFile);
      expect(error).toBeDefined();
      expect(error.details[0].message).toContain('less');
    });
  });

  describe('Multiple Files Validation', () => {
    const validFiles = [
      {
        mimetype: 'image/jpeg',
        size: 1024 * 1024,
        name: 'test1.jpg'
      },
      {
        mimetype: 'image/png',
        size: 512 * 1024,
        name: 'test2.png'
      }
    ];

    it('should validate array of valid files', () => {
      const { error } = file.multipleFiles.validate(validFiles);
      expect(error).toBeUndefined();
    });

    it('should reject array with invalid file', () => {
      const invalidFiles = [
        ...validFiles,
        {
          mimetype: 'invalid',
          size: 1024,
          name: 'test3.txt'
        }
      ];
      const { error } = file.multipleFiles.validate(invalidFiles);
      expect(error).toBeDefined();
    });

    it('should validate files with mimetype constraints', () => {
      const schema = file.multipleFiles.items(
        file.fileObject.keys({
          mimetype: file.fileObject.extract('mimetype').valid('image/jpeg', 'image/png')
        })
      );
      const { error } = schema.validate(validFiles);
      expect(error).toBeUndefined();
    });

    it('should reject files with invalid mimetype', () => {
      const schema = file.multipleFiles.items(
        file.fileObject.keys({
          mimetype: file.fileObject.extract('mimetype').valid('image/jpeg', 'image/png')
        })
      );
      const invalidFiles = [
        ...validFiles,
        {
          mimetype: 'application/pdf',
          size: 1024,
          name: 'test3.pdf'
        }
      ];
      const { error } = schema.validate(invalidFiles);
      expect(error).toBeDefined();
    });
  });

  describe('Common File Type Validations', () => {
    describe('Image Validation', () => {
      const validImage = {
        mimetype: 'image/jpeg',
        size: 1024 * 1024,
        name: 'test.jpg'
      };

      it('should validate valid image file', () => {
        const { error } = file.image.validate(validImage);
        expect(error).toBeUndefined();
      });

      it('should reject non-image file', () => {
        const { error } = file.image.validate({
          ...validImage,
          mimetype: 'application/pdf'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });

      it('should reject image exceeding size limit', () => {
        const { error } = file.image.validate({
          ...validImage,
          size: 6 * 1024 * 1024 // 6MB
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('less');
      });
    });

    describe('PDF Validation', () => {
      const validPDF = {
        mimetype: 'application/pdf',
        size: 2 * 1024 * 1024,
        name: 'test.pdf'
      };

      it('should validate valid PDF file', () => {
        const { error } = file.pdf.validate(validPDF);
        expect(error).toBeUndefined();
      });

      it('should reject non-PDF file', () => {
        const { error } = file.pdf.validate({
          ...validPDF,
          mimetype: 'image/jpeg'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });

      it('should reject PDF exceeding size limit', () => {
        const { error } = file.pdf.validate({
          ...validPDF,
          size: 11 * 1024 * 1024 // 11MB
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('less');
      });
    });

    describe('Document Validation', () => {
      const validDocument = {
        mimetype: 'application/msword',
        size: 5 * 1024 * 1024,
        name: 'test.doc'
      };

      it('should validate valid document file', () => {
        const { error } = file.document.validate(validDocument);
        expect(error).toBeUndefined();
      });

      it('should reject non-document file', () => {
        const { error } = file.document.validate({
          ...validDocument,
          mimetype: 'image/png'
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('valid');
      });

      it('should reject document exceeding size limit', () => {
        const { error } = file.document.validate({
          ...validDocument,
          size: 16 * 1024 * 1024 // 16MB
        });
        expect(error).toBeDefined();
        expect(error.details[0].message).toContain('less');
      });
    });
  });
}); 