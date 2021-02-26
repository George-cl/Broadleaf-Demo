casper-client put-deploy \
--node-address http:/127.0.0.1:40101 \
--secret-key ~/CasperLabs/casper-node/utils/nctl/assets/net-1/faucet/secret_key.pem \
--chain-name casper-net-1 \
--session-path ~/CasperLabs/Broadleaf-Demo/contract/broadleaf-messenger/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--session-arg "sender:string='Sender:George'" \
--session-arg "message:string='Message:Hey hows it going for you?" \
--payment-amount 1000000000 | jq