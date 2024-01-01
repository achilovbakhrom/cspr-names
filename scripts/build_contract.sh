#!/bin/bash

CONTRACT_NAMES=("configuration", "database", "main", "nft-contract", "nft-core", "price-oracle", "registry")

echo "${CONTRACT_NAMES[@]}"

isCorrectContractName () {
  
    local contractName="$1"
    
    if [[ $(echo ${CONTRACT_NAMES[@]} | fgrep -w $contractName) ]]
    then
        return 0
    else
        return 1
    fi
}

isCorrectContractName $1

if [[ $? -eq 1 ]]
then
    echo "provide correct contract name!!!"
    echo "correct names are: ${CONTRACT_NAMES[@]}"
    exit 1
fi

cd "contracts/$1" && make build-contract
