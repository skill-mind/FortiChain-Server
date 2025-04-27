const { body, param } = require("express-validator");

module.exports = {
  validateIdParam: [
    param("id", "Invalid ID format").isMongoId()
  ],

  validateReplyMessage: [
    body("message", "Message is required").notEmpty()
  ]
};
