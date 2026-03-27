# Storage Core Module

## Overview
The Storage Core module provides optimized storage management capabilities for the Gatheraa platform. It implements efficient data storage patterns, metrics tracking, and maintenance operations.

## Features

### Storage Optimization
- Configurable storage parameters
- Automatic optimization routines
- Performance metrics tracking
- Cleanup operations for expired data

### Metrics and Monitoring
- Operation counting
- Success/failure rate tracking
- Performance optimization metrics
- Health status monitoring

### Configuration Management
- Flexible storage configuration
- Runtime parameter updates
- Threshold-based operations
- Error rate monitoring

## Contracts

### StorageContract
The main storage management contract that provides:
- Storage initialization and configuration
- Performance optimization
- Data cleanup operations
- Metrics collection and reporting

## Key Components

### StorageConfig
```rust
pub struct StorageConfig {
    pub max_operations: u64,
    pub max_error_rate: u32,
    pub cleanup_interval: u64,
    pub optimization_threshold: u64,
}
```

### StorageMetrics
```rust
pub struct StorageMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub optimization_count: u64,
    pub cleanup_count: u64,
    pub last_optimization: u64,
    pub last_cleanup: u64,
    pub error_rate: u32,
}
```

## Usage

### Initialization
```rust
StorageContract::initialize(env, admin, config);
```

### Storage Optimization
```rust
StorageContract::optimize_storage(env, admin);
```

### Cleanup Operations
```rust
StorageContract::cleanup_expired(env, admin, cutoff_time);
```

### Health Monitoring
```rust
let is_healthy = StorageContract::is_healthy(env);
let metrics = StorageContract::get_metrics(env);
```

## Dependencies
- `gathera-common`: Shared utilities and types
- `soroban-sdk`: Stellar Soroban SDK

## Security Considerations
- Admin-only operations for critical functions
- Reentrancy protection on state changes
- Input validation for all parameters
- Rate limiting on optimization operations

## Performance
- Optimized storage patterns for minimal gas usage
- Efficient data structures for quick access
- Batch operations for bulk updates
- Lazy evaluation for complex calculations
