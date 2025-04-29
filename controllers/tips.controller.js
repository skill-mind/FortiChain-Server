// controllers/tipController.js
const  Tip  = require('../models/tips.models');
const { validationResult } = require('express-validator');

exports.createTip = async (req, res, next) => {
  try {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }

    const tip = await Tip.create({
      ...req.body,
      authorId: req.user.id
    });

    res.status(201).json(tip);
  } catch (error) {
    next(error);
  }
};

exports.getTips = async (req, res, next) => {
  try {
    const { category, bestModel, limit = 10, offset = 0 } = req.query;
    const where = {};
    
    if (category) where.category = category;
    if (bestModel) where.isBestModel = bestModel === 'true';

    const tips = await Tip.findAll({
      where,
      limit: parseInt(limit),
      offset: parseInt(offset),
      order: [['createdAt', 'DESC']],
      include: ['author']
    });

    res.json(tips);
  } catch (error) {
    next(error);
  }
};

exports.getTip = async (req, res, next) => {
  try {
    const tip = await Tip.findByPk(req.params.id, {
      include: ['author', 'ratings']
    });

    if (!tip) {
      return res.status(404).json({ message: 'Tip not found' });
    }

    res.json(tip);
  } catch (error) {
    next(error);
  }
};

exports.updateTip = async (req, res, next) => {
  try {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }

    const tip = await Tip.findByPk(req.params.id);
    if (!tip) {
      return res.status(404).json({ message: 'Tip not found' });
    }

    // Authorization check - only author or admin can update
    if (tip.authorId !== req.user.id && !req.user.isAdmin) {
      return res.status(403).json({ message: 'Not authorized' });
    }

    await tip.update(req.body);
    res.json(tip);
  } catch (error) {
    next(error);
  }
};

exports.deleteTip = async (req, res, next) => {
  try {
    const tip = await Tip.findByPk(req.params.id);
    if (!tip) {
      return res.status(404).json({ message: 'Tip not found' });
    }

    // Authorization check - only author or admin can delete
    if (tip.authorId !== req.user.id && !req.user.isAdmin) {
      return res.status(403).json({ message: 'Not authorized' });
    }

    await tip.destroy();
    res.status(204).end();
  } catch (error) {
    next(error);
  }
};

exports.markAsBestModel = async (req, res, next) => {
  try {
    // Only admin can mark as best model
    if (!req.user.isAdmin) {
      return res.status(403).json({ message: 'Not authorized' });
    }

    const tip = await Tip.findByPk(req.params.id);
    if (!tip) {
      return res.status(404).json({ message: 'Tip not found' });
    }

    // If marking as best model, first unmark any existing best model in same category
    if (req.body.isBestModel) {
      await Tip.update(
        { isBestModel: false },
        { where: { category: tip.category, isBestModel: true } }
      );
    }

    await tip.update({ isBestModel: req.body.isBestModel });
    res.json(tip);
  } catch (error) {
    next(error);
  }
};