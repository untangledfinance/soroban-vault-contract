const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, aliceKeys } = loadFixtures(); // Load the fixtures

const call = vault.call(
  "redeem",
  StellarSdk.nativeToScVal(aliceKeys.publicKey(), { type: "address" }),
  StellarSdk.xdr.ScVal.scvI128(10000000), // 1 USDyc
  StellarSdk.xdr.ScVal.scvI128(10000000) // 1 USDC
);

invokeContract(aliceKeys, call); // Invoke the contract
