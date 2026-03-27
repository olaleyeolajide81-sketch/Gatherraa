use soroban_sdk::{Address, Env, Symbol, U256, I256};

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub max_operations: u64,
    pub max_error_rate: u32,
    pub cleanup_interval: u64,
    pub optimization_threshold: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            max_operations: 1_000_000,
            max_error_rate: 100, // 1%
            cleanup_interval: 86400, // 24 hours
            optimization_threshold: 10000,
        }
    }
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug)]
pub enum StorageError {
    InitializationError,
    ConfigurationError,
    OptimizationError,
    CleanupError,
    CapacityExceeded,
    AccessDenied,
}

pub enum DataKey {
    Admin,
    Config,
    Metrics,
    Version,
    StorageData(Symbol),
    UserStorage(Address, Symbol),
}
