## 1) **DomainList** - Complex Store

- **DomainListStore** - Dictionary

    Key: **KEY_DATABASE_DICTIONARY_DOMAIN_LIST**<br>
    Value: 
    ```plantuml
    @startuml
    <p>Value</p>
    @startjson
    {
        "0 (Pagination)": "Vec<String>([test.cspr, test2.cspr])"
    }
    @endjson
    @enduml
    ```

- **StateStore** - KeyValueStore <br>
    Key: **KEY_DATABASE_DOMAIN_LIST_PAGINATION**<br>
    Value: **String** (current pagination number)

<br><hr><br>

## 2) **DomainMap** - Dictionary

Key: **KEY_DATABASE_DICTIONARY_DOMAIN**
Value:
 ```plantuml
@startuml
<p>Value</p>
@startjson
{
    "test.cspr": "class DomainName"
}
@endjson
@enduml
```

<br><hr><br>

## 3) **DomainPaginationMap** - Dictionary

Key: **KEY_DATABASE_DICTIONARY_DOMAIN_MAP**Value:
Value:
 ```plantuml
@startuml
<p>Value</p>
@startjson
{
    "test.cspr": "0 (Page)"
}
@endjson
@enduml
```

<br><hr><br>

## 4) **OwnerDomainList** - Dictionary

Key: **KEY_DATABASE_DICTIONARY_OWNER_DOMAIN_LIST**
Value:
 ```plantuml
@startuml
<p>Value</p>
@startjson
{
    "owner <AccountHash>": "Vec<String> [test.cspr, test1.cspr]"
}
@endjson
@enduml
```

<br><hr><br>

## 5) **OwnerDomainList** - KeyVallueStore

Key: **KEY_DATABASE_TOTALS_DOMAIN_COUNT**
Value: **u64** (Total DomainCount in URef of Smartcontract)

<br><hr><br>

## 6) **SubdomainList** - Dictionary

Key: **KEY_DATABASE_DICTIONARY_SUBDOMAIN_LIST**
Value:
 ```plantuml
@startuml
<p>Value</p>
@startjson
{
    "test.cspr": "Vec<String> [sub1.test.cspr, sub2.test.cspr]"
}
@endjson
@enduml
```

<br><hr><br>

## 7) **SubdomainMap** - Dictionary

Key: **KEY_DATABASE_DICTIONARY_SUBDOMAIN**
Value:
 ```plantuml
@startuml
<p>Value</p>
@startjson
{
    "sub.test.cspr": "class SubdomainName"
}
@endjson
@enduml
```

<br><hr><br>