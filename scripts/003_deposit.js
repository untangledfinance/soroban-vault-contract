const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, aliceKeys } = loadFixtures();

const call = vault.call(
  "deposit",
  StellarSdk.nativeToScVal(aliceKeys.publicKey(), { type: "address" }),
  new StellarSdk.ScInt(100000000).toI128(), // 10 USDyc
  new StellarSdk.ScInt(100000000).toI128()
);

invokeContract(aliceKeys, call);
