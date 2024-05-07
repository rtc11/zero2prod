# INTEGRATION TEST
Slike tester sørger for at vi fanger opp at vi er *bakoverkompatible* eller om vi introduserer *breaking changes*.
Actix-web har noen innebygde funksjoner, macros osv for å gjøre dette enklere.
MEN - det kan være smart å være rammeverk-agnostisk, slik at man ikke trenger å skrive om de viktigste testene man 
har ved eventuelt bytte av rammeverk.
DEN beste løsningen blir å bruke en HTTP client, e.g. *reqwest*.

Det finnes 3 steder å legge testene.
1. I samme fil som koden.
2. I en egen fil under /tests
3. Som en del av dokumentasjonen (doc test)

Hovedforskjellen er at embedded test (1) er en del av koden, så den kan få tilgang til private funksjoner.
De andre to må bruke public API, og er derfor godt egnet for integrasjonstester.

```
//! tests/health.rs

use zero2prod::main;

#[test]
fn dummy() {
    main()
}
```

For å få dette til å funke må man gjøre om prosjektet fra (default) binary til shared library.
Vi må også slenge på hvor main er:

```
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "zero2prod"
```

