const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { usdcContract, distributorKeys, vault } = loadFixtures(); // Load the fixtures

const call = usdcContract.call(
  "approve",
  StellarSdk.nativeToScVal(distributorKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(vault.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.xdr.ScVal.scvI128(1000000000000000), // 100000000 USDC
  StellarSdk.xdr.ScVal.scvU32(4294967295) // max uint32
);

invokeContract(distributorKeys, call); // Invoke the contract
