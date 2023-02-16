# initia-rust

Rust crate for interacting with [Protobufs] defined by the [Initia].

The goal of this crate is to provide complete proto struct definitions for interacting
with the Initia blockchain based on Cosmos SDK.

Currently, this crate only provides a subset of the many total structs exported by
Initia and Cosmos SDK proto files.

Pull requests to expand coverage are welcome.

NOTICE: prost occurs syntax error due to Validator in cosmos/staking/v1beta1/authz and same one in initia/mstaking/v1/authz.
I think there's no go around to resolve it yet. so manual fix is needed after you run `make proto-gen`.



## Minimum Supported Rust Version

This crate is supported on Rust **1.59** or newer.
