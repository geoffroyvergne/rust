# Rust Rocket Mongo Starter

## Start MongoDB

```docker run --rm -d --name mongo -p 27017:27017 mongo:4.0.9-xenial```

## Create project

```cargo new rust_rocker_diesel_mongo_starter --bin```

## Use nightly version of rust

```rustup override set nightly```
```rustup update && cargo update```

## Run

```cargo run```

```ROCKET_ENV=dev cargo run```
```ROCKET_ENV=stage cargo run```
```ROCKET_ENV=prod cargo run```

```ROCKET_PORT=3000 cargo run```

```ROCKET_DATABASES='{ url = "mongodb://localhost:27017/users" }'```

## Build

```cargo build```

```cargo build --release```

## Test

```curl localhost:8000/index```

# Users

```curl localhost:8000/users```

```
curl -X POST \
    -H 'Content-Type: application/json' \
    -d '{"email":"test2@email.com","first_name":"test2","last_name":"test2"}' \
    localhost:8000/users
```

```curl localhost:8000/users/test2@email.com```

```
curl localhost:3000/users/test2@email.com -X DELETE
```
