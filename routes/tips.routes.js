// routes/tipRoutes.js
const express = require('express');
const router = express.Router();
const tipController = require('../controllers/tips.controller');
const { check } = require('express-validator');
const authMiddleware = require('../middlewares/handleValidationErr');


const tipValidationRules = [
  check('title').isString().isLength({ min: 3, max: 100 }),
  check('content').isString().isLength({ min: 10, max: 5000 }),
  check('category').isString().notEmpty()
];


router.get('/', tipController.getTips);
router.get('/:id', tipController.getTip);

router.use(authMiddleware);

router.post('/', tipValidationRules, tipController.createTip);
router.put('/:id', tipValidationRules, tipController.updateTip);
router.delete('/:id', tipController.deleteTip);


router.patch('/:id/best-model', 
  [authMiddleware, check('isBestModel').isBoolean()],
  tipController.markAsBestModel
);

module.exports = router;