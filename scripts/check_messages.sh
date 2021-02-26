casper-client query-state \
-n http://127.0.0.1:40101 \
--state-root-hash 78a73b75d32bf0d43bbd9bc4acbf227c91fd7e444cba8e9a162a08af1f58be4a \
--key 013037f71429b16b5c28af7f7175b5298500e30a8b654521d64eed4d39af118691 \
--query-path "Sender:George" \
| jq -r '.result.stored_value.CLValue.parsed'