const express = require("express");
const walletController = require("../controllers/wallet.controller");
const { authenticate, authorize } = require("../middlewares/auth");
const { roles } = require("../config/roles");

const router = express.Router();

// Example: Only authenticated users can create a wallet
router.post(
  "/",
  authenticate,
  authorize(roles.ADMIN, roles.SUPER_ADMIN),
  walletController.createWallet
);

// All users can get wallets (adjust based on your needs)
router.get(
  "/",
  authenticate,
  authorize(roles.USER, roles.ADMIN, roles.SUPER_ADMIN),
  walletController.getAllWallets
);

// Get wallet by ID
router.get(
  "/id/:id",
  authenticate,
  authorize(roles.USER, roles.ADMIN),
  walletController.getWalletById
);

// Get wallet by address
router.get(
  "/address/:address",
  authenticate,
  authorize(roles.USER),
  walletController.getWalletByAddress
);

// Update wallet
router.put(
  "/:id",
  authenticate,
  authorize(roles.ADMIN),
  walletController.updateWallet
);

// Delete wallet
router.delete(
  "/:id",
  authenticate,
  authorize(roles.SUPER_ADMIN),
  walletController.deleteWallet
);

module.exports = router;
