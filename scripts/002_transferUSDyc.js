const StellarSdk = require("@stellar/stellar-sdk");
const { invokeContract, loadFixtures } = require("./utils");

const { distributorKeys, vault, usdycContract } = loadFixtures();

const call = usdycContract.call(
  "transfer",
  StellarSdk.nativeToScVal(distributorKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(vault.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  StellarSdk.xdr.ScVal.scvI128(10000000000) // 1000 USDyc
);

invokeContract(distributorKeys, call);
