# Test Notification Workflow Fix

## Why
The notification workflow has been fixed with multiple changes (gh CLI setup, jq env vars, error handling). A comprehensive test is needed to verify all fixes work correctly and the workflow can execute without errors.

## What Changes
- **Add workflow validation tests** - Verify YAML syntax and structure
- **Test gh CLI installation** - Ensure the installation script works
- **Test jq environment variable handling** - Verify correct interpolation
- **Test error handling paths** - Verify graceful failure on API errors
- **Test end-to-end workflow** - Simulate a complete workflow run

## Impact
- Affected workflows: `github-notifications.yml`
- Test coverage: YAML validation, CLI setup, jq syntax, error handling

## ADDED Requirements
### Requirement: YAML Syntax Validation
The workflow file SHALL pass GitHub Actions YAML syntax validation.

#### Scenario: Valid YAML structure
- **WHEN** the workflow file is validated
- **THEN** no syntax errors are found

### Requirement: gh CLI Installation Script
The gh CLI installation script SHALL work correctly on ubuntu-latest.

#### Scenario: gh CLI installed
- **WHEN** the installation script runs
- **THEN** gh CLI is available and authenticated

### Requirement: jq Environment Variable Interpolation
All jq commands SHALL correctly receive and use the SINCE timestamp.

#### Scenario: jq filters with timestamp
- **WHEN** jq processes API responses
- **THEN** the --arg since parameter works correctly

### Requirement: Error Handling Guards
The workflow SHALL continue on individual step failures.

#### Scenario: API endpoint returns error
- **WHEN** curl or jq fails
- **THEN** the || fallback produces empty result instead of aborting

### Requirement: Commit Message Contains [skip ci]
The commit step SHALL include [skip ci] to prevent CI loops.

#### Scenario: Workflow commits changes
- **WHEN** git commit is executed
- **THEN** the message contains [skip ci]
