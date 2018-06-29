#!/usr/bin/env bash

set -e -o pipefail

# Test private key & address
PKEY="1234567890123456789012345678901234567890123456789012345678901234"
PADDR="2e988a386a799f506693793c6a5af6b54dfaabfb"

# Chain Manager Native Contract
CMNC_ADDR="00000000000000000000000000000000000000ce"
CMNC_ABI=$(solc --abi scripts/contracts/system/chain_manager.sol 2>/dev/null | tail -1)

# Templates for some shell commands
ETHCALL='{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"%s", "data":"%s"}, "latest"],"id":2}'

# Test contract file
CONTRACT_DEMO="scripts/contracts/tests/contracts/cross_chain_token.sol"
DEMO_ABI=$(solc --abi ${CONTRACT_DEMO} 2>/dev/null | tail -1)

# Global variables which are set in functions
MAIN_CONTRACT_ADDR=
SIDE_CONTRACT_ADDR=

function title () {
    echo
    echo
    echo "################################################################################"
    echo "################################################################################"
    echo "################################################################################"
    echo
    echo "[$(date --iso-8601="seconds")] $@"
    echo
    echo "################################################################################"
    echo "################################################################################"
    echo "################################################################################"
    echo
    echo
}

function python_run () {
    local pycmd="$1;"
    shift 1
    while [ -n "$1" ]; do
        pycmd="${pycmd} $1;"
        shift 1
    done
    python -c "${pycmd}"
}

function func_encode () {
    local func="$1"
    python_run \
        "import sha3" \
        "keccak = sha3.keccak_256()" \
        "keccak.update('${func}')" \
        "print(keccak.hexdigest()[0:8])"
}

function abi_encode () {
    local abi="$1"
    local func="$2"
    local data="$3"
    python_run \
        "from ethereum.abi import ContractTranslator" \
        "ct = ContractTranslator('''${abi}''')" \
        "tx = ct.encode('${func}', [${data}])" \
        "print(tx.encode('hex'))"
}

function txtool_run () {
    local chain=$1
    shift 1
    cd "${chain}tool/txtool"
    python "$@" 2>/dev/null
    cd ../..
}

function start_chain () {
    local chain=$1
    local size=$2
    title "Start chain [${chain}] ..."
    for ((id=0;id<${size};id++)); do
        bin/cita setup ${chain}chain/${id}
    done
    for ((id=0;id<${size};id++)); do
        bin/cita start ${chain}chain/${id} >/dev/null 2>&1 &
    done
}

function wait_chain_for_height () {
    local chain=$1
    local height=$2
    title "Waiting for chain [${chain}] ..."
    while true; do
        local height_now=$(txtool_run ${chain} block_number.py | tail -1)
        if [ "${height}" != "None" ] \
                && [ "${height_now}" != "None" ] \
                && [ "${height_now}" -gt "${height}" ]; then
            break
        fi
    done
}

function deploy_contract () {
    local chain="$1"
    local solfile="$2"
    local extra="$3"
    local code="$(solc --bin "${solfile}" 2>/dev/null | tail -1)${extra}"
    txtool_run ${chain} make_tx.py --privkey "${PKEY}" --code "${code}"
    txtool_run ${chain} send_tx.py
    txtool_run ${chain} get_receipt.py --forever true
}

function send_contract () {
    local chain="$1"
    local addr="$2"
    local abi="$3"
    local func="$4"
    local input="$5"
    local code="$(abi_encode "${abi}" "${func}" "${input}")"
    txtool_run ${chain} make_tx.py --privkey "${PKEY}" \
        --to "${addr}" --code "0x${code}"
    txtool_run ${chain} send_tx.py
    txtool_run ${chain} get_receipt.py --forever true
}

function call_contract () {
    local chain="$1"
    local addr="$2"
    local code="$3"
    case ${chain} in
        main)
            port=11337;;
        side)
            port=21337;;
        ?)
            exit 1
            ;;
    esac
    curl -s -X POST -d "$(printf "${ETHCALL}" "${addr}" "0x${code}")" \
        127.0.0.1:${port} \
        | jq .result | xargs -I {} echo {}
}

function get_addr () {
    local chain="$1"
    eval echo \
        $(txtool_run ${chain} get_receipt.py --forever true \
        | jq .contractAddress) | cut -c 3-
}

function get_tx () {
    local chain="$1"
    eval echo \
        $(txtool_run ${chain} get_receipt.py --forever true \
        | jq .transactionHash) | cut -c 3-
}

function hex2dec () {
    local hex="$1"
    if [ "$(echo ${hex} | cut -c 1-2)" != "0x" ] || [ "${hex}" = "0x" ]; then
        echo "none"
    else
        python_run "print(int('${hex}', 16))"
    fi
}

function parse_addresses () {
    local addrs="$(echo $1 | cut -c 131-)"
    local start=1
    local stop=64
    while [ -n "${addrs}" ]; do
        echo ${addrs} | cut -c 25-64 | xargs -I {} echo "0x{}"
        addrs="$(echo ${addrs} | cut -c 65-)"
    done | sort
}

