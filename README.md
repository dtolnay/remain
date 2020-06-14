Remain sorted
=============

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/remain-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/remain)
[<img alt="crates.io" src="https://img.shields.io/crates/v/remain.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/remain)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-remain-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/remain)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/remain/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/remain/actions?query=branch%3Amaster)

This crate provides an attribute macro to check at compile time that the
variants of an enum or the arms of a match expression are written in sorted
order.

```toml
[dependencies]
remain = "0.2"
```

## Syntax

Place a `#[remain::sorted]` attribute on enums, structs, match-expressions, or
let-statements whose value is a match-expression.

Alternatively, import as `use remain::sorted;` and use `#[sorted]` as the
attribute.

```rust
#[remain::sorted]
#[derive(Debug)]
pub enum Error {
    BlockSignal(signal::Error),
    CreateCrasClient(libcras::Error),
    CreateEventFd(sys_util::Error),
    CreateSignalFd(sys_util::SignalFdError),
    CreateSocket(io::Error),
    DetectImageType(qcow::Error),
    DeviceJail(io_jail::Error),
    NetDeviceNew(virtio::NetError),
    SpawnVcpu(io::Error),
}

#[remain::sorted]
#[derive(Debug)]
pub struct Registers {
    ax: u16,
    cx: u16,
    di: u16,
    si: u16,
    sp: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        #[remain::sorted]
        match self {
            BlockSignal(e) => write!(f, "failed to block signal: {}", e),
            CreateCrasClient(e) => write!(f, "failed to create cras client: {}", e),
            CreateEventFd(e) => write!(f, "failed to create eventfd: {}", e),
            CreateSignalFd(e) => write!(f, "failed to create signalfd: {}", e),
            CreateSocket(e) => write!(f, "failed to create socket: {}", e),
            DetectImageType(e) => write!(f, "failed to detect disk image type: {}", e),
            DeviceJail(e) => write!(f, "failed to jail device: {}", e),
            NetDeviceNew(e) => write!(f, "failed to set up virtio networking: {}", e),
            SpawnVcpu(e) => write!(f, "failed to spawn VCPU thread: {}", e),
        }
    }
}
```

If an enum variant, struct field, or match arm is inserted out of order,

```diff
      NetDeviceNew(virtio::NetError),
      SpawnVcpu(io::Error),
+     AaaUhOh(Box<dyn StdError>),
  }
```

then the macro produces a compile error.

```console
error: AaaUhOh should sort before BlockSignal
  --> tests/stable.rs:49:5
   |
49 |     AaaUhOh(Box<dyn StdError>),
   |     ^^^^^^^
```

## Compiler support

The attribute on enums and structs is supported on any rustc version 1.31+.

Rust does not yet have stable support for user-defined attributes within a
function body, so the attribute on match-expressions and let-statements requires
a nightly compiler and the following two features enabled:

```rust
#![feature(proc_macro_hygiene, stmt_expr_attributes)]
```

As a stable alternative, this crate provides a function-level attribute called
`#[remain::check]` which makes match-expression and let-statement attributes
work on any rustc version 1.31+. Place this attribute on any function containing
`#[sorted]` to make them work on a stable compiler.

```rust
impl Display for Error {
    #[remain::check]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        #[sorted]
        match self {
            /* ... */
        }
    }
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
