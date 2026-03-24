#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

/// Seconds of lease time granted per unit of funds added (1 day per unit).
pub const SECS_PER_UNIT: u64 = 86_400;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    pub landlord: Address,
    pub tenant: Address,
    pub amount: i128,
    pub active: bool,
    pub expiry_time: u64,
}

#[contract]
pub struct LeaseContract;

#[contractimpl]
impl LeaseContract {
    /// Initializes a lease between a landlord and a tenant.
    /// `lease_id` uniquely identifies the lease in storage.
    /// `duration` sets the initial lease duration in seconds.
    pub fn create_lease(
        env: Env,
        lease_id: Symbol,
        landlord: Address,
        tenant: Address,
        amount: i128,
        duration: u64,
    ) -> Symbol {
        let expiry_time = env.ledger().timestamp().saturating_add(duration);
        let lease = Lease {
            landlord,
            tenant,
            amount,
            active: true,
            expiry_time,
        };
        env.storage().instance().set(&lease_id, &lease);
        symbol_short!("created")
    }

    /// Returns the lease details for the given `lease_id`.
    pub fn get_lease(env: Env, lease_id: Symbol) -> Lease {
        env.storage()
            .instance()
            .get(&lease_id)
            .expect("Lease not found")
    }

    /// Adds funds to an existing lease, extending `expiry_time` proportionally.
    /// Each unit of `amount` extends the lease by `SECS_PER_UNIT` seconds.
    /// Requires authorization from the tenant.
    pub fn add_funds(env: Env, lease_id: Symbol, amount: i128) -> Symbol {
        assert!(amount > 0, "amount must be positive");

        let mut lease: Lease = env
            .storage()
            .instance()
            .get(&lease_id)
            .expect("Lease not found");

        lease.tenant.require_auth();

        let extra_secs = (amount as u64).saturating_mul(SECS_PER_UNIT);
        lease.amount = lease.amount.saturating_add(amount);
        lease.expiry_time = lease.expiry_time.saturating_add(extra_secs);

        env.storage().instance().set(&lease_id, &lease);

        symbol_short!("extended")
    }
}

mod test;
