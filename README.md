<h1 align="center">oports</h1>
<div align="center">
 <strong>Rust library to asynchronously retrieve open ports for a given IP address</strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/ndelvalle/oports/actions?query=workflow%3ARust">
    <img src="https://img.shields.io/github/workflow/status/ndelvalle/oports/Rust?style=flat-square" alt="actions status" />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/oports">
    <img src="https://img.shields.io/crates/v/oports.svg?style=flat-square" alt="Crates.io version" />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/oports">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" />
  </a>
</div>


## Install

Add `oports` as a dependency in the cargo.toml file of your project:

```toml
[dependencies]
oports = "1.0.0"
```

If you have [cargo-edit](https://github.com/killercup/cargo-edit) utility tool
installed, use:

```bash
$ cargo add oports
```

## Interface

#### is_port_open(ip: IpAddr, port: u16) -> bool

Check if the given port is open for a specified IP address.

```rust
use oports;
use std::net::IpAddr;

let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let is_open = oports::is_port_open(localhost, 4040).await;
```

#### get_open_ports(ip: IpAddr, ports: Vec<u16>, concurrency: Option<usize>) -> Vec<u16>

Retrieves a vec with open ports for a given vec of port numbers an IP addresses.
The default concurrency is `100` if the `Option` resolves to a `None` value.

```rust
use oports;
use std::net::IpAddr;

let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let ports = (8000..9090).collect::<Vec<u16>>();
let concurrency = Some(120)
let open_ports = oports::get_open_ports(localhost, ports, concurrency).await;
```

#### get_all_open_ports(ip: IpAddr, concurrency: Option<usize>) -> Vec<u16>

Retrieves a vec with all open ports for a given IP address. The default concurrency
is `100` if the `Option` resolves to a `None` value.

```rust
use oports;
use std::net::IpAddr;

let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let ports = (8080..u16::max_value()).collect::<Vec<u16>>();
let concurrency = None // Default to 100
let all_open_ports = oports::get_all_open_ports(localhost, concurrency).await;
```

## License
[MIT License](https://github.com/ndelvalle/oports/blob/master/LICENSE)
