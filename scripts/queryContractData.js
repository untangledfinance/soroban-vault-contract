const StellarSdk = require("@stellar/stellar-sdk");
const rpc = new StellarSdk.rpc.Server("https://soroban-testnet.stellar.org");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, distributorKeys, usdyc } = loadFixtures();

const call = vault.call("get_offer");

async function main() {
  const vaultBalance = await rpc.getSACBalance(
    "CD4YJPUEMYOQX7NZZSVG3KDSP6VJSWQ3MIPTEHDVGGKTPDC4VHBLUDDN",
    new StellarSdk.Asset(
      "testUSDyc",
      "GDS6B2CZTPNJRDEARO4Q64EJAJ5E2KLBVPC4PIDACCEHISK5QTAKCMM3"
    ),
    StellarSdk.Networks.TESTNET
  );

  console.log(
    "Vault balance: ",
    vaultBalance.balanceEntry ? vaultBalance.balanceEntry.amount : 0
  );
}

main();
