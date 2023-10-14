# Storage of registry contract representation

## 1) **DomainContractHashDictionary** - Dictionary

Key: **KEY_REGISTRY_DOMAIN_CONTRACT_MAP** <br>

```plantuml
@startuml
<p>Value</p>
@startjson
{
    "<Domain name>(For example: test.cspr)": "RegistryContractHashPair"
}
@endjson
@enduml
```

<br><hr><br>

## 2) **ContractHashListStore** - KeyValueStore

Key: **contract_type+attribute**<br>
Value: **ContractHash[]**

<br><hr><br>

## 3) **PointerStore** - KeyValueStore

Key: **contract_type+attribute**<br>
Value: **[class]RegistryPointer**

<br><hr><br>

## 4) **DatabaseContractMaxValueStore** - KeyValueStore

Key: **key_registry_database_contract_max_count_value**<br>
Value: **u16** type value

<br><hr><br>

## 5) **NftCoreContractMaxValueStore** - KeyValueStore

Key: **key_registry_nft_core_contract_max_count_value**<br>
Value: **u16** type value

## 6) **Operators** - KeyValueStore

Key: **key_registry_operators**
Value: ** ContractHash[] **