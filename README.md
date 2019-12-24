# oports

![](https://github.com/ndelvalle/oports/workflows/Rust/badge.svg)

Async library to retrieve open ports for a given IP address

## Install

Add `oports` as a dependency in the cargo.toml file if your project:

```toml
[dependencies]
oports = "0.2"
```

If you have [cargo-edit](https://github.com/killercup/cargo-edit) utility tool
installed, use:

```bash
$ cargo add oports
```

## Use

```rust
use Oports;
use std::net::IpAddr;

let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let op = Oports::new(ip_v4_addr);

// Check if the given port is open or not
let is_open_port = op.is_port_open(4040).await;

// Retrieve a vec with open port for a given port range
let open_ports_by_range = op.open_ports_by_range(0, 10).await;

// Retrieve a vec with open port for all ports (0 - 65535)
let open_ports = op.open_ports().await;
```
