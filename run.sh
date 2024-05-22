#! /bin/bash

# To find the correct one
# for ((i=0;; i++)); do JQP=$(jq -r ".packages[${i}].targets[0].name" <<<"${CMTDT}"); if [ "${JQP}" == "lyrics" ]; then echo "${i}"; break; else echo "${JQP}" ; fi; done

BIN_NAME=$(cargo metadata --format-version 1 | jq -r '.packages[63].targets[0].name')
# cargo run --bin ${BIN_NAME} -- "${@}"

# Get this script location
MELOC=$(
  dirname "$(readlink -f "${BASH_SOURCE[0]}")"
)

# Check for both (debug or release) and select the most recent one
if [ -f "${MELOC}/target/debug/${BIN_NAME}" ]; then
  LOC=debug
elif [ -f "${MELOC}/target/release${BIN_NAME}" ]; then
  LOC=release
else
  cargo build --release
  LOC=release
fi

# Get the correct path to binary file
BIN="${MELOC}/target/${LOC}/${BIN_NAME}"

# Run the binary
${BIN} "${@}"
