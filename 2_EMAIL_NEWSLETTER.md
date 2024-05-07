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

### Implementation
```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
   })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### HttpServer
Handles *sockets*, *connections*, *rate limits*, *TLS*, etc,

### App
Routing, middleware, request handlers, etc.

### Routes
Path with optional template (e.g. `/{name}`).
Handler function (e.g. `greet`) - noe som implementerer `Responder` trait.
Guards (kriterie for å bruke en handler, f.eks web::get()).

### Runtime (tokio)
Async runtime. `#[]` betyr at det er en prosedyre-makro.
Vi kan se hva makroen gjør ved å installere nightly og cargo-expand:
> rustup toolchain install nightly
> cargo install cargo-expand
> cargo +nightly expand



