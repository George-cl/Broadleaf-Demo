STATE_ROUTE_HASH=$(casper-client get-block -n http://127.0.0.1:40101 | jq -r '.result.block.header.state_root_hash')

echo "= Fraser's sent messages ="
casper-client query-state \
-n http://127.0.0.1:40101 \
--state-root-hash $STATE_ROUTE_HASH \
--key 01a8203800ced6be79bedac72f6ce13049eb5bbbb7205a7e9b63e1a3d76ea162cd \
--query-path "Fraser" \
| jq -r '.result.stored_value.CLValue.parsed' | jq

echo "= George's sent messages ="
casper-client query-state \
-n http://127.0.0.1:40101 \
--state-root-hash $STATE_ROUTE_HASH \
--key 01a8203800ced6be79bedac72f6ce13049eb5bbbb7205a7e9b63e1a3d76ea162cd \
--query-path "George" \
| jq -r '.result.stored_value.CLValue.parsed' | jq
