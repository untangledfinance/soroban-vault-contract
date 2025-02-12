const StellarSdk = require("@stellar/stellar-sdk");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, distributorKeys, usdyc } = loadFixtures();

const call = vault.call(
  "claim",
  StellarSdk.nativeToScVal(usdyc.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.xdr.ScVal.scvI128(10000000000) // 1000 USDyc
);

invokeContract(distributorKeys, call);
