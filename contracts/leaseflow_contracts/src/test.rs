#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn test_lease() {
    let env = Env::default();
    let contract_id = env.register(LeaseContract, ());
    let client = LeaseContractClient::new(&env, &contract_id);

    let landlord = Address::generate(&env);
    let tenant = Address::generate(&env);
    let lease_id = symbol_short!("lease");
    let amount = 1000i128;
    let duration = 86_400u64; // 1 day

    client.create_lease(&lease_id, &landlord, &tenant, &amount, &duration);
    let lease = client.get_lease(&lease_id);

    assert_eq!(lease.landlord, landlord);
    assert_eq!(lease.tenant, tenant);
    assert_eq!(lease.amount, amount);
    assert!(lease.active);
    assert_eq!(lease.expiry_time, duration); // ledger timestamp starts at 0 in tests
}

#[test]
fn test_add_funds() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(LeaseContract, ());
    let client = LeaseContractClient::new(&env, &contract_id);

    let landlord = Address::generate(&env);
    let tenant = Address::generate(&env);
    let lease_id = symbol_short!("lease1");
    let initial_amount = 1000i128;
    let duration = 86_400u64; // 1 day

    client.create_lease(&lease_id, &landlord, &tenant, &initial_amount, &duration);

    let before = client.get_lease(&lease_id);
    let added_amount = 500i128;

    client.add_funds(&lease_id, &added_amount);

    let after = client.get_lease(&lease_id);

    assert_eq!(after.amount, initial_amount + added_amount);
    assert_eq!(
        after.expiry_time,
        before.expiry_time + (added_amount as u64 * SECS_PER_UNIT)
    );
    assert_eq!(after.landlord, landlord);
    assert_eq!(after.tenant, tenant);
    assert!(after.active);
}
