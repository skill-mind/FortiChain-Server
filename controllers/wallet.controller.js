const Wallet = require('../models/wallet.model');
const { validateWallet } = require('../utils/validators');
const logger = require('../utils/logger');

// Create a new wallet
exports.createWallet = async (req, res) => {
  try {
    // Validate request body
    const { error } = validateWallet(req.body);
    if (error) {
      logger.warn(`Validation error: ${error.details[0].message}`, { address: req.body.address });
      return res.status(400).json({
        success: false,
        message: error.details[0].message
      });
    }

    // Check if wallet address already exists
    const existingWallet = await Wallet.findOne({
      where: { address: req.body.address }
    });

    if (existingWallet) {
      logger.warn(`Attempted to create duplicate wallet with address: ${req.body.address}`);
      return res.status(409).json({
        success: false,
        message: 'Wallet address already exists'
      });
    }

    // Create new wallet
    const wallet = await Wallet.create({
      address: req.body.address,
      role: req.body.role
    });

    logger.info(`Wallet created successfully with ID: ${wallet.id}`, { address: wallet.address, role: wallet.role });
    return res.status(201).json({
      success: true,
      message: 'Wallet created successfully',
      data: {
        id: wallet.id,
        address: wallet.address,
        role: wallet.role,
        createdAt: wallet.createdAt
      }
    });
  } catch (error) {
    logger.error('Error creating wallet:', error);
    return res.status(500).json({
      success: false,
      message: 'Internal server error',
      error: error.message
    });
  }
};

// Get all wallets
exports.getAllWallets = async (req, res) => {
  try {
    const wallets = await Wallet.findAll();
    logger.info(`Retrieved ${wallets.length} wallets`);
    return res.status(200).json({
      success: true,
      count: wallets.length,
      data: wallets
    });
  } catch (error) {
    logger.error('Error fetching wallets:', error);
    return res.status(500).json({
      success: false,
      message: 'Internal server error',
      error: error.message
    });
  }
};

// Get wallet by ID
exports.getWalletById = async (req, res) => {
  try {
    const wallet = await Wallet.findByPk(req.params.id);

    if (!wallet) {
      logger.warn(`Wallet not found with ID: ${req.params.id}`);
      return res.status(404).json({
        success: false,
        message: 'Wallet not found'
      });
    }

    logger.info(`Retrieved wallet by ID: ${wallet.id}`);
    return res.status(200).json({
      success: true,
      data: wallet
    });
  } catch (error) {
    logger.error(`Error fetching wallet by ID ${req.params.id}:`, error);
    return res.status(500).json({
      success: false,
      message: 'Internal server error',
      error: error.message
    });
  }
};

// Get wallet by address
exports.getWalletByAddress = async (req, res) => {
  try {
    const wallet = await Wallet.findOne({
      where: { address: req.params.address }
    });

    if (!wallet) {
      logger.warn(`Wallet not found with address: ${req.params.address}`);
      return res.status(404).json({
        success: false,
        message: 'Wallet not found'
      });
    }

    logger.info(`Retrieved wallet by address: ${wallet.address}`);
    return res.status(200).json({
      success: true,
      data: wallet
    });
  } catch (error) {
    logger.error(`Error fetching wallet by address ${req.params.address}:`, error);
    return res.status(500).json({
      success: false,
      message: 'Internal server error',
      error: error.message
    });
  }
};

// Update wallet
exports.updateWallet = async (req, res) => {
  try {
    // Validate request body
    if (req.body.role) {
      const { error } = validateWallet({
        address: "0x0000000000000000000000000000000000000000", // Dummy address for validation
        role: req.body.role
      });

      if (error) {
        logger.warn(`Validation error updating wallet ${req.params.id}: ${error.details[0].message}`);
        return res.status(400).json({
          success: false,
          message: error.details[0].message
        });
      }
    }

    // Find wallet
    const wallet = await Wallet.findByPk(req.params.id);

    if (!wallet) {
      logger.warn(`Attempted to update non-existent wallet with ID: ${req.params.id}`);
      return res.status(404).json({
        success: false,
        message: 'Wallet not found'
      });
    }

    // Update wallet
    await wallet.update({
      role: req.body.role || wallet.role
    });

    logger.info(`Wallet updated successfully with ID: ${wallet.id}`, { role: wallet.role });
    return res.status(200).json({
      success: true,
      message: 'Wallet updated successfully',
      data: wallet
    });
  } catch (error) {
    logger.error(`Error updating wallet ${req.params.id}:`, error);
    return res.status(500).json({
      success: false,
      message: 'Internal server error',
      error: error.message
    });
  }
};

// Delete wallet
exports.deleteWallet = async (req, res) => {
  try {
    // Find wallet
    const wallet = await Wallet.findByPk(req.params.id);

    if (!wallet) {
      logger.warn(`Attempted to delete non-existent wallet with ID: ${req.params.id}`);
      return res.status(404).json({
        success: false,
        message: 'Wallet not found'
      });
    }

    // Delete wallet
    await wallet.destroy();

    logger.info(`Wallet deleted successfully with ID: ${req.params.id}`);
    return res.status(200).json({
      success: true,
      message: 'Wallet deleted successfully'
    });
  } catch (error) {
    logger.error(`Error deleting wallet ${req.params.id}:`, error);
    return res.status(500).json({
      success: false,
      message: 'Internal server error',
      error: error.message
    });
  }
};
