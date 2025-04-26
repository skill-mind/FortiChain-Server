const db = require('../models');
const ValidatorRanking = db.ValidatorRanking;
const User = db.User;

exports.createRanking = async (req, res) => {
  try {
    const { validatorId, rank, score, notes } = req.body;
    const validator = await User.findByPk(validatorId);
    if (!validator) return res.status(404).json({ error: 'Validator not found.' });

    const ranking = await ValidatorRanking.create({ validatorId, rank, score, notes });
    res.status(201).json(ranking);
  } catch (err) {
    res.status(500).json({ error: 'Failed to create ranking.' });
  }
};

exports.getAllRankings = async (req, res) => {
  try {
    const { page = 1, limit = 20 } = req.query;
    const offset = (page - 1) * limit;
    const rankings = await ValidatorRanking.findAndCountAll({
      include: [{ model: User, as: 'validator' }],
      limit: parseInt(limit),
      offset: parseInt(offset),
      order: [['rank', 'ASC']],
    });
    res.json(rankings);
  } catch (err) {
    console.error('getAllRankings error:', err);
    res.status(500).json({ error: 'Failed to fetch rankings.', details: err.message });
  }
};

exports.getRankingById = async (req, res) => {
  try {
    const ranking = await ValidatorRanking.findByPk(req.params.id, {
      include: [{ model: User, as: 'validator' }],
    });
    if (!ranking) return res.status(404).json({ error: 'Ranking not found.' });
    res.json(ranking);
  } catch (err) {
    console.error('getRankingById error:', err);
    res.status(500).json({ error: 'Failed to fetch ranking.', details: err.message });
  }
};

exports.updateRanking = async (req, res) => {
  try {
    const { rank, score, notes } = req.body;
    const ranking = await ValidatorRanking.findByPk(req.params.id);
    if (!ranking) return res.status(404).json({ error: 'Ranking not found.' });
    if (rank !== undefined) ranking.rank = rank;
    if (score !== undefined) ranking.score = score;
    if (notes !== undefined) ranking.notes = notes;
    await ranking.save();
    res.json(ranking);
  } catch (err) {
    res.status(500).json({ error: 'Failed to update ranking.' });
  }
};

exports.deleteRanking = async (req, res) => {
  try {
    const ranking = await ValidatorRanking.findByPk(req.params.id);
    if (!ranking) return res.status(404).json({ error: 'Ranking not found.' });
    await ranking.destroy();
    res.json({ message: 'Ranking deleted successfully.' });
  } catch (err) {
    res.status(500).json({ error: 'Failed to delete ranking.' });
  }
}; 