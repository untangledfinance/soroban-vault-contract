const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { usdcContract, treasuryKeys, vault } = loadFixtures(); // Load the fixtures

const call = usdcContract.call(
  "approve",
  StellarSdk.nativeToScVal(treasuryKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(vault.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  new StellarSdk.ScInt(1000000000000000).toI128(), // 100000000 USDC
  StellarSdk.xdr.ScVal.scvU32(3110400) // max uint32
);

invokeContract(treasuryKeys, call); // Invoke the contract
