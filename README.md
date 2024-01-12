<!-- markdownlint-disable -->
<div align="center">
<img src="https://github.com/KasarLabs/brand/blob/main/projects/ditto/logo.png?raw=true" height="200">
</div>
<div align="center">
<br />
<!-- markdownlint-restore -->

[![Project license](https://img.shields.io/github/license/kasarLabs/ditto.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/kasarLabs/ditto/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
<a href="https://twitter.com/KasarLabs">
<img src="https://img.shields.io/twitter/follow/KasarLabs?style=social"/> </a>
<a href="https://github.com/kasarlabs/ditto">
<img src="https://img.shields.io/github/stars/kasarlabs/ditto?style=social"/>
</a>

</div>

# ⚙️ Ditto: Library to test and benchmark Starknet full nodes

_A simple Rust RPC test utility for the [Deoxys](https://github.com/KasarLabs/deoxys) Starknet node._

## Getting started

> 💡 Before you get started, make sure you have access to an active Deoxys and Pathfinder node.

For tests to work, you will need to specify an **pathfinder api url** and **deoxys api url**. These must be stored in `./unit_tests/secret.json`.

> ⚠️ Make sure to **never commit or share your api keys** in `./unit_tests/secret.json`.

*secret.json format:*
```json
{
    "pathfinder": "pathfinder-node-url",
    "deoxys": "deoxys-node-url"
}
```

## Writing unit tests

Unit tests should be written inside of `./unit_test/tests/`, but nothing stops you from creating your own module. Just make sure to import the necessary dependencies, which are:

### Guidelines

> 💡 You can find a module for common imports in `./unit_tests/tests/common.rs

- Failure case tests should be prefixed with `fail_`, success tests should be prefixed with `work_`.
- Test should be well documented, following [Rustdoc](https://doc.rust-lang.org/rustdoc/index.html) guidelines.
- Any recurring initialization logic should be extracted to `./unit_tests/src/fixtures.rs`. See [rstest](https://docs.rs/rstest/latest/rstest/) for more information on how to create fixtures.
- Any constants should be extracted to `./unit_tests/src/constants.rs`, along with proper [Rustdoc](https://doc.rust-lang.org/rustdoc/index.html) documentation. If the constant contains a block hash, transaction hash, or other Starknet hashed location, please include a link to the hash's page on [StarkScan](https://starkscan.co/).

- Try and test as many edge cases as possible. Ths mostly includes optional parameters. You can find a list
of Starknet RPC call and their arguments / return data [here](https://playground.open-rpc.org/?uiSchema%5BappBar%5D%5Bui:splitView%5D=false&schemaUrl=https://raw.githubusercontent.com/starkware-libs/starknet-specs/master/api/starknet_api_openrpc.json&uiSchema%5BappBar%5D%5Bui:input%5D=false&uiSchema%5BappBar%5D%5Bui:darkMode%5D=true&uiSchema%5BappBar%5D%5Bui:examplesDropdown%5D=false)

### Example Test

*./unit_tests/tests/test_block_number.rs*
```rust
mod common;
use common::*;

use std::collections::HashMap;

use starknet_providers::{jsonrpc::{HttpTransport, JsonRpcClient}, Provider};

///
/// Unit test for `starknet_getBlockNumber`
/// 
/// purpose: get block number on latest block
/// success case: retrieve correct block number
/// 
#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.block_number().await.expect("Deoxys : Error while getting the block number");
    let response_pathfinder = pathfinder.block_number().await.expect("RPC : Error while getting the block number");

    assert_eq!(response_deoxys, response_pathfinder);
}