const StellarSdk = require("@stellar/stellar-sdk");
const rpc = new StellarSdk.rpc.Server("https://soroban-testnet.stellar.org");

const { invokeContract, loadFixtures } = require("./utils");

const { vault, distributorKeys, usdyc } = loadFixtures();

const call = vault.call("get_offer");

async function main() {
  const res = await invokeContract(distributorKeys, call);
  console.log(res);
}

main();
