# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial CHANGELOG.md following Keep a Changelog format
- CI check requiring CHANGELOG.md updates for PRs modifying contract code
- **predictify-hybrid**: the cap on outcomes per market is now a runtime parameter instead of a hard-coded `10`. Admins call `set_max_outcomes_per_market` (range 2-20, default 10) and read it back with `get_max_outcomes_per_market`; the cap is enforced by `create_market` and by `update_event_outcomes`, which previously had no upper bound at all (closes #9)

### Changed

### Deprecated

### Removed

### Fixed

### Security

## Project Overview

This monorepo contains Stellar smart contracts for a decentralized prediction market system. Changes are organized by contract and category (Added, Changed, Deprecated, Removed, Fixed, Security).

### Contracts

- **admin** - Administrative contract for system governance
- **allowlist** - Contract for managing access control and allowlists
- **analytics** - Analytics and reporting contract
- **auction** - Auction mechanism contract
- **betting** - Betting functionality contract
- **disputes** - Dispute resolution contract
- **fees** - Fee management contract
- **markets** - Core markets contract
- **migrate** - Contract migration utilities
- **monitor** - Monitoring contract
- **oracles** - Oracle price feeds
- **predictify-hybrid** - Hybrid prediction contract
- **reporting** - Reporting contract
- **resolution** - Resolution contract
- **validators** - Validator contract
- **analytics** - Analytics contract

### Guidelines for Contributors

When adding changes to this CHANGELOG:

1. **Add entries to the [Unreleased] section** before the PR is merged
2. **Use appropriate subsections**: Added, Changed, Deprecated, Removed, Fixed, Security
3. **Be descriptive**: Explain what changed and why, not just "bug fix"
4. **Group by contract**: When describing changes, indicate which contract(s) are affected
5. **Link to issues/PRs**: Reference GitHub issues or PRs when relevant

Example entry:
```
### Added
- **markets**: New oracle price feed validation in `submit_oracle_outcome` (#1234)
- **disputes**: Support for multi-validator dispute resolution

### Fixed
- **fees**: Corrected fee calculation overflow on large amounts (#1235)
- **betting**: Fixed race condition in bet placement under high concurrency
```

### Release Process

When creating a new release:

1. Create a new section with the version and date: `## [X.Y.Z] - YYYY-MM-DD`
2. Move all entries from [Unreleased] to the new section
3. Add a footer link at the bottom of the file
4. Tag the release in git
5. Create a GitHub release using the CHANGELOG entries

### Version Links

[Unreleased]: https://github.com/your-org/hermes/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/your-org/hermes/releases/tag/v0.0.1
