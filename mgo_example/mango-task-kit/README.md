# Mango Task Kit

This developer example follows an EVM event through task admission, issuance of a nonfinancial test entitlement on Mango, and a queryable receipt. PostgreSQL retains the original target transaction digest so an uncertain outcome can be reconciled against the original transaction. The test entitlement package binds each task ID to one recipient and payload.

## Run the local checks

Use Docker, Python 3.10 or newer, and the Rust stable toolchain. Run these commands from `mgo_example/mango-task-kit`. The reference build used Rust 1.96.0.

```sh
cargo +stable build --locked --bin mango-local-task-kit
python3 examples/smoke.py --binary target/debug/mango-local-task-kit
python3 examples/check_worker_recovery.py
```

The smoke test starts temporary PostgreSQL 16 and applies the migrations before opening the HTTP service on loopback. It checks the browser page, synthetic task admission, reader and operator permissions, and replay of a reconciliation request. It prints the results and cleans up the temporary services. The `--postgres-image` option accepts an alternative PostgreSQL 16 image already available to Docker.

Run the entitlement package tests with a Mango CLI compatible with this repository. With Python's `cryptography` package installed, the witness tests can run separately.

```sh
mgo move test --path entitlement --skip-fetch-latest-git-deps
python3 -m unittest witness.test_auth
```

## Read the testnet receipts

The testnet reference run used BNB Smart Chain Testnet, chain ID `97`, and Mango Testnet2, chain identifier `88b02231`. The command below checks the two published receipt pairs through public RPC. It verifies inclusion and finality, the expected source contract and Mango package, and the matching event payload and task ID.

```sh
python3 examples/check_testnet_receipts.py
```

[Verification report](VERIFICATION.md) records the original testnet receipts and checkpoints, the database restore exercise, and what was covered by the historical candidate. The local smoke test above covers this source checkout. A new end-to-end testnet run uses approved source and issuer accounts, a registered witness, and a testnet deployment configured for the pilot.

## Worker restart and recovery

Apply migrations in order through `0004_submission_dispatch.sql` before using the local worker. The worker durably records dispatch before calling the target. A replacement worker with a valid lease can submit the original signed bytes when the intent was recorded by the updated worker and dispatch has yet to start. It uses digest lookup for legacy intents and attempts that have already crossed the dispatch boundary.

If the worker stops after recording the dispatch marker but before the RPC request reaches Mango, the task may remain pending indefinitely. On restart, the worker looks up the original transaction digest but does not automatically rebroadcast the signed transaction.

The recovery check uses disposable PostgreSQL and a synthetic target. It expires the first worker's lease, checks that the replacement worker keeps the same signed bytes and attempt count, and exercises the lookup path for both legacy and marked intents.

## Review the implementation

| Area | Files |
| --- | --- |
| Task state, leases, intents, and receipts | [PostgreSQL migrations](migrations/) |
| HTTP contract, service, and browser view | [OpenAPI](contracts/openapi.json), [service](src/main.rs), [browser assets](ui/) |
| Loopback EVM scanner and replay | [Source fixture and scanner](source/) |
| Target reconciliation and transaction digest | [Local target adapter](target_adapter/), [digest utility](src/bin/mango-transaction-digest.rs) |
| Testnet source event and target entitlement | [Solidity contract](evm/S6TestEntitlementSource.sol), [Move package](entitlement/) |
| Testnet witness message and signature checks | [Witness implementation](witness/) |

To use the loopback EVM fixture independently, install its locked Node dependencies with `npm ci --prefix source/fixture`. The fixture runner accepts the fixture directory, a target configuration JSON file, and a new output path for the connection details. The scanner and replay utilities accept that fixture manifest and a loopback RPC URL. Their `--help` output lists the PostgreSQL environment and command inputs. The Move package resolves the Mango framework from the repository.

The implementation uses a single configured route and an explicitly authorized operational witness. The source and target observations in the testnet report are RPC evidence. The PostgreSQL backup exercise restores the task store while finalized chain effects retain their original history.

The Mango repository license applies to this example. Rust and Node dependencies are recorded in the corresponding lockfiles.
