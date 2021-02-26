casper-client query-state \
-n http://127.0.0.1:40101 \
--state-root-hash 6ff6da1a1e570ecb5a0603b22b5f26ae13e82f7420efc0ce79763b1c4ebf5454 \
--key 01a8203800ced6be79bedac72f6ce13049eb5bbbb7205a7e9b63e1a3d76ea162cd \
--query-path "George" \
| jq -r '.result.stored_value.CLValue.parsed' | jq