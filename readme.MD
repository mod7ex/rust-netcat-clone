# net-cat clone using Rust

## [openssl docs](https://man.openbsd.org/openssl.1)

---

## SSL

> `cargo run serve localhost --port 3000 --cert ./certs/server.pem --key ./certs/server-key.pem`

> `cargo run connect localhost --port 3000 --ca ./certs/ca.pem`

---

## Command

> `cargo run serve localhost --port 3000 --cmd sh`

> `cargo run connect localhost --port 3000`
