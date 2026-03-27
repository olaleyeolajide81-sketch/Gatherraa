# Upgrade Core Module

## Overview
The Upgrade Core module provides secure contract upgrade functionality for the Gatheraa platform. It implements version management, upgrade scheduling, and rollback capabilities with proper security controls.

## Features

### Version Management
- Semantic versioning support
- Version history tracking
- Upgrade state management
- Rollback capabilities

### Secure Upgrades
- Time-locked upgrades
- Admin authorization required
- Configurable notice periods
- Multi-signature support

### Upgrade Scheduling
- Scheduled upgrade execution
- Cancellation capabilities
- Status tracking
- Event notifications

## Contracts

### UpgradeContract
The main upgrade management contract that provides:
- Contract initialization and configuration
- Upgrade scheduling and execution
- Version management
- Security controls

## Key Components

### UpgradeConfig
```rust
pub struct UpgradeConfig {
    pub min_notice_period: u64,
    pub max_upgrade_time: u64,
    pub require_multisig: bool,
    pub allowed_upgraders: Vec<Address>,
}
```

### UpgradeState
```rust
pub enum UpgradeState {
    Idle,
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}
```

### UpgradeInfo
```rust
pub struct UpgradeInfo {
    pub new_version: u32,
    pub new_contract_hash: Bytes,
    pub scheduled_time: u64,
    pub status: UpgradeStatus,
}
```

## Usage

### Initialization
```rust
UpgradeContract::initialize(env, admin, config);
```

### Scheduling Upgrades
```rust
UpgradeContract::schedule_upgrade(env, admin, new_version, new_contract_hash);
```

### Executing Upgrades
```rust
UpgradeContract::execute_upgrade(env, admin);
```

### Canceling Upgrades
```rust
UpgradeContract::cancel_upgrade(env, admin);
```

### Version Management
```rust
let current_version = UpgradeContract::get_current_version(env);
let upgrade_state = UpgradeContract::get_upgrade_state(env);
```

## Security Features

### Time Locks
- Minimum notice periods required
- Time-windowed execution
- Automatic expiration

### Authorization
- Admin-only operations
- Configurable allowed upgraders
- Multi-signature support

### Validation
- Contract hash validation
- Version compatibility checks
- State validation

## Dependencies
- `gathera-common`: Shared utilities and types
- `soroban-sdk`: Stellar Soroban SDK

## Security Considerations
- Time-locked upgrades prevent rushed changes
- Admin authorization ensures proper oversight
- Multi-signature support for critical operations
- Comprehensive audit trail through events

## Best Practices
- Use minimum notice periods for security
- Test upgrades thoroughly on testnet
- Maintain backup and rollback procedures
- Monitor upgrade events and status
