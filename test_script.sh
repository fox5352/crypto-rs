#!/usr/bin/env bash
# ^ This tells the system to run this script with the user's environment's bash

set -euo pipefail
# -e  : exit immediately if a command exits with a non-zero status
# -u  : treat unset variables as an error
# -o pipefail : make pipelines fail if *any* command in them fails

BIN=./target/release/crypto-rs   # Path to your compiled binary (adjust name if needed)
KEY="01234567890123456789012345678901"  # 32-byte AES key (required by your program)

echo "=== Test 1: Help flag ==="
$BIN --help   # Show help message using --help
$BIN -h       # Show help message using -h
echo          # Blank line for readability

echo "=== Test 2: Encrypt ==="
PLAINTEXT="Hello, world!"                # Example message to encrypt
ENCRYPTED=$($BIN "$KEY" "$PLAINTEXT")    # Run program to encrypt and capture output
echo "Plaintext : $PLAINTEXT"            # Print original text
echo "Encrypted : $ENCRYPTED"            # Print encrypted base64 output
echo

echo "=== Test 3: Decrypt ==="
DECRYPTED=$($BIN "$KEY" "$ENCRYPTED" --decrypt)  # Run program to decrypt
echo "Decrypted : $DECRYPTED"
# Check if decryption matches original
if [[ "$DECRYPTED" != "$PLAINTEXT" ]]; then
  echo "❌ Decryption did not match!"
  exit 1   # Exit with error
else
  echo "✅ Decryption successful"
fi
echo

echo "=== Test 4: Wrong key length ==="
# Try with an invalid key (too short)
if $BIN shortkey "Hello" 2>/dev/null; then
  # If program succeeds, that's wrong
  echo "❌ Expected error for short key"
  exit 1
else
  # If program fails, that's correct
  echo "✅ Got expected error for short key"
fi
echo

echo "=== Test 5: Invalid base64 input ==="
# Try decrypting something that isn’t base64
if $BIN "$KEY" "not_base64_data" --decrypt 2>/dev/null; then
  echo "❌ Expected base64 decode error"
  exit 1
else
  echo "✅ Got expected base64 decode error"
fi
echo

echo "=== All tests passed ✅ ==="
