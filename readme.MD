# net-cat clone using Rust

## https://man.openbsd.org/openssl.1

`cargo run serve localhost --port 3000 --cert ./certs/server.pem --key ./certs/server-key.pem`

`cargo run connect localhost --port 3000 --ca ./certs/ca.pem`