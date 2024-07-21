const { sorobanServer, keypair, SorobanSdk } = require('../config/soroban');

const invokeContract = async (contractId, functionName, args) => {
  const account = await sorobanServer.loadAccount(keypair.publicKey());

  const transaction = new SorobanSdk.TransactionBuilder(account, {
    fee: SorobanSdk.BASE_FEE,
    networkPassphrase: SorobanSdk.Networks.TESTNET,
  })
    .addOperation(SorobanSdk.Operation.invokeContract({
      contractId,
      functionName,
      args,
    }))
    .setTimeout(30)
    .build();

  transaction.sign(keypair);
  const result = await sorobanServer.submitTransaction(transaction);
  return result;
};

module.exports = { invokeContract };
