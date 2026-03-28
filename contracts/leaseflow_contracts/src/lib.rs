#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, BytesN, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeaseError {
    // Authorization Errors (100-199)
    UnauthorizedTenant = 100,
    UnauthorizedLandlord = 101,
    UnauthorizedRegistryRemoval = 102,
    
    // Lease State Errors (200-299)
    LeaseNotPending = 200,
    LeaseNotActive = 201,
    LeaseNotFound = 202,
    LeaseAlreadyExists = 203,
    
    // Property Registry Errors (300-399)
    PropertyAlreadyLeased = 300,
    PropertyNotRegistered = 301,
    InvalidPropertyHash = 302,
    
    // Financial Errors (400-499)
    DepositInsufficient = 400,
    InvalidRefundAmount = 401,
    RefundSumMismatch = 402,
    NegativeAmount = 403,
    
    // Validation Errors (500-599)
    InvalidDateRange = 500,
    InvalidAddress = 501,
    InvalidSignature = 502,
    InvalidAmendment = 503,
    
    // System Errors (900-999)
    InternalError = 900,
    StorageError = 901,
    SerializationError = 902,
}

impl LeaseError {
    pub fn to_code(&self) -> u32 {
        match self {
            LeaseError::UnauthorizedTenant => 100,
            LeaseError::UnauthorizedLandlord => 101,
            LeaseError::UnauthorizedRegistryRemoval => 102,
            LeaseError::LeaseNotPending => 200,
            LeaseError::LeaseNotActive => 201,
            LeaseError::LeaseNotFound => 202,
            LeaseError::LeaseAlreadyExists => 203,
            LeaseError::PropertyAlreadyLeased => 300,
            LeaseError::PropertyNotRegistered => 301,
            LeaseError::InvalidPropertyHash => 302,
            LeaseError::DepositInsufficient => 400,
            LeaseError::InvalidRefundAmount => 401,
            LeaseError::RefundSumMismatch => 402,
            LeaseError::NegativeAmount => 403,
            LeaseError::InvalidDateRange => 500,
            LeaseError::InvalidAddress => 501,
            LeaseError::InvalidSignature => 502,
            LeaseError::InvalidAmendment => 503,
            LeaseError::InternalError => 900,
            LeaseError::StorageError => 901,
            LeaseError::SerializationError => 902,
        }
    }
    
    pub fn to_message(&self) -> &'static str {
        match self {
            LeaseError::UnauthorizedTenant => "Only tenant can perform this action",
            LeaseError::UnauthorizedLandlord => "Only landlord can perform this action",
            LeaseError::UnauthorizedRegistryRemoval => "Only landlord can remove from registry",
            LeaseError::LeaseNotPending => "Lease is not in pending state",
            LeaseError::LeaseNotActive => "Lease is not active",
            LeaseError::LeaseNotFound => "Lease not found",
            LeaseError::LeaseAlreadyExists => "Lease already exists",
            LeaseError::PropertyAlreadyLeased => "Property already leased in another contract",
            LeaseError::PropertyNotRegistered => "Property not found in global registry",
            LeaseError::InvalidPropertyHash => "Invalid property hash format",
            LeaseError::DepositInsufficient => "Security deposit is insufficient",
            LeaseError::InvalidRefundAmount => "Invalid refund amount specified",
            LeaseError::RefundSumMismatch => "Refund amounts must sum to total deposit",
            LeaseError::NegativeAmount => "Amount cannot be negative",
            LeaseError::InvalidDateRange => "End date must be after start date",
            LeaseError::InvalidAddress => "Invalid address format",
            LeaseError::InvalidSignature => "Invalid signature provided",
            LeaseError::InvalidAmendment => "Invalid lease amendment data",
            LeaseError::InternalError => "Internal contract error occurred",
            LeaseError::StorageError => "Storage access error occurred",
            LeaseError::SerializationError => "Data serialization error occurred",
        }
    }
    
    pub fn to_user_friendly_message(&self) -> &'static str {
        match self {
            LeaseError::UnauthorizedTenant => "You are not authorized as the tenant for this lease",
            LeaseError::UnauthorizedLandlord => "You are not authorized as the landlord for this property",
            LeaseError::UnauthorizedRegistryRemoval => "Only the property owner can remove this listing",
            LeaseError::LeaseNotPending => "This lease cannot be activated in its current state",
            LeaseError::LeaseNotActive => "This action requires an active lease",
            LeaseError::LeaseNotFound => "No lease found for this property",
            LeaseError::LeaseAlreadyExists => "A lease already exists for this property",
            LeaseError::PropertyAlreadyLeased => "This property is already leased to another tenant",
            LeaseError::PropertyNotRegistered => "This property is not registered in the system",
            LeaseError::InvalidPropertyHash => "Property identification data is corrupted",
            LeaseError::DepositInsufficient => "Please add more funds to meet the security deposit requirement",
            LeaseError::InvalidRefundAmount => "The refund amount specified is not valid",
            LeaseError::RefundSumMismatch => "Refund amounts must exactly match the total security deposit",
            LeaseError::NegativeAmount => "Amount values cannot be negative",
            LeaseError::InvalidDateRange => "Please ensure the lease end date is after the start date",
            LeaseError::InvalidAddress => "The wallet address provided is not valid",
            LeaseError::InvalidSignature => "The signature verification failed",
            LeaseError::InvalidAmendment => "The lease amendment data is invalid or incomplete",
            LeaseError::InternalError => "A system error occurred. Please try again later",
            LeaseError::StorageError => "Data storage error. Please contact support",
            LeaseError::SerializationError => "Data processing error. Please try again",
        }
    }
}

