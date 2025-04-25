const express = require('express');
const walletController = require('../controllers/wallet.controller');

const router = express.Router();

// Create a new wallet
router.post('/', walletController.createWallet);

// Get all wallets
router.get('/', walletController.getAllWallets);

// Get wallet by ID
router.get('/id/:id', walletController.getWalletById);

// Get wallet by address
router.get('/address/:address', walletController.getWalletByAddress);

// Update wallet
router.put('/:id', walletController.updateWallet);

// Delete wallet
router.delete('/:id', walletController.deleteWallet);

module.exports = router;
