# Entities

** - required field <br>
? - optional field

# Common Entities


## DomainName

```plantuml
entity DomainName {
    + ** name: String
    + ** owner: AccountHash
    + ** resolver: AccountHash
    + ** token_id: String
    + ** end_time: u64
}
```

## SubdomainName

```plantuml
entity SubdomainName {
    + ** name: String
    + ** resolver: AccountHash
}
```

## NFTMetadata

```plantuml
entity NftMetadata {
    + ** name: String
    + ** token_name: String
}
```

## PriceItem

```plantuml
entity PriceItem {
    + ** char_count: u8,
    + ** price: U512
}
```

## Price

```plantuml
entity Price {
    + ** price_type: PriceType
    + ** price_min: U512
    + ** price_by_count: Vec<PriceItem>
    + ** price_more: U512
}
```

## RegistryContractHashPair
```plantuml
entity RegistryContractHashPair {
    + ** db_contract_hash: ContractHash
    + ** nft_contract_hash: ContractHash
}
```

## ContractHashItem

```plantuml
entity ContractHashItem {
    + ** c_type: ContractType
    + ** hash: ContractHash
}
```

## RegistryContractHashList
```plantuml
entity RegistryContractHashList {
    + ** contract_type: ContractType
    + ** contract_hash_list: Array<ContractHash>
    + ? attr_key: String
}
```

## RegistryPointer

```plantuml
entity RegistryPointer {
    + ** index: u16
    + ? count: u64
}
```

# Enums

## ContractType

```plantuml
enum ContractType {
    Main
    --
    Registry
    --
    Database
    --
    Nft
    --
    NftCore
    --
    PriceOracle
}
```

## PriceType

```plantuml
enum PriceType {
    Fixed
    --
    Dynamic
}

```

