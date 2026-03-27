#![no_std]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::cast_possible_truncation)]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Symbol, Vec,
};

use gathera_common::{
    validate_address, set_reentrancy_guard, remove_reentrancy_guard,
    require_admin, read_version, write_version
};

mod storage_types;
use storage_types::{DataKey, Role, RoleConfig, AccessControlError};

#[contract]
pub struct AccessControlContract;

/// Access Control Contract provides role-based access control (RBAC) functionality.
///
/// Features include role assignment, permission management, and access validation.
#[contractimpl]
impl AccessControlContract {
    /// Initialize the access control contract
    pub fn initialize(env: Env, admin: Address, config: RoleConfig) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        validate_address(&env, &admin);
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::Version, &1u32);

        // Grant admin role to initializer
        Self::grant_role(env.clone(), admin.clone(), Symbol::new(&env, "ADMIN"));
        
        // Emit event
        env.events().publish(
            (Symbol::new(&env, "access_control_initialized"), admin),
            config,
        );
    }

    /// Grant a role to an address
    pub fn grant_role(env: Env, admin: Address, user: Address, role: Symbol) {
        require_admin(&env, &admin);
        validate_address(&env, &user);

        let role_key = DataKey::Role(role.clone(), user.clone());
        let role_data = Role {
            role: role.clone(),
            granted_at: env.ledger().timestamp(),
            granted_by: admin,
        };

        env.storage().persistent().set(&role_key, &role_data);

        env.events().publish(
            (Symbol::new(&env, "role_granted"), admin),
            (user, role),
        );
    }

    /// Revoke a role from an address
    pub fn revoke_role(env: Env, admin: Address, user: Address, role: Symbol) {
        require_admin(&env, &admin);
        validate_address(&env, &user);

        let role_key = DataKey::Role(role.clone(), user.clone());
        if !env.storage().persistent().has(&role_key) {
            panic!("Role not assigned");
        }

        env.storage().persistent().remove(&role_key);

        env.events().publish(
            (Symbol::new(&env, "role_revoked"), admin),
            (user, role),
        );
    }

    /// Check if an address has a specific role
    pub fn has_role(env: Env, user: Address, role: Symbol) -> bool {
        validate_address(&env, &user);
        let role_key = DataKey::Role(role, user);
        env.storage().persistent().has(&role_key)
    }

    /// Get all roles for an address
    pub fn get_user_roles(env: Env, user: Address) -> Vec<Symbol> {
        validate_address(&env, &user);
        // This is a simplified implementation
        // In practice, you'd need to iterate through stored roles
        Vec::new(&env)
    }

    /// Update role configuration
    pub fn update_config(env: Env, admin: Address, config: RoleConfig) {
        require_admin(&env, &admin);
        env.storage().instance().set(&DataKey::Config, &config);

        env.events().publish(
            (Symbol::new(&env, "config_updated"), admin),
            config,
        );
    }

    /// Get role configuration
    pub fn get_config(env: Env) -> RoleConfig {
        env.storage().instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Access control not initialized"))
    }

    /// Check if access is allowed for a specific permission
    pub fn check_access(env: Env, user: Address, permission: Symbol) -> bool {
        validate_address(&env, &user);
        
        // Check if user has admin role (full access)
        if Self::has_role(env.clone(), user.clone(), Symbol::new(&env, "ADMIN")) {
            return true;
        }

        // Check specific role permissions based on configuration
        let config = Self::get_config(env.clone());
        // Implement permission checking logic here
        // This is a simplified version
        
        false
    }

    /// Require specific role (panics if not present)
    pub fn require_role(env: Env, user: Address, role: Symbol) {
        if !Self::has_role(env.clone(), user.clone(), role) {
            panic!("Insufficient permissions");
        }
    }

    /// Require specific permission (panics if not present)
    pub fn require_permission(env: Env, user: Address, permission: Symbol) {
        if !Self::check_access(env.clone(), user.clone(), permission) {
            panic!("Access denied");
        }
    }
}
