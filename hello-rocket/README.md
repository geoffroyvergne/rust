# Rust Cargo rest api

## Create project

```cargo new hello_cargo --bin```

## Use nightly version of rust

```rustup default stable```

```rustup override set nightly```
```rustup update && cargo update```

## Run

```cargo run```
```ROCKET_CLI_COLORS=off cargo run```

```ROCKET_ENV=dev cargo run```
```ROCKET_ENV=stage cargo run```
```ROCKET_ENV=prod cargo run```

```ROCKET_PORT=3000 cargo run```

## Build

```cargo build```

```cargo build --release```

