const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, aliceKeys } = loadFixtures();

const call = vault.call(
  "deposit",
  StellarSdk.nativeToScVal(aliceKeys.publicKey(), { type: "address" }),
  StellarSdk.xdr.ScVal.scvI128(100000000), // 10 USDyc
  StellarSdk.xdr.ScVal.scvI128(100000000) // 10 
);

invokeContract(aliceKeys, call);