function call_demo_for_main () {
    local code="$1"
    call_contract main "${MAIN_CONTRACT_ADDR}" "${code}"
}

function call_demo_for_side () {
    local code="$1"
    call_contract side "${SIDE_CONTRACT_ADDR}" "${code}"
}

function assert_equal () {
    local left="$1"
    local right="$2"
    local errmsg="$3"
    if [ "${left}" = "${right}" ]; then
        : # Test is passed!
    else
        echo "[ERROR] <${left}> != <${right}>: ${errmsg}."
        exit 1
    fi
}

function test_demo_contract () {
    local data=
    local code="$(func_encode 'getChainId()')"
    local main_chain_id=$(hex2dec $(call_contract main "${CMNC_ADDR}" "${code}"))
    local side_chain_id=$(hex2dec $(call_contract side "${CMNC_ADDR}" "${code}"))
    local main_tokens=50000
    local side_tokens=30000
    local crosschain_tokens=1234

    title "Check all authorities."
    local data=$(printf "%064x" "${side_chain_id}")
    local code="$(func_encode 'getAuthorities(uint256)')${data}"
    assert_equal \
        "$(parse_addresses \
            $(call_contract main "${CMNC_ADDR}" "${code}") \
                | xargs -I {} printf {})" \
        "$(cat sidechain/template/authorities.list \
            | sort | xargs -I {} printf {})" \
        "The authorities is not right for side chain."
    local data=$(printf "%064x" "${main_chain_id}")
    local code="$(func_encode 'getAuthorities(uint256)')${data}"
    assert_equal \
        "$(parse_addresses \
            $(call_contract side "${CMNC_ADDR}" "${code}") \
                | xargs -I {} printf {})" \
        "$(cat mainchain/template/authorities.list \
            | sort | xargs -I {} printf {})" \
        "The authorities is not right for main chain."

    title "Deploy contract for both main chain and side chain."
    deploy_contract main "../../${CONTRACT_DEMO}" \
        "$(printf "%064x" ${main_tokens})$(printf "%064x" ${main_chain_id})"
    deploy_contract side "../../${CONTRACT_DEMO}" \
        "$(printf "%064x" ${side_tokens})$(printf "%064x" ${side_chain_id})"
    MAIN_CONTRACT_ADDR=$(get_addr main)
    SIDE_CONTRACT_ADDR=$(get_addr side)
    echo "Demo contract for main at [${MAIN_CONTRACT_ADDR}]."
    echo "Demo contract for side at [${SIDE_CONTRACT_ADDR}]."

    title "Check from_chain_id for both chains."
    code="$(func_encode "get_from_chain_id()")"
    assert_equal "${main_chain_id}" \
        "$(hex2dec $(call_demo_for_main "${code}"))" \
        "The from_chain_id is not right for main chain."
    assert_equal "${side_chain_id}" \
        "$(hex2dec $(call_demo_for_side "${code}"))" \
        "The from_chain_id is not right for side chain."

    title "Check tokens for both chains."
    data=$(printf "%64s" "${PADDR}" | tr ' ' '0')
    code="$(func_encode 'get_balance(address)')${data}"
    assert_equal ${main_tokens} \
        "$(hex2dec $(call_demo_for_main "${code}"))" \
        "The tokens is not right for main chain."
    assert_equal ${side_tokens} \
        "$(hex2dec $(call_demo_for_side "${code}"))" \
        "The tokens is not right for side chain."

    title "Send tokens from main chain."
    send_contract main "${MAIN_CONTRACT_ADDR}" "${DEMO_ABI}" \
        "send_to_side_chain" \
        "${side_chain_id}, '${SIDE_CONTRACT_ADDR}', ${crosschain_tokens}"
    local maintx=$(get_tx main)

    title "Waiting for proof."
    local height_now=$(txtool_run main block_number.py | tail -1)
    wait_chain_for_height main $((height_now+3))

    title "Send tokens to side chain."
    local sidetx=$(./bin/cita-relayer-parser \
        -c ${main_chain_id} -t ${maintx} \
        -f ../../tools/relayer-parser/res/relayer-parser-demo.json)
    txtool_run side get_receipt.py --tx=${sidetx} --forever true

    title "Check balance for both chains after crosschain transaction."
    data=$(printf "%64s" "${PADDR}" | tr ' ' '0')
    code="$(func_encode 'get_balance(address)')${data}"
    assert_equal $((main_tokens-crosschain_tokens)) \
        "$(hex2dec $(call_demo_for_main "${code}"))" \
        "The balance is not right for main chain."
    assert_equal $((side_tokens+crosschain_tokens)) \
        "$(hex2dec $(call_demo_for_side "${code}"))" \
        "The balance is not right for side chain."
}

function main () {

    title "Test is starting ..."

    local code=

    cd target/install

    title "Enable side chain ...."
    send_contract main "${CMNC_ADDR}" "${CMNC_ABI}" \
        "enableSideChain" "2"

    cd ../..

}

main "$@"
