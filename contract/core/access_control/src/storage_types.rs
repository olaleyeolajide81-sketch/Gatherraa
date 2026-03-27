use soroban_sdk::{Address, Symbol, Env};

#[derive(Clone, Debug)]
pub struct RoleConfig {
    pub max_roles_per_user: u32,
    pub require_admin_approval: bool,
    pub role_expiry_enabled: bool,
    pub default_role_duration: u64,
}

impl Default for RoleConfig {
    fn default() -> Self {
        Self {
            max_roles_per_user: 5,
            require_admin_approval: true,
            role_expiry_enabled: false,
            default_role_duration: 86400 * 30, // 30 days
        }
    }
}

#[derive(Clone, Debug)]
pub struct Role {
    pub role: Symbol,
    pub granted_at: u64,
    pub granted_by: Address,
}

#[derive(Clone, Debug)]
pub enum AccessControlError {
    NotInitialized,
    AlreadyInitialized,
    InvalidAddress,
    RoleNotFound,
    InsufficientPermissions,
    MaxRolesExceeded,
    RoleExpired,
    Unauthorized,
}

pub enum DataKey {
    Admin,
    Config,
    Version,
    Role(Symbol, Address),
    UserRole(Address),
    Permission(Symbol),
}
