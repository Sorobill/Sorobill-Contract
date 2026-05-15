#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

use crate::{errors::SubstrataError, types::BillingInterval, SubstrataContract, SubstrataContractClient};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn setup() -> (Env, SubstrataContractClient<'static>, Address, Address, Address) {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register_contract(None, SubstrataContract);
    let client = SubstrataContractClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let merchant = Address::generate(&e);
    let subscriber = Address::generate(&e);

    client.initialize(&admin);

    (e, client, admin, merchant, subscriber)
}

/// Deploy a native-style test token, mint `amount` to `to`, and approve the
/// contract to spend on behalf of `to`.
fn setup_token(
    e: &Env,
    contract_id: &Address,
    to: &Address,
    amount: i128,
) -> Address {
    let token_admin = Address::generate(e);
    let token_id = e.register_stellar_asset_contract_v2(token_admin.clone());
    let token_addr = token_id.address();

    let asset_client = StellarAssetClient::new(e, &token_addr);
    asset_client.mint(to, &amount);

    // Approve the Substrata contract to pull funds
    let token_client = TokenClient::new(e, &token_addr);
    token_client.approve(to, contract_id, &amount, &(e.ledger().sequence() + 10_000));

    token_addr
}

// ── Plan Tests ────────────────────────────────────────────────────────────────

#[test]
fn test_create_plan() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    assert_eq!(plan_id, 0);
    let plan = client.get_plan(&plan_id).unwrap();
    assert_eq!(plan.price, 100);
    assert!(plan.active);
}

#[test]
fn test_create_plan_invalid_price() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let err = client
        .try_create_plan(&merchant, &0, &BillingInterval::Monthly, &token)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::InvalidPrice.into());
}

#[test]
fn test_update_plan_price() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.update_plan_price(&merchant, &plan_id, &200).unwrap();
    assert_eq!(client.get_plan(&plan_id).unwrap().price, 200);
}

#[test]
fn test_deactivate_plan_blocks_subscribe() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.deactivate_plan(&merchant, &plan_id).unwrap();

    let err = client
        .try_subscribe(&subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::PlanInactive.into());
}

// ── Subscription Tests ────────────────────────────────────────────────────────

#[test]
fn test_subscribe_and_get() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    let sub = client.get_subscription(&subscriber, &plan_id).unwrap();
    assert!(sub.active);
    assert!(!sub.paused);
    assert_eq!(sub.failed_attempts, 0);
}

#[test]
fn test_double_subscribe_fails() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    let err = client
        .try_subscribe(&subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::AlreadySubscribed.into());
}

#[test]
fn test_cancel_subscription() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();
    client.cancel(&subscriber, &plan_id).unwrap();

    let sub = client.get_subscription(&subscriber, &plan_id).unwrap();
    assert!(!sub.active);
}

#[test]
fn test_pause_and_resume() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();
    client.pause(&subscriber, &plan_id).unwrap();

    assert!(client.get_subscription(&subscriber, &plan_id).unwrap().paused);

    client.resume(&subscriber, &plan_id).unwrap();
    assert!(!client.get_subscription(&subscriber, &plan_id).unwrap().paused);
}

// ── Payment Tests ─────────────────────────────────────────────────────────────

#[test]
fn test_billing_not_due() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    // Billing is not due yet (next_billing = now + interval)
    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::BillingNotDue.into());
}

#[test]
fn test_successful_billing() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    // Advance ledger past billing interval
    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    client.execute_billing(&admin, &subscriber, &plan_id).unwrap();

    let token_client = TokenClient::new(&e, &token);
    assert_eq!(token_client.balance(&merchant), 100);
    assert_eq!(token_client.balance(&subscriber), 900);
}

#[test]
fn test_insufficient_balance_increments_failed_attempts() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    // Give subscriber only 50, plan costs 100
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::InsufficientBalance.into());
    assert_eq!(
        client.get_subscription(&subscriber, &plan_id).unwrap().failed_attempts,
        1
    );
}

#[test]
fn test_three_failures_auto_cancel() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Daily, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    for _ in 0..3 {
        e.ledger().with_mut(|l| {
            l.timestamp += BillingInterval::Daily.as_secs() + 1;
        });
        let _ = client.try_execute_billing(&admin, &subscriber, &plan_id);
    }

    let sub = client.get_subscription(&subscriber, &plan_id).unwrap();
    assert!(!sub.active, "subscription should be auto-cancelled after 3 failures");
}

#[test]
fn test_billing_blocked_while_paused() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();
    client.pause(&subscriber, &plan_id).unwrap();

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::SubscriptionPaused.into());
}

#[test]
fn test_unauthorized_billing_rejected() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);
    let rando = Address::generate(&e);

    let plan_id = client
        .create_plan(&merchant, &100, &BillingInterval::Monthly, &token)
        .unwrap();

    client.subscribe(&subscriber, &plan_id).unwrap();

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&rando, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::Unauthorized.into());
}
