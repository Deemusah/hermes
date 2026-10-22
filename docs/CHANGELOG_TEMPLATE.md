# Changelog Entry Template

Use this template when updating `CHANGELOG.md` for your PR. Add your entries to the appropriate subsection under `[Unreleased]`.

## Examples

### Feature Addition

```markdown
### Added
- **markets**: New price feed validation ensures oracle prices are within acceptable bounds
- **disputes**: Support for multi-signature dispute approval process
```

### Bug Fix

```markdown
### Fixed
- **betting**: Corrected fee calculation overflow when bet amount exceeds u128 threshold (#1234)
- **markets**: Fixed race condition in concurrent bet placement
```

### Breaking Change / Deprecation

```markdown
### Changed
- **fees**: Fee calculation now deducts platform fees before user rewards distribution
- **oracles**: Simplified oracle price submission interface

### Deprecated
- **markets**: `legacy_claim_winnings` entry point (use `claim_winnings` instead)
```

### Security Fix

```markdown
### Security
- **admin**: Fixed authorization bypass in `transfer_ownership` that allowed non-admin transfers
- **disputes**: Prevented double-spending in dispute resolution awards
```

## Guidelines

1. **Start each bullet with a contract name in bold**: `- **contract_name**: Description`
2. **Be specific about what changed**: Instead of "Fixed bug", write "Fixed fee calculation in `place_bet` that overflowed on large amounts"
3. **Reference issues/PRs when available**: Use format `(#1234)` at the end of the line
4. **Group related changes**: If multiple related fixes are in one PR, group them under one subsection
5. **Explain the impact**: Help users understand why this change matters

## When to Add Entries

Add entries whenever you modify:
- Contract code in `contracts/*/src/`
- Core utilities in `crates/*/src/`
- Test infrastructure (when it affects behavior, not all test changes)

Do NOT add entries for:
- Documentation-only changes
- Formatting/style changes
- CI/build configuration changes (unless they affect the build output)

## Release Process

When preparing a release:

1. Review all entries in `[Unreleased]`
2. Create a new section: `## [X.Y.Z] - YYYY-MM-DD`
3. Move all entries from `[Unreleased]` to the new section
4. Add version link at the bottom of the file
5. Commit and tag the release
