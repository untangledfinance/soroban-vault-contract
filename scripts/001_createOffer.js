const StellarSdk = require("@stellar/stellar-sdk");
const { invokeContract, loadFixtures } = require("./utils");

const { distributorKeys, usdyc, usdc, vault } = loadFixtures();

const call = vault.call(
  "create",
  StellarSdk.nativeToScVal(distributorKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(usdyc.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.nativeToScVal(usdc.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.xdr.ScVal.scvU32(100),
  StellarSdk.xdr.ScVal.scvU32(100)
);

invokeContract(distributorKeys, call);
