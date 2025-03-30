const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, distributorKeys } = loadFixtures(); // Load the fixtures

const call = vault.call(
  "updt_price",
  StellarSdk.xdr.ScVal.scvU32(100),
  StellarSdk.xdr.ScVal.scvU32(200)
); // 1 USDyc = 2 USDC

invokeContract(distributorKeys, call); // Invoke the contract
