# Mango Task Kit verification

The Task Kit follows a source event on BNB Smart Chain Testnet through to a test entitlement and receipt on Mango Testnet2. The local checks used isolated services. The testnet checks used BNB Smart Chain Testnet chain ID `97` and Mango Testnet2 chain identifier `88b02231`. [Pilot request #68](https://github.com/MangoNet-Labs/mango/issues/68)

## Candidate and coverage

The historical checks used Mango commit `72b889a872d7c71e2dd9886f956fbc3cfff1789e` as their upstream baseline. The S5 local preview manifest has SHA-256 `0b58f8f691246d9df550063db47b4924241ace663f91c6f16ae96616364c73d3`. The historical preview used Apple Silicon macOS binaries, an isolated Mango chain, and simulated source events and authorization.

The results below apply to the historical candidates used in each check. For the source package in this PR, use the pilot README to run the local checks and record the PR commit, command, and result. G4 still requires two independent pilot integrations and an authorized maintainer's sign-off.

## Source checkout checks

- Built the HTTP service with Rust 1.96.0 and the committed Cargo lockfile.
- Passed the smoke test on PostgreSQL 16.14. The English browser page returned HTTP 200, the synthetic receipt was pending, and repeating a `RECONCILE_ONLY` request preserved the original target digest and one target attempt.
- Passed the worker restart regression with Python 3.10 and PostgreSQL 16.14. After the first lease expired, the replacement worker dispatched the original signed bytes once through a synthetic target. Both legacy intents and attempts with a dispatch marker used digest lookup.
- Passed three Move tests for same-task replay and conflicting digest or recipient, and six witness verification tests.
- Ran the loopback EVM fixture with the previously installed locked dependencies. The first call emitted one event and replaying it emitted zero new events.
- Rechecked both published testnet receipt pairs, including the finalized BNB blocks, Mango checkpoints, expected contracts and event types, and matching payload digest and task ID.

## Local acceptance

| Stage | Result | Checks |
| --- | --- | --- |
| S3 | 5 of 5 passed | API contract and receipt evidence roles, isolated Move package publication, issuance and replay, and source authorization with a fixture. Replaying the same task left the registry count at one. |
| S4 | 4 of 4 passed | PostgreSQL migrations, client integration, HTTP and browser behavior, and recovery of an unknown target outcome by looking up the original digest. |
| S5 | 24 of 24 passed | 22 local acceptance cases, reconciliation, and independent reproduction. Across two full replays, five tasks matched the recorded chain effects, with zero additional target broadcasts. |

## Testnet acceptance

The independent G3 review confirmed all five cases.

| Case | Result | Observation |
| --- | --- | --- |
| AC-22 | Pass | Witness rotation at BNB block `145000427` preserved prior receipts. Mango checkpoints `58279421` and `58279642` contain the two confirmed effects. |
| AC-23 | Pass | An operator withdrawal exercise preserved the original finalized evidence and created an incident record. The run submitted zero target transactions. |
| AC-26 | Pass | A conflicting route returned `ROUTE_MIGRATION_REQUIRED`. The run submitted zero target transactions. |
| AC-27 | Pass | Both cursors progressed toward fixed reference heads. The moving BNB head still led the source cursor by 420 blocks at the final observation. These readings came from trusted RPC responses. |
| AC-28 | Pass | Twelve HTTP checks covered reader and operator permissions, and the operator action was recorded as `RECONCILE_ONLY`. The source event finalized at BNB block `145007574`, with zero target submissions. |

## Public testnet receipts

| BNB source transaction hash | BNB block | Mango target transaction digest | Mango checkpoint |
| --- | ---: | --- | ---: |
| `0xe8a5910b2ac74f8226ad25155c6a819271c38b46d98afcdfac2758fc9f98927b` | 144998369 | `1BUfuaWiuxCeo2kVn7kQsGPF8wCzJuqpQHdmixsb5R6` | 58279421 |
| `0xca5d7fa0dd171c8b25c4f387b1bb9f8a75deb5c133c656b45a887af50c2989e6` | 145000790 | `4naN4pLYra1ncWz3Xy4DU5We2QuoUUhGivnHyvRaNrAN` | 58279642 |

## PostgreSQL migration and recovery

AC-21 passed. The exercise upgraded an N−1 schema and independently restored a retained backup into a fresh database. Four business-state snapshots matched, including both testnet receipts and their recorded effects. A synthetic no-send fixture checked queue replay. Readiness returned `200 → 503 → 200` during the dependency fault and recovery, and liveness stayed at `200`. The exercise sent zero chain transactions. The recovery restores database state while confirmed on-chain effects and source finality retain their original history.
