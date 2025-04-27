const express = require('express');
const router = express.Router();
const validatorRankingController = require('../controllers/validatorRanking.controller');

router.post('/', validatorRankingController.createRanking);

router.get('/', validatorRankingController.getAllRankings);

router.get('/:id', validatorRankingController.getRankingById);

router.put('/:id', validatorRankingController.updateRanking);

router.delete('/:id', validatorRankingController.deleteRanking);

module.exports = router; 