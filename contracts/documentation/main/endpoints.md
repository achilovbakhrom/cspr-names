
# ** Main Contract **
1) Endpoint name: *****register_domain_name***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Creating domain name
    - **arg_main_duration** - Duration for the name (in years)
    - **arg_main_resolver_address** - Resolver address for the name
    - **arg_main_amount** - Payment amount for the name <br><br>
  
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:** <br>
```plantuml
@startuml

start

:**Arguments:**

**Domain name: ** test.cspr
**Duration: ** (duration in years)
**Resolver: ** (Resolver address - public key)
**Amount: ** (Payment amount in motes);

if (Is name correct?) then (yes)
else (no)
  :Return error message;
  stop
endif

:Call **get_contract_hash_for_domain_name** of the **Registry** contract
and take call contract result to **result** variable;


if (Is name exist in the registry?
**get_contract_hash_for_domain_name** of **Registry** Contract) then (yes)
  :Fetch data from database contract;
  if (Is name active?) then (yes)
    :Return error message;
    stop
  else (no)
    if (Is name in grace period?) then (yes)
      :Return error message;
      stop
    else (no)
    endif
  endif
else (no)
endif

:Get current_price form
**Price Oracle Contract**;

if (Amount == current_price) then (yes)  
else (no)
  :Return error message;
  stop
endif

:Perform payment
(Send assets from user's
account to smartcontract);

:Get Active NFT Contract;

:Mint name NFT on the Contract;

:Get appropriate database contract;

:Store name in the contract;

:Add name to the registry;

:Store name to User's Account Context;

stop

@enduml
```

<hr /> <br>

2) Endpoint name: *****extend***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Extending domain name
    - **arg_main_duration** - Duration (in milliseconds)
    - **arg_main_amount** - Payment amount for the name <br><br>  
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start
:**Domain name: ** test.cspr
**Duration: ** (duration in milliseconds)
**Amount: ** (Payment amount in motes);

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

:Get **current_price** from Price Oracle Contract;
if (Amount == current_price) then (yes)
else (no)
  :Return error message;
  stop
endif

:Perform payment;

:Update record date in Db;

stop
@enduml
```

<hr /> <br>

3) Endpoint name: *****domain_set_resolver_address***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Domain name
    - **arg_main_resolver_address** - the new Resolver address
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start
:**Domain name: ** test.cspr
**Resolver: ** the new Resolver address - public key;

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

if (Is name expired?) then (yes)
  :Return error message;
  stop
else (no)
endif

:Update record date in Db;

stop
@enduml
```

<hr /> <br>

4) Endpoint name: *****register_subdomain_name***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Domain name
    - **arg_main_subdomain_name** - Creating Subdomain name
    - **arg_main_resolver_address** - the new Resolver address
    - **arg_main_amount** - Payment amount <br> <br>
   
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start
:**Domain name: ** test.cspr
**Subdomain name:** sub.test.cspr
**Resolver: ** resolver address for subdomain name - public key
**Amount: ** Payment amount;

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

if (Is name expired?) then (yes)
  :Return error message;
  stop
else (no)
endif

:Get current_price from Price Oracle Contract;

if (Amount == current_price) then (yes)
else (no)
  :Return error message;
  stop
endif

:Perform payment process;

:Create subdomain name;

stop
@enduml
```

<hr /> <br>

5) Endpoint name: *****remove_subdomain_name***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Domain name
    - **arg_main_subdomain_name** - Removing Subdomain name
   
   Return value: <br>
    - **void;** <br><br>

   **Implementation:**<br>

```plantuml
@startuml
start
:**Domain name: ** test.cspr
**Subdomain name: ** removing subdomain name;

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

if (Is subdomain exist?) then (yes)
else (no)
  :Return error message;
  stop
endif

stop
@enduml
```

6) Endpoint name: *****remove_domain_name***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Removing Domain name
   
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start
:**Domain name: ** test.cspr
**Subdomain name: ** removing subdomain name;

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

:Remove domain name in database;
:Remove domain name from registry;

stop
@enduml
```

7) Endpoint name: *****list***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Listing Domain name
    - **arg_main_price** - Listing price
   
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start

:**Domain name: ** test.cspr
**Listing Price: ** Price;

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

:Get **token_id** for the domain name;

:List domain name;

stop
@enduml
```

8) Endpoint name: *****unlist***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - UnListing Domain name    
   
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start

:**Domain name: ** test.cspr
**Listing Price: ** Price;

if (Is name exist in registry?) then (yes)
else (no)
  :Return error message;
  stop
endif

if (Is caller owner?) then (yes)
else
  :Return error message;
  stop
endif

:Get **token_id** for the domain name;

:UnList domain name;


stop
@enduml
```

9) Endpoint name: *****buy***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Domain name
    - **arg_main_subdomain_name** - Creating Subdomain name
    - **arg_main_resolver_address** - the new Resolver address
    - **arg_main_amount** - Payment amount <br> <br>
   
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start

stop
@enduml
```

10) Endpoint name: *****transfer***** <br>
   
   Arguments: <br>
    - **arg_main_domain_name** - Domain name
    - **arg_main_subdomain_name** - Creating Subdomain name
    - **arg_main_resolver_address** - the new Resolver address
    - **arg_main_amount** - Payment amount <br> <br>
   
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:**<br>

```plantuml
@startuml
start

stop
@enduml
```
