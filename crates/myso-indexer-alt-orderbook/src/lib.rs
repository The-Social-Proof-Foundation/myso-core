use url::Url;

pub mod handlers;
pub(crate) mod models;
pub mod traits;

pub const NOT_MAINNET_PACKAGE: &str = "<not on mainnet>";

pub const MAINNET_REMOTE_STORE_URL: &str = "https://checkpoints.mainnet.mysocial.network";
pub const TESTNET_REMOTE_STORE_URL: &str =
    "https://storage.googleapis.com/mysocial-testnet-checkpoints";

// Package addresses for different environments
const MAINNET_PACKAGES: &[&str] = &[
    "0x00000000000000000000000000000000000000000000000000000000000050c1", // Latest
];

// Framework-level address: orderbook is published at the same address as myso-social (0x50c1)
const TESTNET_PACKAGES: &[&str] = &[
    "0x00000000000000000000000000000000000000000000000000000000000050c1", // Latest
];

// Mainnet margin package addresses
const MAINNET_MARGIN_PACKAGES: &[&str] = &[
    "0x00000000000000000000000000000000000000000000000000000000000050c1", // Latest
];
const TESTNET_MARGIN_PACKAGES: &[&str] = &[
    "0x00000000000000000000000000000000000000000000000000000000000050c1", // Latest
];

// Module definitions
/// Core Orderbook modules that handle trading, orders, and pool management
pub const CORE_MODULES: &[&str] = &[
    "balance_manager",
    "order",
    "order_info",
    "vault",
    "myso_price",
    "state",
    "governance",
    "pool",
];

/// Margin trading modules that handle lending and borrowing
pub const MARGIN_MODULES: &[&str] = &[
    "margin_manager",
    "margin_pool",
    "margin_registry",
    "protocol_fees",
    "tpsl",
];

/// MYSO system modules
pub const MYSO_MODULES: &[&str] = &["myso"];

/// Enum representing different module types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    Core,
    Margin,
    MySo,
    Unknown,
}

/// Check if a module is a core Orderbook module
pub fn is_core_module(module: &str) -> bool {
    CORE_MODULES.contains(&module)
}

/// Check if a module is a margin trading module
pub fn is_margin_module(module: &str) -> bool {
    MARGIN_MODULES.contains(&module)
}

/// Check if a module is a MYSO system module
pub fn is_myso_module(module: &str) -> bool {
    MYSO_MODULES.contains(&module)
}

/// Get the module type (core, margin, myso, or unknown)
pub fn get_module_type(module: &str) -> ModuleType {
    if is_core_module(module) {
        ModuleType::Core
    } else if is_margin_module(module) {
        ModuleType::Margin
    } else if is_myso_module(module) {
        ModuleType::MySo
    } else {
        ModuleType::Unknown
    }
}

/// Get all known module names
pub fn get_all_known_modules() -> Vec<&'static str> {
    let mut modules = Vec::new();
    modules.extend_from_slice(CORE_MODULES);
    modules.extend_from_slice(MARGIN_MODULES);
    modules.extend_from_slice(MYSO_MODULES);
    modules
}

/// Get all core module names
pub fn get_core_modules() -> &'static [&'static str] {
    CORE_MODULES
}

/// Get all margin module names
pub fn get_margin_modules() -> &'static [&'static str] {
    MARGIN_MODULES
}

/// Get all MYSO module names
pub fn get_myso_modules() -> &'static [&'static str] {
    MYSO_MODULES
}

/// Check if a margin package address is valid
pub fn is_valid_margin_package(package: &str) -> bool {
    package != NOT_MAINNET_PACKAGE
}

/// Check if any margin package addresses are valid for the given environment
pub fn is_valid_margin_packages(packages: &[&str]) -> bool {
    packages.iter().any(|&pkg| is_valid_margin_package(pkg))
}

/// Check if margin trading is supported in the given environment
pub fn is_margin_supported(env: OrderbookEnv) -> bool {
    match env {
        OrderbookEnv::Mainnet => is_valid_margin_packages(MAINNET_MARGIN_PACKAGES),
        OrderbookEnv::Testnet => is_valid_margin_packages(TESTNET_MARGIN_PACKAGES),
    }
}

/// Get the margin package addresses for the given environment
pub fn get_margin_package_addresses(env: OrderbookEnv) -> &'static [&'static str] {
    match env {
        OrderbookEnv::Mainnet => MAINNET_MARGIN_PACKAGES,
        OrderbookEnv::Testnet => TESTNET_MARGIN_PACKAGES,
    }
}

/// Get the first valid margin package address for the given environment with validation
pub fn get_margin_package_address(env: OrderbookEnv) -> Result<&'static str, String> {
    let packages = get_margin_package_addresses(env);

    // Find the first valid package
    for &package in packages {
        if is_valid_margin_package(package) {
            return Ok(package);
        }
    }

    Err(format!(
        "Margin trading is not supported on {:?}. \
        The margin package has not been deployed on this network.",
        env
    ))
}

/// Get all core package addresses for the given environment
pub fn get_core_package_addresses(env: OrderbookEnv) -> &'static [&'static str] {
    match env {
        OrderbookEnv::Mainnet => MAINNET_PACKAGES,
        OrderbookEnv::Testnet => TESTNET_PACKAGES,
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OrderbookEnv {
    Mainnet,
    Testnet,
}

impl OrderbookEnv {
    pub fn remote_store_url(&self) -> Url {
        let url = match self {
            OrderbookEnv::Mainnet => MAINNET_REMOTE_STORE_URL,
            OrderbookEnv::Testnet => TESTNET_REMOTE_STORE_URL,
        };
        Url::parse(url).unwrap()
    }

    /// Get all package addresses (Orderbook + Margin) for this environment
    fn get_all_package_strings(&self) -> Vec<&str> {
        let (packages, margin_packages) = match self {
            OrderbookEnv::Mainnet => (MAINNET_PACKAGES, MAINNET_MARGIN_PACKAGES),
            OrderbookEnv::Testnet => (TESTNET_PACKAGES, TESTNET_MARGIN_PACKAGES),
        };

        let mut all_packages = packages.to_vec();

        // Add margin packages if they're not invalid
        for &margin_package in margin_packages {
            if margin_package != NOT_MAINNET_PACKAGE {
                all_packages.push(margin_package);
            }
        }

        all_packages
    }

    pub fn package_ids(&self) -> Vec<myso_types::base_types::ObjectID> {
        use myso_types::base_types::ObjectID;
        use std::str::FromStr;

        self.get_all_package_strings()
            .iter()
            .map(|pkg| ObjectID::from_str(pkg).unwrap())
            .collect()
    }

    pub fn package_addresses(&self) -> Vec<move_core_types::account_address::AccountAddress> {
        use move_core_types::account_address::AccountAddress;
        use std::str::FromStr;

        self.get_all_package_strings()
            .iter()
            .map(|pkg| AccountAddress::from_str(pkg).unwrap())
            .collect()
    }
}
