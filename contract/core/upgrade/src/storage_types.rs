use soroban_sdk::{Address, Bytes, Env, Symbol};

#[derive(Clone, Debug)]
pub struct UpgradeConfig {
    pub min_notice_period: u64,
    pub max_upgrade_time: u64,
    pub require_multisig: bool,
    pub allowed_upgraders: Vec<Address>,
}

impl Default for UpgradeConfig {
    fn default() -> Self {
        Self {
            min_notice_period: 86400, // 24 hours
            max_upgrade_time: 3600,   // 1 hour
            require_multisig: true,
            allowed_upgraders: Vec::new(&Env::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UpgradeState {
    Idle,
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug)]
pub struct UpgradeInfo {
    pub new_version: u32,
    pub new_contract_hash: Bytes,
    pub scheduled_time: u64,
    pub status: UpgradeStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UpgradeStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Clone, Debug)]
pub enum UpgradeError {
    NotInitialized,
    AlreadyScheduled,
    InsufficientNotice,
    Unauthorized,
    InvalidHash,
    TimeWindowExpired,
}

pub enum DataKey {
    Admin,
    Config,
    CurrentVersion,
    State,
    UpgradeInfo,
}
