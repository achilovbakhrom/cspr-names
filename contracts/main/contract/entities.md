

@startuml
json Domain {
    "name":"some_name.cspr",
    "end_time": 167884231313,
    "owner": "<SomeOwner>",
    "resolver": "<SomeCasperAddress>"
}
json Subdomain {
    "name": "sub.some_name.cspr",
    "resolver": "<SomeCasperAddress>"
}
json DomainList {
    "names": ["domain1.cspr", "domain2.cspr", "domain3.cspr", "...", "domainN.cspr"],
    "total": "N"
}
json DomainData {
    "name": "some_name.cspr",
    "data": [
        {
            "key": "some_key",
            "value_type": "Phone",
            "value": "+998900000000"
        },
        {
            "key": "some_key",
            "value_type": "Phone",
            "value": "+998900000000"
        },
        {
            "key": "some_key",
            "value_type": "Phone",
            "value": "+998900000000"
        }
    ]
}
json DomainMetadata {
    "totalCount": 101,
    "latestPage": 20,
    "latestItemNo": 1
}
json SubDomainMetadata {
    "totalCount": 101,
    "latestPage": 20,
    "latestItemNo": 1
}

@enduml


