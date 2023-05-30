# **Registry Contract**

1. Endpoint name: *****map_domain_name_to_contract_hash***** <br>
   
   Arguments: <br>
    - **arg_registry_domain_name** - The domain name required for mapping
    - **arg_registry_database_contract_hash** - Database Contract Hash
    - **arg_nft_contract_hash** - NFT Contract Hash
  
   Return value: <br />
    - **void;** <br /><br />
  
   **Implementation:** <br />
```plantuml
@startuml

start

:**Arguments:**

**Domain name: ** test.cspr
**Database Contract Hash: ** Contract hash
**NFT Contract Hash: ** Contract Hash;

if (is **caller** in operators list or maintainer?) then (yes)
else (no)
    :Return error message;
    stop
endif

:Create an object with type **RegistryContractHashPair**;

:Store the object into **DomainContractHashDictionary** as keypair
key = **domain name**, value = **object**;

end

@enduml
```

<hr> <br>

2. Endpoint name: *****get_contract_hash_for_domain_name***** <br />

    Arguments: <br>
    - **arg_registry_domain_name** - domain name key for requesting data
    
    Return value: <br>
    - **RegistryContractHashPair** <br><br>
  
   **Implementation:** <br />

```plantuml
@startuml

start

:**Arguments:**

**Domain name: ** test.cspr;

if (is caller in operators) then (yes)
else (no)
    :Return error message;
    stop
endif

if (Is domain name exists in the **DomainContractHashDictionary**?) then (yes)
else (no)
    :Show Error message;
    stop
endif

:Retrieve data with
type **RegistryContractHashPair**
from **DomainContractHashDictionary** store;

end

@enduml
```

<hr> <br>

1. Endpoint name: *****set_contract_hash_list***** <br />
    
    Arguments: <br>
    - ** arg_registry_contract_hash_list ** - represents list of contracts **RegistryContractHashList** <br><br>
    
    Return value:
    - **void;** <br><br>
  
    **Implementation:**

```plantuml
@startuml
start

:**Arguments:** 

**Contract Hash List:** RegistryContractHashList type;

if (is caller maintainer) then (yes)
else (no)
    :Return error message;
    stop
endif


:Iterate RegistryContractHashList;

:convert contract_hash list and save it to the ContractHashListStore;

if (item.contract_type == DatabaseContractType || item.contract_type == NftCoreContractType) then (yes)
    :Set(Migrate) values like **[index, count]** for PointerStore;
else (no)
    :Set(Migrate) values like **[index]** for PointerStore;
endif

end
@enduml
```
<hr> <br>

1. Endpoint name: *****get_contract***** <br>
   
   Arguments: <br>
   - **arg_registry_contract_type** - ContractType
   - **arg_registry_attribute** - attribute is used for database contract (Optional) <br><br>

    Return value:
    - Actual contract hash <br><br>
    
    **Implementation:**

```plantuml
start

:**Arguments:** 

**Contract Type:** [enum]ContractType type
**Attribute: ** attribute (extension);

if (is caller in operators) then (yes)
else (no)
    :Return error message;
    stop
endif

if (contract_type == db && attribute == None) then(yes)
    :Return error message;
    stop;
else (no)
endif

:Convert **ContractType** to **RegistryContractType**;

:Call **getter_method** (The Diagram below);

:Get ContractHash by the **getter_method** and return it;

end
```

## Getter Method <br>


```plantuml
@startuml
start
:**Arguments:**

**contract_type:** [enum]ContractType
**attribute:** Attribute is extension of name (optional);

:key = attribute != None ? contract_type+:+attribute : contract_type;

:get **pointer[index, count(optional)]** for **the key** from **PointerStore**;

if (pointer == None) then (yes)
    :Return error message;
    stop
else (no)
endif

:get **contract_hash_list** from **ContractHashListStore**  by **key**;

if (index >= contract_hash_list.len()) then (yes)
    :Return error message;
    stop
else (no)
endif

:Get **Contract Hash** by index in the **pointer**;

:Return the **Contract Hash**;

end
@enduml
```

<hr> <br>

5. Endpoint name: *****increment_count_of_contract***** <br>
   
   Arguments: <br>
   - **arg_registry_contract_type** - ContractType
   - **arg_registry_attribute** - attribute is used for database contract (Optional) <br><br>

    Return value:
    - void <br><br>
    
    **Implementation:**

```plantuml
@startuml

start

:**Arguments:**

**contract_type:** [enum]ContractType
**attribute:** Attribute is extension of name (optional);

if (is caller in operators) then (yes)
else (no)
    :Return error message;
    stop
endif

if (contract_type != DatabaseContractType && contract_type != NftCoreContractType) then (yes)
    :Return error message;
    stop
else (no)
endif

:key = attribute != None ? contract_type+:+attribute : contract_type;

:get **pointer[index, count(optional)]** for **the key** from **PointerStore**;

:get **count** from the **pointer**;

if (count == None) then (yes)
    :Return error message;
    stop
else (no)
endif

:let mut max_value;

if (contract_type == DatabaseContractType) then (yes)
    :max_value = value from **DatabaseContractMaxValueStore**;
else (no)
    :max_value = value from **NftCoreContractMaxValueStore**;
endif

if (max_value == None) then (yes)
    :Return error message;
    stop
else (no)
endif

if (count >= max_value) then (yes)
    :increment index of the **pointer**;
    :count=0;
    :store pointer into **PointerStore**;
else (no)
endif

end

@enduml
```

<hr> <br><br>


1. Endpoint name: *****add_operator***** <br>
   
   Arguments: <br>
   - **arg_registry_contract_hash** - ContractHash
   <br><br>

    Return value:
    - void <br><br>
    
    **Implementation:**

```plantuml
@startuml

start

:**Arguments:**

**arg_registry_contract_hash:** ContractHash;

if (is caller maintainer?) then(yes)
else (no)
    :Return error message;
    stop
endif

if (is contract_hash in the **Operators** list) then (yes)
else (no)
    :add contract_hash to the **Operators** Store;
endif

end

@enduml
```

<hr> <br><br>

7. Endpoint name: *****remove_operator***** <br>
   
   Arguments: <br>
   - **arg_registry_contract_hash** - ContractHash
   <br><br>

    Return value:
    - void <br><br>
    
    **Implementation:**

```plantuml
@startuml

start

:**Arguments:**

**arg_registry_contract_hash:** ContractHash;

if (is caller maintainer?) then(yes)
else (no)
    :Return error message;
    stop
endif

if (is contract_hash in the **Operators** list) then (yes)
else (no)
    :remove contract_hash from the **Operators** Store;
endif

end

@enduml
```

<hr> <br><br>