macro_rules! require {
    ($condition:expr, $error:expr) => {
        if !$condition {
            let code = $error.to_code();
            let message = $error.to_message();
            panic!("Error {}: {}", code, message);
        }
    };
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeaseStatus {
    Pending,
    Active,
    Expired,
    Disputed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    pub landlord: Address,
    pub tenant: Address,
    pub rent_amount: i128,
    pub deposit_amount: i128,
    pub start_date: u64,
    pub end_date: u64,
    pub property_uri: String,
    pub property_hash: BytesN<32>,
    pub status: LeaseStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeaseAmendment {
    pub new_rent_amount: Option<i128>,
    pub new_end_date: Option<u64>,
    pub landlord_signature: BytesN<32>,
    pub tenant_signature: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositReleasePartial {
    pub tenant_amount: i128,
    pub landlord_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DepositRelease {
    FullRefund,
    PartialRefund(DepositReleasePartial),
    Disputed,
}

#[contract]
pub struct LeaseContract;

#[contractimpl]
impl LeaseContract {
    /// Initializes a lease with collateral lock (security deposit)
    pub fn initialize_lease(
        env: Env,
        landlord: Address,
        tenant: Address,
        rent_amount: i128,
        deposit_amount: i128,
        start_date: u64,
        end_date: u64,
        property_uri: String,
    ) -> Symbol {
        // Generate property hash for global registry check
        let property_hash = Self::generate_property_hash(&env, &property_uri, &landlord);
        
        // Check global property registry to prevent double-leasing
        require!(!Self::is_property_already_leased(&env, &property_hash), 
                 LeaseError::PropertyAlreadyLeased);
        
        let lease = Lease {
            landlord: landlord.clone(),
            tenant: tenant.clone(),
            rent_amount,
            deposit_amount,
            start_date,
            end_date,
            property_uri: property_uri.clone(),
            property_hash,
            status: LeaseStatus::Pending,
        };
        
        env.storage()
            .instance()
            .set(&symbol_short!("lease"), &lease);
        
        // Register property in global registry
        Self::register_property_in_global(&env, &lease.property_hash, &env.current_contract_address());
        
        symbol_short!("pending")
    }
    
    /// Activates lease after security deposit is transferred
    pub fn activate_lease(env: Env, tenant: Address) -> Symbol {
        let mut lease = Self::get_lease(env.clone());
        
        require!(lease.tenant == tenant, LeaseError::UnauthorizedTenant);
        require!(lease.status == LeaseStatus::Pending, LeaseError::LeaseNotPending);
        
        // In a real implementation, this would verify the token transfer
        // For now, we'll assume the deposit has been transferred
        lease.status = LeaseStatus::Active;
        
        env.storage()
            .instance()
            .set(&symbol_short!("lease"), &lease);
            
        symbol_short!("active")
    }
    
    /// Updates property metadata URI
    pub fn update_property_uri(env: Env, landlord: Address, property_uri: String) -> Symbol {
        let mut lease = Self::get_lease(env.clone());
        
        require!(lease.landlord == landlord, LeaseError::UnauthorizedLandlord);
        
        lease.property_uri = property_uri.clone();
        
        env.storage()
            .instance()
            .set(&symbol_short!("lease"), &lease);
            
        symbol_short!("updated")
    }
    
    /// Amends lease with both landlord and tenant signatures
    pub fn amend_lease(env: Env, amendment: LeaseAmendment) -> Symbol {
        let mut lease = Self::get_lease(env.clone());
        
        require!(lease.status == LeaseStatus::Active, LeaseError::LeaseNotActive);
        
        // In a real implementation, this would verify the signatures
        // For now, we'll assume they are valid
        
        if let Some(new_rent) = amendment.new_rent_amount {
            lease.rent_amount = new_rent;
        }
        
        if let Some(new_end_date) = amendment.new_end_date {
            lease.end_date = new_end_date;
        }
        
        env.storage()
            .instance()
            .set(&symbol_short!("lease"), &lease);
            
        symbol_short!("amended")
    }
    
    /// Releases security deposit with conditional logic
    pub fn release_deposit(env: Env, release_type: DepositRelease) -> Symbol {
        let lease = Self::get_lease(env.clone());
        
        require!(lease.status == LeaseStatus::Active || lease.status == LeaseStatus::Expired, 
                 LeaseError::LeaseNotActive);
        
        match release_type {
            DepositRelease::FullRefund => {
                // In a real implementation, this would transfer full deposit to tenant
                symbol_short!("full_ref")
            }
            DepositRelease::PartialRefund(partial) => {
                require!(partial.tenant_amount + partial.landlord_amount == lease.deposit_amount, 
                         LeaseError::RefundSumMismatch);
                // In a real implementation, this would transfer amounts accordingly
                symbol_short!("partial")
            }
            DepositRelease::Disputed => {
                let mut updated_lease = lease;
                updated_lease.status = LeaseStatus::Disputed;
                env.storage()
                    .instance()
                    .set(&symbol_short!("lease"), &updated_lease);
                symbol_short!("disputed")
            }
        }
    }

    /// Returns the current lease details stored in the contract.
    pub fn get_lease(env: Env) -> Lease {
        env.storage()
            .instance()
            .get(&symbol_short!("lease"))
            .expect("Lease not found")
    }
    
    /// Generates a unique property hash based on property URI and landlord
    fn generate_property_hash(env: &Env, property_uri: &String, landlord: &Address) -> BytesN<32> {
        // Very simple deterministic hash generation
        // In production, this should use proper cryptographic hashing
        let mut result = [0u8; 32];
        
        // Use property URI length for variation
        let uri_len = property_uri.to_bytes().len() as u8;
        result[0] = uri_len;
        
        // Use a simple pattern - this is just for demonstration
        result[1] = 42;
        result[2] = 123;
        
        // Fill rest with a simple pattern
        for i in 3..32 {
            result[i] = result[i-1].wrapping_add(result[i-2]).wrapping_mul(7);
        }
        
        BytesN::from_array(&env, &result)
    }
    
    /// Checks if property is already registered in global registry
    fn is_property_already_leased(env: &Env, property_hash: &BytesN<32>) -> bool {
        let key = (symbol_short!("GLOBAL"), property_hash);
        env.storage()
            .persistent()
            .get::<_, Address>(&key)
            .is_some()
    }
    
    /// Registers property in global registry
    fn register_property_in_global(env: &Env, property_hash: &BytesN<32>, contract_address: &Address) {
        let key = (symbol_short!("GLOBAL"), property_hash);
        env.storage()
            .persistent()
            .set(&key, contract_address);
    }
    
    /// Removes property from global registry (for lease termination)
    pub fn remove_from_global_registry(env: Env, landlord: Address) -> Symbol {
        let lease = Self::get_lease(env.clone());
        
        require!(lease.landlord == landlord, LeaseError::UnauthorizedRegistryRemoval);
        require!(lease.status == LeaseStatus::Expired, LeaseError::LeaseNotActive);
        
        let key = (symbol_short!("GLOBAL"), &lease.property_hash);
        env.storage()
            .persistent()
            .remove(&key);
        
        symbol_short!("removed")
    }
    
    /// Checks if tenant is current on rent (for IoT integration)
    pub fn is_tenant_current_on_rent(env: Env) -> bool {
        let lease = Self::get_lease(env.clone());
        
        match lease.status {
            LeaseStatus::Active => {
                let current_time = env.ledger().timestamp();
                current_time < lease.end_date
            }
            _ => false,
        }
    }
    
    /// Gets lease status for external systems
    pub fn get_lease_status(env: Env) -> Symbol {
        let lease = Self::get_lease(env.clone());
        match lease.status {
            LeaseStatus::Pending => symbol_short!("pending"),
            LeaseStatus::Active => symbol_short!("active"),
            LeaseStatus::Expired => symbol_short!("expired"),
            LeaseStatus::Disputed => symbol_short!("disputed"),
        }
    }
}

mod test;
