const StellarSdk = require("@stellar/stellar-sdk");

const rpc = new StellarSdk.rpc.Server("https://soroban-testnet.stellar.org");
const horizon = new StellarSdk.Horizon.Server(
  "https://horizon-testnet.stellar.org"
);
const dotenv = require("dotenv");
dotenv.config();

function invokeContract(account, fnCall) {
  rpc
    .getAccount(account.publicKey())
    .then(async (acc) => {
      let tx = new StellarSdk.TransactionBuilder(acc, {
        fee: 100,
        networkPassphrase: StellarSdk.Networks.TESTNET,
      })
        .addOperation(fnCall)
        .setTimeout(30)
        .build();

      let preparedTx = await rpc.prepareTransaction(tx);
      preparedTx.sign(account);
      return rpc.sendTransaction(preparedTx);
    })
    .then(console.log)
    .catch(console.error);
}

function invokeOperation(account, op) {
  horizon
    .loadAccount(account.publicKey())
    .then((acc) => {
      let tx = new StellarSdk.TransactionBuilder(acc, {
        fee: 100,
        networkPassphrase: StellarSdk.Networks.TESTNET,
      })
        .addOperation(op)
        .setTimeout(30)
        .build();

      tx.sign(account);
      return horizon.submitTransaction(tx);
    })
    .then(console.log)
    .catch(console.error);
}

function createAssetContract(account, asset) {
  rpc.getAccount(caller.publicKey()).then(async (acc) => {
    let tx = new StellarSdk.TransactionBuilder(acc, {
      fee: 100,
      networkPassphrase: StellarSdk.Networks.TESTNET,
    })
      .addOperation(
        StellarSdk.Operation.createStellarAssetContract({
          asset: asset,
        })
      )
      .setTimeout(30)
      .build();

    let preparedTx = await rpc.prepareTransaction(tx);
    preparedTx.sign(caller);
    return rpc.sendTransaction(preparedTx);
  });
}

function loadFixtures() {
  const issuerKeys = StellarSdk.Keypair.fromSecret(process.env.ISSUER_KEYS);
  const distributorKeys = StellarSdk.Keypair.fromSecret(
    process.env.DISTRIBUTOR_KEYS
  );
  const aliceKeys = StellarSdk.Keypair.fromSecret(process.env.ALICE_KEYS);
  const usdyc = new StellarSdk.Asset("testUSDYC", issuerKeys.publicKey());
  const usdc = new StellarSdk.Asset("USDC", issuerKeys.publicKey());

  const vault = new StellarSdk.Contract(process.env.VAULT_ADDRESS);
  const usdycContract = new StellarSdk.Contract(process.env.USDYC_CONTRACT);
  const usdcContract = new StellarSdk.Contract(process.env.USDC_CONTRACT);

  return {
    issuerKeys,
    distributorKeys,
    aliceKeys,
    usdyc,
    usdc,
    vault,
    usdycContract,
    usdcContract,
  };
}

module.exports = {
  invokeContract,
  invokeOperation,
  createAssetContract,
  loadFixtures,
};
