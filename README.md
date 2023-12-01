# Porta BackEnd #
A simple backend written un Rust-Lang.

## To build and test the code: ##

* Run Server
> ``` cargo watch -q -c -w src/ -w .cargo/ -x run ```

* Run Tests
> ``` cargo watch -q -c -w examples/ -x "run --example  quick_dev -- --nocapture" ```

* Run single test
> ``` cargo watch -q -c -x "test test_create_ok -- --nocapture" ```

* Run model tests
> ``` cargo watch -q -c -x "test model::bounty::tests -- --nocapture" ```


## Starting the DB, it must be via Docker if using Ubuntu
#  Postgres version must be >= 13
``` sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
    -e POSTGRES_PASSWORD=welcome \
    postgres:15
