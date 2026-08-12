# Contributing to Stellar Oracle Shield

Thank you for considering a contribution to Stellar Oracle Shield.

This project contains Stellar/Soroban smart contracts written in Rust, together with supporting tools and SDKs written in TypeScript. Contributions may include code, tests, documentation, security improvements, developer tooling, and issue reports.

## Code of Conduct

Be respectful, constructive, and welcoming.

Harassment, personal attacks, discrimination, and intentionally disruptive behaviour are not accepted. Maintainers may moderate or remove contributions and discussions that do not follow these expectations.

## Before You Start

For small fixes, documentation improvements, or additional tests, you may open a pull request directly.

For larger changes, please open an issue before starting implementation. This is particularly important for changes involving:

* public contract interfaces;
* storage layout or storage lifetime;
* authorization rules;
* error codes;
* score semantics;
* contract upgrade or migration behaviour;
* generated bindings or SDK APIs;
* deployment and release processes.

Early discussion helps prevent incompatible designs and duplicated work.

## Reporting Bugs

Before opening a bug report, check the existing issues to avoid duplicates.

A useful bug report should include:

* a clear description of the problem;
* the expected and actual behaviour;
* steps or a minimal example that reproduces it;
* the Rust, Stellar CLI, Node.js, and package-manager versions involved;
* the network or environment used, when relevant;
* logs or error messages with secrets and private data removed.

## Reporting Security Issues

Do not publicly disclose suspected vulnerabilities in a GitHub issue.

Use GitHub's private security-reporting feature when it is available. Otherwise, contact the maintainers privately through a channel published by the project.

Please include:

* the affected component and version;
* the vulnerability's potential impact;
* reproduction steps or a proof of concept;
* any suggested mitigation.

Do not access accounts, funds, contracts, or systems that you do not own or have explicit permission to test.

## Repository Structure

The repository is organized as a multi-language project.

```text
contracts/               Rust Stellar/Soroban contracts
  oracle-shield/         Oracle Shield contract
packages/ or sdk/        TypeScript packages and generated SDKs
scripts/                 Development, deployment, and release tooling
.github/workflows/       Continuous-integration and release workflows
```

The exact TypeScript directory names may evolve. Follow the existing repository structure when adding new packages.

## Development Prerequisites

Install the following tools:

* Git;
* a recent stable Rust toolchain;
* the WebAssembly target required by the Stellar toolchain;
* the Stellar CLI;
* Node.js, for TypeScript packages;
* the package manager selected by the repository.

Install Rust and the required target:

```bash
rustup update stable
rustup target add wasm32v1-none
```

Confirm that the Stellar CLI is available:

```bash
stellar --version
```

For TypeScript work, use the Node.js and package-manager versions declared by the repository, such as in `.nvmrc`, `.node-version`, `package.json`, or the lockfile.

Avoid introducing a second TypeScript package manager or lockfile.

## Setting Up the Repository

Fork the repository, then clone your fork:

```bash
git clone https://github.com/<your-account>/stellar-oracle-shield.git
cd stellar-oracle-shield
```

Add the upstream repository:

```bash
git remote add upstream https://github.com/sunzu-lab/stellar-oracle-shield.git
```

Create a branch from the latest `main`:

```bash
git fetch upstream
git switch main
git rebase upstream/main
git switch -c <type>/<short-description>
```

Examples:

```text
feat/add-score-expiration
fix/reject-unauthorized-update
test/add-boundary-cases
docs/explain-contract-storage
chore/update-ci
```

Keep branches focused on one logical change.

## Rust and Contract Contributions

### Formatting

Format the entire Rust workspace:

```bash
cargo fmt --all
```

Verify formatting without changing files:

```bash
cargo fmt --all --check
```

### Linting

Run Clippy across the workspace and all targets:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

New warnings should not be introduced without a documented reason.

### Tests

Run all Rust tests:

```bash
cargo test --workspace
```

Contributions that change contract behaviour should include tests covering:

* the successful path;
* authorization failures;
* invalid and boundary inputs;
* repeated calls and state transitions;
* storage behaviour;
* emitted events, when applicable;
* expected contract errors;
* regressions fixed by the change.

Tests should be deterministic and must not depend on public network availability.

### Building Contracts

Use the Stellar CLI to build contracts:

```bash
stellar contract build
```

For an optimized release artifact:

```bash
stellar contract build --optimize
```

Prefer these commands over invoking a generic Cargo WebAssembly build directly. The Stellar CLI applies contract-specific build and bindings behaviour.

Do not commit generated build artifacts unless the repository explicitly tracks them.

### Contract Design Expectations

Smart-contract changes require particular care because deployed behaviour may be difficult or impossible to reverse.

When changing contract code:

* require authorization explicitly for privileged operations;
* keep storage keys and storage types clearly separated by responsibility;
* document storage lifetime and expiration behaviour;
* avoid changing public interfaces unintentionally;
* use stable, unambiguous error codes;
* consider collisions with errors originating from dependencies;
* validate numeric ranges and boundary conditions;
* avoid unnecessary allocations and ledger storage;
* preserve deterministic behaviour;
* document compatibility or migration implications.

Error codes should use a project-reserved range rather than low generic values that may be confused with errors originating from the Stellar SDK or invoked contracts.

Any change to an existing public contract interface must be clearly identified in the pull request as compatible or breaking.

## TypeScript Contributions

Use the scripts defined in the relevant `package.json`. Typical checks may include:

```bash
npm install
npm run format
npm run lint
npm run typecheck
npm test
npm run build
```

Replace `npm` with the package manager selected by the repository.

TypeScript contributions should:

* compile with strict type checking;
* avoid unnecessary `any` types;
* include tests for observable behaviour;
* keep generated files separate from handwritten source;
* document exported APIs;
* preserve compatibility with supported Node.js versions;
* use contract-generated bindings where appropriate;
* produce reproducible builds.

