const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, aliceKeys } = loadFixtures(); // Load the fixtures

const call = vault.call(
  "redeem_request",
  StellarSdk.nativeToScVal(aliceKeys.publicKey(), { type: "address" }),
  new StellarSdk.ScInt(10000000).toI128() // 1 USDyc
);

invokeContract(aliceKeys, call); // Invoke the contract
