const StellarSdk = require("@stellar/stellar-sdk");
const { invokeContract, loadFixtures } = require("./utils");

const { distributorKeys, vault, usdycContract } = loadFixtures();

const call = usdycContract.call(
  "transfer",
  StellarSdk.nativeToScVal(distributorKeys.publicKey(), { type: "address" }),
  StellarSdk.nativeToScVal(vault.contractId(StellarSdk.Networks.TESTNET), {
    type: "address",
  }),
  new StellarSdk.ScInt(10000000000).toI128() // 1000 USDyc
);

invokeContract(distributorKeys, call);
