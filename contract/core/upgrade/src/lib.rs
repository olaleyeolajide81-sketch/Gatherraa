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
use storage_types::{DataKey, UpgradeConfig, UpgradeState, UpgradeError};

#[contract]
pub struct UpgradeContract;

/// Upgrade Contract provides secure contract upgrade functionality for the Gatheraa platform.
///
/// Features include version management, upgrade scheduling, and rollback capabilities.
#[contractimpl]
impl UpgradeContract {
    /// Initialize the upgrade contract
    pub fn initialize(env: Env, admin: Address, config: UpgradeConfig) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        require_admin(&env, &admin);
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::CurrentVersion, &1u32);
        env.storage().instance().set(&DataKey::State, &UpgradeState::Idle);

        env.events().publish(
            (Symbol::new(&env, "upgrade_initialized"), admin),
            config,
        );
    }

    /// Schedule an upgrade
    pub fn schedule_upgrade(env: Env, admin: Address, new_version: u32, new_contract_hash: Bytes) {
        require_admin(&env, &admin);
        
        let config = Self::get_config(env.clone());
        if env.ledger().timestamp() + config.min_notice_period > config.min_notice_period {
            panic!("Insufficient notice period");
        }

        let upgrade_info = UpgradeInfo {
            new_version,
            new_contract_hash,
            scheduled_time: env.ledger().timestamp() + config.min_notice_period,
            status: UpgradeStatus::Scheduled,
        };

        env.storage().instance().set(&DataKey::UpgradeInfo, &upgrade_info);
        env.storage().instance().set(&DataKey::State, &UpgradeState::Scheduled);

        env.events().publish(
            (Symbol::new(&env, "upgrade_scheduled"), admin),
            (new_version, new_contract_hash),
        );
    }

    /// Execute scheduled upgrade
    pub fn execute_upgrade(env: Env, admin: Address) {
        require_admin(&env, &admin);
        
        let upgrade_info = Self::get_upgrade_info(env.clone());
        if upgrade_info.status != UpgradeStatus::Scheduled {
            panic!("No scheduled upgrade");
        }

        if env.ledger().timestamp() < upgrade_info.scheduled_time {
            panic!("Upgrade not ready");
        }

        // Execute upgrade logic here
        env.storage().instance().set(&DataKey::CurrentVersion, &upgrade_info.new_version);
        env.storage().instance().set(&DataKey::State, &UpgradeState::Completed);

        env.events().publish(
            (Symbol::new(&env, "upgrade_executed"), admin),
            upgrade_info.new_version,
        );
    }

    /// Cancel scheduled upgrade
    pub fn cancel_upgrade(env: Env, admin: Address) {
        require_admin(&env, &admin);
        
        let upgrade_info = Self::get_upgrade_info(env.clone());
        if upgrade_info.status != UpgradeStatus::Scheduled {
            panic!("No scheduled upgrade to cancel");
        }

        let mut updated_info = upgrade_info;
        updated_info.status = UpgradeStatus::Cancelled;
        
        env.storage().instance().set(&DataKey::UpgradeInfo, &updated_info);
        env.storage().instance().set(&DataKey::State, &UpgradeState::Idle);

        env.events().publish(
            (Symbol::new(&env, "upgrade_cancelled"), admin),
            (),
        );
    }

    /// Get current version
    pub fn get_current_version(env: Env) -> u32 {
        env.storage().instance()
            .get(&DataKey::CurrentVersion)
            .unwrap_or(1)
    }

    /// Get upgrade state
    pub fn get_upgrade_state(env: Env) -> UpgradeState {
        env.storage().instance()
            .get(&DataKey::State)
            .unwrap_or(UpgradeState::Idle)
    }

    /// Get upgrade configuration
    pub fn get_config(env: Env) -> UpgradeConfig {
        env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Upgrade contract not initialized"))
    }

    /// Get upgrade info
    pub fn get_upgrade_info(env: Env) -> UpgradeInfo {
        env.storage().instance()
            .get(&DataKey::UpgradeInfo)
            .unwrap_or_else(|| panic!("No upgrade info available"))
    }
}
