const StellarSdk = require("@stellar/stellar-sdk");
const { invokeContract, loadFixtures } = require("./utils");

const { distributorKeys, treasuryKeys, usdyc, usdc, vault } = loadFixtures();

const call = vault.call(
  "initialize",
  StellarSdk.nativeToScVal(distributorKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(treasuryKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(usdyc.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.nativeToScVal(usdc.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.xdr.ScVal.scvU32(100000),
  StellarSdk.xdr.ScVal.scvU32(100000)
);

invokeContract(distributorKeys, call);
