const fs = require('fs');

let {CasperClient, RuntimeArgs, DeployUtil } = require('casper-client-sdk');

let nodeUrl = 'http://localhost:40101';
let eventStoreUrl = 'http://localhost:3000';
let wasmPath = '/home/ethilios/CasperLabs/Broadleaf-Demo/contract/broadleaf-messenger/contract/target/wasm32-unknown-unknown/release/contract.wasm';
let networkName = 'casper-net-1';

let client = new CasperClient(nodeUrl, eventStoreUrl);

const buildMessengerDeploy = (accountPublicKey, args) => {
    let deployParams = new DeployUtil.DeployParams(
        accountPublicKey,
        networkName
    );
    var session = new Uint8Array(fs.readFile(wasmPath, null).buffer);
    let runtimeArgs = RuntimeArgs.fromMap(args);

    let sessionModule = DeployUtil.ExecutableDeployItem.newModuleBytes(
        session,
        runtimeArgs
    );
    let payment = DeployUtil.standardPayment(100000000000);
    return DeployUtil.makeDeploy(deployParams, sessionModule, payment);
};

const sendMessengerDeploy = async (deploy, signingKeys) => {
    for(let key of signingKeys){
        console.log(`Signed by: ${toAccountHashString(key.publicKey)}`);
        deploy = client.signDeploy(deploy, key);
    }
    await client.putDeploy(deploy);
};

export {
    buildMessengerDeploy,
    sendMessengerDeploy
}