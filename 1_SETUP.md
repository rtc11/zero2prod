# Zero To Production
Fokuserer på hvordan man lager *cloud-native* apps for 4 utvikler av ulike kompetansenivå
    * High avaiability
    * Running in fault-prone environments
    * Continuous delivery with zero downtime
    * Handle dynamic workloads
    * Instrumentation/observabiltiy
    * Database and schema migrations
    * Serving APIs
    * Automated tests

Er rust et produktivt programmering-språk for utvikling av APIer? Ja
Hvem er faggruppen for? Backend-utviklere som enten har vært igjennom 'The Rust Book' eller som ikke kan noe Rust fra før.

# Toolchain manager
https://rustup.rs
Installerer og oppdaterer rustc (compiler) og cargo (build system) 

```sh
cargo new zero2prod
# Add new empty repo on GitHub
git add .
git commit -am "Init"
git remote add origin git@github.com:USER/zero2prod.git
git push -u origin  main
```

# IDE
RustRover - by IntelliJ
Rust-analyzer - LSP (VSCode, vim/neovim, emacs, Sublime, etc)

# Inner development loop
    * make a change
    * compile
    * run tests
    * run app

Rust sin compiler er ikke den raskeste, for å kompensere for dette kan man gjøre noen tiltak

## Faster Linking
Bytt ut default rust linker med *lld* fra LLVM.

```sh
brew install llvm
brew info llvm
```

```toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]
```

## Hot Reload
> cargo install cargo-watch
> cargo watch -x check -x test -x run

# Linter
install: rustup component add clippy
run:     cargo clippy

# Formatter
install: rustup component add rustfmt
run:     cargo fmt

# Security vulnerabilities
install: cargo install cargo-audit
run:     cargo audit


