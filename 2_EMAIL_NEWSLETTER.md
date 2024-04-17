# EPOST NYHETER

## In scope
Som en leser,
Ønsker jeg å melde meg på nyhetsbrevet,
Slik at jeg kan få epost når nye artikler er publisert.

Som en forfatter,
Ønsker jeg å sende epost til alle lesere,
Slik at jeg kan gi beskjed når nye artikler er publisert.

## Out of scope
* avmelding
* fler ulike nyhetsbrev
* instrummentere leser-klikk og antall åpnede artikler

# Påmelde en ny leser
Vi må lage et endepunkt: `POST /subscriptions`

Dette krever bl.a: 
* Valg av web-rammeverk
* Definere test-strategi
* Valg av database-rammeverk
* Hvordan vi håndterer database-endringer
* Skrive database-spørringer


## 1. Helsesjekk
Det finnes mange gode web-rammeverk, inkl `actix-web, axum, poem, tide, rocket, etc`.
En av de elste er ´actix-web`, som har mest produksjonserfaring, fint utgangspunkt.

### Documentation
https://actix.rs
https://docs.rs/actix-web/latest/actix_web/
https://github.com/actix/examples

```rust

```

