# Stellar Oracle Shield Threat Model

Status: Proposed  
Last reviewed: 2026-09-03  
Method: [SDF STRIDE threat-modeling guidance](https://developers.stellar.org/docs/build/security-docs/threat-modeling)

## Scope and assumptions

Stellar Oracle Shield is a risk-signal and circuit-breaker contract, not a price oracle. An off-chain monitor derives a health score from price-oracle and market data; the configured operator publishes that score for a `(base, quote)` pair. The Soroban contract stores the latest score and publication timestamp, converts the score to `Healthy`, `Degraded`, or `Unsafe`, and allows consuming DeFi protocols to decide how to react.

This model covers the contract in this repository, its deployment and upgrade path, the operator publication path, and its integration by a consuming protocol. The off-chain monitor is shown because it is a security-critical dependency, although its implementation is outside this repository. Controls attributed to it below are therefore requirements, not claims about existing code.

Security objectives are:

- Only the configured operator can publish scores, and only the administrator can change security-critical configuration or upgrade the contract.
- A returned score/status is authentic, in range, associated with the requested ordered pair, and sufficiently recent.
- Consumers can distinguish healthy, degraded, unsafe, missing, and stale inputs and fail closed where appropriate.
- Administrative, operator, publication, and upgrade actions are attributable and observable.
- The service remains available within reasonable adversarial and upstream-failure conditions.

Primary assets are the administrator and operator signing keys, monitoring inputs and scoring logic, contract WASM and configuration, per-pair scores and timestamps, contract identity/network configuration, emitted events, and the funds or protocol decisions that depend on the signal.

Out of scope are Stellar consensus and cryptography, vulnerabilities in an independent price oracle or consuming protocol except where trust in Shield creates an exploitable interaction, and end-user wallet security beyond the administrator/operator key-handling requirements.

## 1. Dataflow diagram

```mermaid
flowchart TB
  subgraph U["Untrusted / external systems"]
    MD["Market-data sources"]
    PO["Price oracles"]
    A["Attacker"]
  end

  subgraph O["Shield operator boundary (off-chain; not in repository)"]
    RM["Risk monitor and scoring engine"]
    OK["Operator signer / key custody"]
    OL[("Logs, observations, model/config history")]
  end

  subgraph S["Stellar network / Soroban boundary"]
    SC["Oracle Shield contract"]
    CS[("Instance storage: admin, operator, max staleness")]
    PS[("Temporary storage: pair score and timestamp")]
    EV[("Contract events / ledger history")]
    CP["Consuming DeFi protocol"]
  end

  subgraph G["Governance / release boundary"]
    AD["Administrator signer / key custody"]
    CI["Reviewed build and WASM artifact"]
  end

  MD -->|"trades, reference prices, liquidity"| RM
  PO -->|"oracle observations"| RM
  RM -->|"score proposal"| OK
  RM -->|"inputs, computation, alerts"| OL
  OK -->|"authorized set_score transaction"| SC
  AD -->|"set operator, set staleness, upgrade"| SC
  CI -->|"WASM hash approved by administrator"| SC
  SC <--> CS
  SC <--> PS
  SC -->|"StatusChange"| EV
  CP -->|"get_status or get_score"| SC
  SC -->|"status/score or explicit error"| CP
  PO -->|"price used for protected operation"| CP
  A -.->|"manipulation, spoofing, flooding, key theft"| MD
  A -.-> PO
  A -.-> RM
  A -.-> SC
  A -.-> CP
```

Trust boundaries and important flows:

1. External data crosses into the off-chain monitor and must be treated as adversarial.
2. The operator signer converts an off-chain conclusion into an authenticated on-chain update.
3. The administrator can replace the operator, alter freshness policy, and replace all contract code; it is the highest-impact role.
4. Contract state and events inherit Stellar ledger integrity, but temporary entries have a finite lifetime and require lifecycle management.
5. A consumer crosses a separate integration boundary: Shield recommends a state, while the consumer implements the actual circuit breaker.

## 2. Threat table

| Threat | Issues |
|---|---|
| Spoofing | **Spoof.1** — Theft or misuse of the operator key lets an attacker publish arbitrary health scores for any pair.<br>**Spoof.2** — Theft or misuse of the administrator key lets an attacker replace the operator, weaken freshness policy, or install malicious WASM.<br>**Spoof.3** — A consumer can query the wrong contract ID, Stellar network, or look-alike asset address and believe the response came from the intended Shield/pair.<br>**Spoof.4** — The off-chain monitor may accept a spoofed market-data/oracle endpoint or unauthenticated response as a legitimate source. |
| Tampering | **Tamper.1** — Manipulated, stale, selectively withheld, or low-liquidity source data can cause the monitor to compute an unsafe score as healthy.<br>**Tamper.2** — A compromised monitor-to-signer pipeline can substitute the pair or score before signing.<br>**Tamper.3** — A malicious or unreviewed upgrade can change authorization, score boundaries, storage layout, or freshness behavior.<br>**Tamper.4** — Configuration changes can set `max_staleness` to an unsafe value (including zero or effectively unbounded) or assign an unintended operator.<br>**Tamper.5** — Base/quote reversal, duplicate asset addresses, or unsupported asset addresses can cause a score to be stored or consumed under unintended semantics. |
| Repudiation | **Repudiate.1** — An operator or administrator may deny publishing a score or changing configuration; the current contract emits only status-boundary changes and no events for every score/config/upgrade action.<br>**Repudiate.2** — The off-chain system may be unable to reproduce why a score was generated if source observations, scoring version, configuration, and transaction hash are not retained.<br>**Repudiate.3** — A consuming protocol may be unable to prove which Shield result and oracle price it used for a financially significant action. |
| Information Disclosure | **Info.1** — Operator/admin seed phrases, signing material, CI credentials, or local Stellar CLI identities may leak through source control, logs, shell history, build artifacts, or compromised runners.<br>**Info.2** — Monitor logs or telemetry may expose paid-provider credentials, internal endpoints, incident-response data, or sensitive operational thresholds.<br>**Info.3** — Public score updates and events reveal monitored pairs and health transitions, enabling adversaries to time attacks or infer risk policy. This is inherent to an on-chain public signal. |
| Denial of Service | **DoS.1** — Upstream oracle/data-source outages, rate limits, or network partitions stop fresh scores; after `max_staleness`, all protected actions may fail closed.<br>**DoS.2** — Exhaustion of the operator account balance, fee spikes, sequence conflicts, or transaction flooding delays score publication.<br>**DoS.3** — Temporary score entries may expire if their TTL is not explicitly extended, causing covered pairs to become `PairNotCovered` even while operators believe them active.<br>**DoS.4** — An administrator can accidentally or maliciously set an impractically short freshness window or an unusable operator, halting updates/consumption.<br>**DoS.5** — Excessive pair cardinality or update frequency can exhaust monitor, signer, RPC, or budget capacity and crowd out critical pairs. |
| Elevation of Privilege | **Elevation.1** — A defect in authorization or an upgrade could allow a public caller to invoke operator/admin capabilities.<br>**Elevation.2** — Compromise of the CI/release/deployment environment can turn code-contribution or runner access into control over an approved WASM hash or deployment identity.<br>**Elevation.3** — A single administrator has unilateral authority to rotate the operator, change freshness, and upgrade code; compromise of that role becomes full protocol control.<br>**Elevation.4** — A consuming protocol may treat `Healthy` as authorization rather than advisory input, allowing the Shield operator to indirectly trigger privileged financial actions beyond its intended role. |

## 3. Threat answer table

Status values below use **Implemented** for controls visible in this repository, **Required** for controls that should be added or enforced operationally, and **Accepted** for deliberate residual risk.

| Threat | Answers / treatments | Status |
|---|---|---|
| Spoof.1 | **Spoof.1.R.1** — Keep `operator.require_auth()` on every score write and add regression tests for unauthorized callers.<br>**Spoof.1.R.2** — Use a dedicated, least-privilege operator account held in an HSM/MPC or hardened signer; never place its secret in application config or CI logs.<br>**Spoof.1.R.3** — Monitor all score transactions, alert on unusual pairs/rates/values, maintain a funded standby signer, and document rapid operator rotation. | R.1 Implemented; R.2–R.3 Required |
| Spoof.2 | **Spoof.2.R.1** — Preserve administrator authentication for configuration and upgrades.<br>**Spoof.2.R.2** — Put administration behind multisig/contract-account governance with separate approvers, hardware-backed keys, and an emergency recovery procedure.<br>**Spoof.2.R.3** — Alert on every administrator action and verify the expected signer and change ticket. | R.1 Implemented; R.2–R.3 Required |
| Spoof.3 | **Spoof.3.R.1** — Pin network passphrase, canonical Shield contract ID, expected `version()`, and canonical SAC addresses in each integration and deployment manifest.<br>**Spoof.3.R.2** — Reject network/contract/version mismatches at startup and in integration tests; display abbreviated IDs only in addition to, never instead of, full machine validation. | Required |
| Spoof.4 | **Spoof.4.R.1** — Authenticate upstreams with TLS and provider credentials, validate certificates and response schemas, and prohibit redirects to untrusted hosts.<br>**Spoof.4.R.2** — Combine independent providers/venues and source classes; do not allow one endpoint identity to count as multiple independent observations. | Required |
| Tamper.1 | **Tamper.1.R.1** — Use robust multi-source aggregation, minimum liquidity/source-count rules, deviation and rate-of-change bounds, stale-input rejection, and asset-specific thresholds.<br>**Tamper.1.R.2** — Fail safe (`Unsafe` or no publication) when quorum/confidence is insufficient; never reuse a last-known-good score without clearly bounded age.<br>**Tamper.1.R.3** — Continuously compare sources and alert/quarantine outliers; adversarially test thin-market, flash-manipulation, and coordinated-source scenarios. | Required (off-chain) |
| Tamper.2 | **Tamper.2.R.1** — Have the signer independently validate score range, allowlisted ordered pair, monotonic/fresh observation time, scoring-policy version, and a digest of the monitor output before signing.<br>**Tamper.2.R.2** — Bind monitor output to the transaction with authenticated IPC or a signed, replay-protected message; isolate monitor and signer privileges. | Required |
| Tamper.3 | **Tamper.3.R.1** — Require reproducible builds, locked dependencies, tests/audit, and multiple approvals for the exact WASM hash before upgrade.<br>**Tamper.3.R.2** — Test storage compatibility and all authorization/freshness invariants before upgrade; publish the approved source commit and hash.<br>**Tamper.3.R.3** — Prefer timelocked/multisig upgrades with monitoring and a rehearsed recovery release. | Required |
| Tamper.4 | **Tamper.4.R.1** — Add contract bounds for `max_staleness` and reject zero or operationally unsafe values; emit configuration-change events.<br>**Tamper.4.R.2** — Validate a new operator is intentional and usable, require two-step rotation or governance approval, and exercise a post-change publication check. | Required |
| Tamper.5 | **Tamper.5.R.1** — Reject `base == quote`; optionally maintain an allowlist of supported canonical SAC pairs and an explicit orientation convention.<br>**Tamper.5.R.2** — Add tests proving `(A,B)` and `(B,A)` are distinct and consumer configuration uses the intended order. | Required |
| Repudiate.1 | **Repudiate.1.R.1** — Emit events for every score publication (including score, pair, timestamp/policy version), operator rotation, freshness change, and upgrade hash; retain transaction and ledger identifiers in an indexed audit log.<br>**Repudiate.1.R.2** — The existing `StatusChange` event remains useful but is insufficient alone because same-status score updates leave no contract event. | Partial; enhancement Required |
| Repudiate.2 | **Repudiate.2.R.1** — Store immutable or append-only records of normalized source observations, source timestamps, scoring code/config/model version, output, signer identity, and submission transaction hash.<br>**Repudiate.2.R.2** — Apply clock synchronization, retention policy, access control, integrity hashes, and periodic restore/replay tests. | Required |
| Repudiate.3 | **Repudiate.3.R.1** — Consuming protocols should emit/store the Shield contract ID, pair, returned status/error, ledger sequence, oracle value, and policy branch used for each protected action. | Required (consumer) |
| Info.1 | **Info.1.R.1** — Use hardware-backed or managed signing, secret scanning, protected environments, least-privilege CI, masked logs, and documented rotation after suspected exposure.<br>**Info.1.R.2** — Treat repository helper arguments and shell history as non-secret; never pass raw seeds through them. | Required |
| Info.2 | **Info.2.R.1** — Redact credentials and sensitive payloads, restrict telemetry access, encrypt storage/transport, minimize retention, and test logs for secret leakage. | Required |
| Info.3 | **Info.3.R.1** — Accept that on-chain state/events are public; do not encode secrets in scores, pair metadata, or events.<br>**Info.3.R.2** — Model front-running/timing as an input to consumer policy: apply conservative limits or pauses during `Degraded`, `Unsafe`, stale, and missing states rather than relying on secrecy. | Accepted with mitigation |
| DoS.1 | **DoS.1.R.1** — Use diverse providers, timeouts, circuit breakers, cached observations within strict age bounds, health checks, and alerting.<br>**DoS.1.R.2** — Consumers must explicitly handle `StaleInput`, `PairNotCovered`, and missing configuration and normally fail closed for safety-sensitive operations; define a separately governed emergency mode. | Contract errors Implemented; operations Required |
| DoS.2 | **DoS.2.R.1** — Monitor/fund the operator account, serialize sequence-number use or use channel accounts, set fee policy, retry idempotently, and alert on publication latency.<br>**DoS.2.R.2** — Maintain a tested alternate RPC path and standby operator rotation procedure. | Required |
| DoS.3 | **DoS.3.R.1** — Explicitly call `extend_ttl` for temporary score entries on write and before expiry, with thresholds tied to maximum staleness and expected update cadence.<br>**DoS.3.R.2** — Monitor remaining entry lifetime and test ledger advancement past TTL. If persistence is desired, document and test the storage-class tradeoff. | Required |
| DoS.4 | **DoS.4.R.1** — Enforce safe configuration bounds, two-person review, preflight simulation, config-change events, and alerts.<br>**DoS.4.R.2** — Document rollback/recovery that does not depend on the failed operator. | Required |
| DoS.5 | **DoS.5.R.1** — Allowlist supported pairs, cap update frequency, prioritize critical pairs, batch/queue off-chain work, apply resource budgets, and load-test worst-case cardinality.<br>**DoS.5.R.2** — Rate-limit public off-chain endpoints, if introduced; contract reads remain bounded and read-only. | Required |
| Elevation.1 | **Elevation.1.R.1** — Maintain negative authorization tests for every privileged function, including after every upgrade; fuzz contract inputs and review all new entry points.<br>**Elevation.1.R.2** — Keep privileged logic small and centralize role checks. | Auth checks/tests Partial; ongoing Required |
| Elevation.2 | **Elevation.2.R.1** — Protect branches/tags, pin actions and dependencies, generate provenance/SBOM, isolate release runners, prohibit untrusted PR code from signing/deploying, and require human approval of the exact artifact hash.<br>**Elevation.2.R.2** — Separate build, upload, and admin-approval identities. | Required |
| Elevation.3 | **Elevation.3.R.1** — Replace a single administrator key with threshold governance; use timelocks for routine changes and a narrowly scoped, transparent emergency path.<br>**Elevation.3.R.2** — Consider separating upgrader and configuration-manager roles in a future contract version. | Required |
| Elevation.4 | **Elevation.4.R.1** — Consumers must treat Shield as advisory and combine it with their own authorization, oracle validation, transaction limits, and invariant checks.<br>**Elevation.4.R.2** — Define explicit behavior for all statuses and errors; `Healthy` must never bypass user authorization or protocol safety checks. | Required (consumer) |

## Validation and maintenance

Before production deployment:

- Convert every **Required** treatment into a tracked control with an owner, test/evidence link, and due date.
- Run unit/property tests for score bounds, status thresholds (`32/33` and `65/66`), pair ordering, authorization, stale-boundary behavior, timestamp arithmetic, and TTL expiry.
- Exercise operator/admin compromise, malicious-source, stale-data, upstream-outage, fee/sequence-contention, bad-upgrade, and consumer fail-open scenarios on testnet.
- Independently audit the production WASM and the off-chain scoring/signing pipeline.
- Verify each consuming protocol handles `Degraded`, `Unsafe`, `StaleInput`, `PairNotCovered`, and missing configuration exactly as documented.
- Revisit this model whenever roles, storage, score computation, contract interface, network deployment, or consumer policy changes, and after any relevant incident.

## References

- [Stellar threat-modeling guide](https://developers.stellar.org/docs/build/security-docs/threat-modeling/threat-modeling-how-to)
- [SDF STRIDE template](https://developers.stellar.org/docs/build/security-docs/threat-modeling/STRIDE-template)
- [Stellar Oracle Shield repository](https://github.com/sunzu-lab/stellar-oracle-shield)
- Repository implementation reviewed: `contracts/oracle-shield/src/contract.rs`, `contracts/oracle-shield/src/score.rs`, client interface/status modules, tests, and deployment/invocation/upgrade helpers.
