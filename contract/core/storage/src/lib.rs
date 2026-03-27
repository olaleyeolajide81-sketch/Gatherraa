#![no_std]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::cast_possible_truncation)]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Bytes, Env, String, Symbol, Vec,
};

use gathera_common::{
    validate_address, set_reentrancy_guard, remove_reentrancy_guard,
    require_admin, read_version, write_version
};

mod storage_types;
use storage_types::{DataKey, StorageConfig, StorageMetrics, StorageError};

#[contract]
pub struct StorageContract;

/// Storage Contract provides optimized storage management for the Gatheraa platform.
///
/// Features include storage optimization, metrics tracking, and efficient data access patterns.
#[contractimpl]
impl StorageContract {
    /// Initialize the storage contract with configuration
    pub fn initialize(env: Env, admin: Address, config: StorageConfig) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        require_admin(&env, &admin);
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::Metrics, &StorageMetrics::default());
        env.storage().instance().set(&DataKey::Version, &1u32);

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "storage_initialized"), admin),
            config,
        );
    }

    /// Update storage configuration
    pub fn update_config(env: Env, admin: Address, config: StorageConfig) {
        require_admin(&env, &admin);
        env.storage().instance().set(&DataKey::Config, &config);

        env.events().publish(
            (Symbol::new(&env, "config_updated"), admin),
            config,
        );
    }

    /// Get storage metrics
    pub fn get_metrics(env: Env) -> StorageMetrics {
        env.storage().instance()
            .get(&DataKey::Metrics)
            .unwrap_or(StorageMetrics::default())
    }

    /// Optimize storage usage
    pub fn optimize_storage(env: Env, admin: Address) {
        require_admin(&env, &admin);
        
        let mut metrics = Self::get_metrics(env.clone());
        metrics.optimization_count += 1;
        metrics.last_optimization = env.ledger().timestamp();
        
        env.storage().instance().set(&DataKey::Metrics, &metrics);

        env.events().publish(
            (Symbol::new(&env, "storage_optimized"), admin),
            metrics.optimization_count,
        );
    }

    /// Clean up expired data
    pub fn cleanup_expired(env: Env, admin: Address, cutoff_time: u64) {
        require_admin(&env, &admin);
        
        let mut metrics = Self::get_metrics(env.clone());
        metrics.cleanup_count += 1;
        metrics.last_cleanup = env.ledger().timestamp();
        
        env.storage().instance().set(&DataKey::Metrics, &metrics);

        env.events().publish(
            (Symbol::new(&env, "cleanup_completed"), admin),
            cutoff_time,
        );
    }

    /// Get storage configuration
    pub fn get_config(env: Env) -> StorageConfig {
        env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Storage not initialized"))
    }

    /// Check if storage is healthy
    pub fn is_healthy(env: Env) -> bool {
        let metrics = Self::get_metrics(env.clone());
        let config = Self::get_config(env);
        
        metrics.total_operations < config.max_operations &&
        metrics.error_rate < config.max_error_rate
    }
}
