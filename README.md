# Configuration Contract

## Configuration
{
    allowed_chars_count: 3,
    max_entity_count: 1000, // 200 entities per database contract
    allowed_extensions: ["cspr", "csprx"],
    domains_per_page: 20
}

## Contracts List
{
    Registry: [contract1, contract2, ...],
    Database: {
        ext1: [contract1, contract2, ...],
        ext2: [contract1, contract2, ...],
        ...
    },
    NFT: {
        ext1: [contract1, contract2, ...],
        ext2: [contract1, contract2, ...],
        ...
    },
    ...
}



# PriceOracle
{
    ext1: price1,
    ext2: price2,
    ...
}



# Registry
{
    domain1: [Db contract1, nft contract1],
    domain2: [Db contract2, nft contract4],
    domain3: [Db contract3, nft contract4],
    domain4: [Db contract4, nft contract4],
}



# Database

## Domains entity table
{
    domain1: domainEntity1,
    domain2: domainEntity2,
    domain3: domainEntity3,
    domain4: domainEntity4,
    ...
}

## Subdomain entity table
{
    subDomain1: subDomainEntity1,
    subDomain2: subDomainEntity2,
    subDomain3: subDomainEntity3,
    subDomain4: subDomainEntity4,
    ...
}

## Subdomains List for domain name
{
    domain1: [subdomain1, subdomain2, ...],
    domain2: [subdomain1, subdomain2, ...],
    ...
}

## Domains List pagination
{
    0: [domain1, domain2, ...], // in our case 20 per array,
    1: [domain1, domain2, ...], // in our case 20 per array,
    ...
}

## Owner's domains list
{
    owner1: [domain1, domain2, ...],
    owner2: [domain1, domain2, ...],
}

## Allowed authorities for domain
{
    domain1: [authority1, authority2, ...],
    domain2: [authority1, authority2, ...],
    ...
}