Do not manually edit generated bindings. Change their source or generation process, then regenerate them.

When a contract interface changes, update and test all affected TypeScript bindings and SDK packages in the same pull request whenever practical.

## Documentation

Update documentation whenever a contribution changes:

* setup instructions;
* commands or configuration;
* public contract methods;
* SDK APIs;
* authorization requirements;
* errors;
* deployment behaviour;
* release procedures.

Code examples should be runnable or explicitly marked as pseudocode.

## Commit Guidelines

Write commits that are focused and understandable.

A recommended format is:

```text
<type>(<scope>): <summary>
```

Examples:

```text
feat(contract): add score expiration
fix(auth): require admin authorization
test(score): cover minimum and maximum values
docs(contributing): document local checks
chore(ci): harden workflow permissions
```

Common types include:

* `feat`;
* `fix`;
* `docs`;
* `test`;
* `refactor`;
* `perf`;
* `build`;
* `ci`;
* `chore`.

Use the imperative mood and keep the first line concise.

Issue or ticket identifiers may be included when applicable, but external contributors are not required to have an internal ticket.

Maintainers may squash commits when merging.

## Pull Requests

Before opening a pull request:

1. Rebase your branch on the latest upstream `main`.
2. Format the code.
3. Run the relevant linters.
4. Run all affected tests.
5. Build the affected contract and TypeScript packages.
6. Update documentation.
7. Review the diff for secrets, debug output, generated noise, and unrelated changes.

A pull request should include:

* the problem being solved;
* the proposed solution;
* the scope of the change;
* how the change was tested;
* contract, storage, security, or compatibility implications;
* related issues;
* follow-up work that is intentionally excluded.

Use draft pull requests for incomplete work when early feedback would be useful.

Keep pull requests reasonably small. Large features should be divided into independently reviewable steps where possible.

## Pull Request Checklist

```markdown
## Summary

<!-- What does this change do, and why? -->

## Testing

<!-- List the commands and scenarios used to test the change. -->

## Contract and security impact

- [ ] No contract behaviour is changed.
- [ ] Authorization implications have been reviewed.
- [ ] Storage and expiration implications have been reviewed.
- [ ] Error-code implications have been reviewed.
- [ ] Public API and compatibility implications have been reviewed.

## Checklist

- [ ] The change is focused and contains no unrelated modifications.
- [ ] Rust code is formatted.
- [ ] Rust lint checks pass.
- [ ] Rust tests pass.
- [ ] A Stellar contract build succeeds.
- [ ] TypeScript formatting, linting, type checking, tests, and builds pass where applicable.
- [ ] New or changed behaviour is tested.
- [ ] Documentation is updated.
- [ ] No secrets or private data are included.
- [ ] Generated files are reproducible.
- [ ] Breaking changes are clearly identified.
```

## Continuous Integration

All required CI checks must pass before a pull request can be merged.

Do not weaken, skip, or bypass a check solely to make a contribution pass. Changes to CI or release workflows should explain why they are needed and what security implications they introduce.

GitHub Actions should follow least-privilege practices:

* declare minimal workflow permissions;
* pin third-party actions according to the project's security policy;
* avoid exposing secrets to untrusted pull-request code;
* validate workflow changes with an appropriate security linter;
* protect release environments with required reviewers;
* keep build and publication responsibilities separated where practical.

## Dependency Changes

Keep dependency changes focused and justified.

When adding or updating a dependency:

* explain why it is needed;
* prefer actively maintained and narrowly scoped packages;
* review its licence and security posture;
* commit the associated lockfile changes;
* avoid unrelated dependency upgrades;
* note any effect on contract size, performance, or compatibility.

Dependencies pulled directly from a Git repository should be pinned to an immutable revision or reviewed tag and should include a reason for not using a registry release.

## Releases

Only maintainers publish releases.

Contributors must not include registry credentials, signing keys, deployment secrets, or personal tokens in code or workflow files.

Release-related changes should account for all published artifacts, including:

* optimized contract WebAssembly;
* Rust crates, when applicable;
* TypeScript or npm packages;
* generated bindings;
* checksums and provenance metadata;
* Stellar ecosystem discovery or publication requirements.

Contract releases and SDK releases must remain compatible and should use clearly documented versioning.

## Backward Compatibility and Versioning

Use semantic-versioning principles for public crates and TypeScript packages.

Examples of potentially breaking changes include:

* renaming or removing contract methods;
* changing method parameters or return values;
* changing serialized public types;
* changing error meanings;
* changing authorization requirements;
* changing SDK exports;
* changing storage in a way that prevents existing state from being read.

Breaking changes require explicit maintainer discussion and a documented migration or upgrade strategy.

## Review Process

Maintainers may request changes related to correctness, testing, security, maintainability, compatibility, or scope.

Address review comments with either:

* a code or documentation change; or
* a concise explanation of why no change is appropriate.

Do not resolve another person's review thread unless the concern has been addressed or the reviewer has indicated agreement.

Approval does not guarantee immediate merging. Maintainers may delay a contribution because of release timing, compatibility concerns, or related architectural work.

## Licence

This project is licensed under the Apache License 2.0.

By intentionally submitting a contribution for inclusion in this project, you agree that your contribution is provided under the same licence, unless a separate written agreement applies.

Only submit work that you have the right to contribute. Clearly identify code or assets derived from third-party sources and ensure their licences are compatible with this project.

## Getting Help

For usage questions, design proposals, and contribution discussions, open a GitHub issue or use the repository's configured discussion channel.

When asking for help, include enough context for another contributor to reproduce or understand the situation, but never include secrets, private keys, seed phrases, credentials, or confidential data.
