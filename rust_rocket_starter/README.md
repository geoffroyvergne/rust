# Rust Rocket Starter

## Create project

```cargo new rust_rocker_starter --bin```

## Use nightly version of rust

```rustup override set nightly```
```rustup update && cargo update```

## Run

```cargo run```

```ROCKET_ENV=dev cargo run```
```ROCKET_ENV=stage cargo run```
```ROCKET_ENV=prod cargo run```

```ROCKET_PORT=3000 cargo run```

## Build

```cargo build```

```cargo build --release```

## Test

```curl localhost:8000/index```
