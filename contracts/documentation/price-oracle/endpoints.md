# **PriceOracle Contract**

1. Endpoint name: *****set_price***** <br>
   
   Arguments: <br>
    - **arg_price_oracle_extension** - Domain name Extension (**String**);
    - **arg_price_oracle_price_type** - PriceType enum
    - **arg_price_oracle_price** - price is used for both types: for fixed type it is the main price, for dynamic type it is min_price;
    - **arg_price_oracle_price_mid** - it is used for dynamic type, vec of U512 type;
    - **arg_price_oracle_chars_count_mid** - used for dynamic type, length should match with the length of price_mid;
    - **arg_price_oracle_price_more** - used for dynamic type and represents tha max_price;
  
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:** <br>
```plantuml
@startuml

start

:**Arguments:**

**Domain name extension** - String
**Price Type** - PriceType enum
**Price** - U512
**PriceMid** - array of U512
**CharsCount** - array of u64
**PriceMore** - the max_price;


if (is **caller** in operators list or maintainer?) then (yes)
else (no)
    :Return error message;
    stop
endif

:Create the **Price** class;

:Save **Price** in **PriceOracleDb**;

end

@enduml
```

<hr> <br>

2. Endpoint name: *****set_price_simple_operations***** <br>
   
   Arguments: <br>
    - **arg_price_oracle_price** - price is used for both types: for fixed type it is the main price, for dynamic type it is min_price;
  
   Return value: <br>
    - **void;** <br><br>
  
   **Implementation:** <br>
```plantuml
@startuml

start

:**Arguments:**

**Price** - U512;


if (is **caller** in operators list or maintainer?) then (yes)
else (no)
    :Return error message;
    stop
endif

:Save **Price** in **SimpleOperations** store;

end

@enduml
```

<hr> <br>

3. Endpoint name: *****get_price***** <br>
   
   Arguments: <br>
    - **arg_price_oracle_extension** - Domain name Extension (**String**);
    - **arg_price_oracle_chars_count** - u8;
  
   Return value: <br>
    - **U512;** <br><br>
  
   **Implementation:** <br>
```plantuml
@startuml

start

:**Arguments:**

**Extension** - String
**Chars count** - u8;


if (is **caller** in operators list or maintainer?) then (yes)
else (no)
    :Return error message;
    stop
endif

:Get price from price_fetcher and return it;

end

@enduml
```

<hr> <br>

4. Endpoint name: *****get_price_simple_operations***** <br>
   
   Arguments: <br>    
  
   Return value: <br>
    - **U512;** <br><br>
  
   **Implementation:** <br>
```plantuml
@startuml

start


if (is **caller** in operators list or maintainer?) then (yes)
else (no)
    :Return error message;
    stop
endif

:Get price from price_fetcher and return it;

end

@enduml
```

<hr> <br>